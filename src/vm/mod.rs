use anyhow::{Result, anyhow};
use deno_ast::swc::ast::{ModuleDecl, TsKeywordType, TsKeywordTypeKind};
use deno_ast::swc::common::Span;
use deno_ast::swc::ecma_visit::{Visit, VisitWith};
use deno_ast::{
    EmitOptions, MediaType, ParseParams, ParsedSource, SourceMapOption, TranspileModuleOptions,
    TranspileOptions,
};
use deno_ast::{ModuleItemRef, SourcePos};
use deno_core::{JsRuntime, OpState, RuntimeOptions, op2};
use deno_lint::linter::{LintConfig, Linter, LinterOptions};
use deno_lint::rules::get_all_rules;

use crate::tasks::Task;
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct VmViolation {
    file: PathBuf,
    line: usize,
    column: usize,
    rule: String,
    message: String,
}

impl VmViolation {
    fn render(&self) -> String {
        format!(
            "error[{}]: {}\n --> {}:{}:{}",
            self.rule,
            self.message,
            self.file.display(),
            self.line,
            self.column
        )
    }
}

#[derive(Debug)]
struct ModuleUnit {
    specifier: deno_ast::ModuleSpecifier,
    path: PathBuf,
    parsed: ParsedSource,
    dependencies: Vec<PathBuf>,
}

/// Script execution context
#[derive(Debug, Clone)]
pub struct ScriptContext {
    /// Current package name
    pub package_name: String,
    /// Current package manifest directory
    pub package_path: PathBuf,
    /// Parent package name (if any)
    pub parent_name: Option<String>,
    /// Parent package manifest directory (if any)
    pub parent_path: Option<PathBuf>,
    /// Root workspace name
    pub root_name: String,
    /// Root workspace manifest directory
    pub root_path: PathBuf,
    /// All packages in the workspace (for `Rift.find()`)
    pub all_packages: HashMap<String, PathBuf>,
}

/// Raw script execution result (before conversion to ScriptResult)
#[derive(Debug, Clone)]
pub struct RawScriptResult {
    pub success: bool,
    pub error: Option<String>,
    /// Dependencies added via Dependencies.add()
    pub dependencies: Vec<DependencyValue>,
    /// Plugins added via Plugins.add()
    pub plugins: Vec<PluginValue>,
    /// Config values set via PackageConfiguration.set()
    pub config: Vec<(String, ConfigValue)>,
    /// Tasks registered via Tasks.register()
    pub tasks: Vec<TaskValue>,
    /// Commands registered via defineCommand()
    pub commands: Vec<String>,
    /// Event subscriptions made via on()
    pub event_subscriptions: Vec<String>,
}

/// A dependency value (name, optional version, and optional attributes)
#[derive(Debug, Clone)]
pub struct DependencyValue {
    pub name: String,
    pub version: Option<String>,
    pub attributes: HashMap<String, serde_json::Value>,
}

/// A plugin value (name and optional version)
#[derive(Debug, Clone)]
pub struct PluginValue {
    pub name: String,
    pub version: Option<String>,
}

/// A configuration value
#[derive(Debug, Clone)]
pub enum ConfigValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

/// A task registration value
#[derive(Debug, Clone)]
pub struct TaskValue {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub is_command: bool,
    pub action: Option<String>,
}

/// Op state to hold collected script results
#[derive(Debug)]
pub struct RiftOpState {
    pub dependencies: Vec<DependencyValue>,
    pub plugins: Vec<PluginValue>,
    pub config: Vec<(String, ConfigValue)>,
    pub tasks: Vec<TaskValue>,
    /// Command names registered by plugins
    pub commands: Vec<String>,
    /// Event subscriptions made by plugins
    pub event_subscriptions: Vec<String>,
}

impl Default for RiftOpState {
    fn default() -> Self {
        Self {
            dependencies: Vec::new(),
            plugins: Vec::new(),
            config: Vec::new(),
            tasks: Vec::new(),
            commands: Vec::new(),
            event_subscriptions: Vec::new(),
        }
    }
}

/// Op to add a dependency
#[op2]
fn op_rift_add_dependency(
    state: &mut OpState,
    #[string] name: String,
    #[string] version: Option<String>,
    #[string] attributes_json: Option<String>,
) {
    let attributes = if let Some(attrs_str) = attributes_json {
        serde_json::from_str::<HashMap<String, serde_json::Value>>(&attrs_str).unwrap_or_default()
    } else {
        HashMap::new()
    };

    let mut op_state = state.try_take::<RiftOpState>().unwrap_or_default();
    op_state.dependencies.push(DependencyValue {
        name,
        version,
        attributes,
    });
    state.put(op_state);
}

/// Op to add a plugin
#[op2]
fn op_rift_add_plugin(
    state: &mut OpState,
    #[string] name: String,
    #[string] version: Option<String>,
) {
    let mut op_state = state.try_take::<RiftOpState>().unwrap_or_default();
    op_state.plugins.push(PluginValue { name, version });
    state.put(op_state);
}

/// Op to set a config value
#[op2(fast)]
fn op_rift_set_config(
    state: &mut OpState,
    #[string] key: String,
    #[string] value_type: String,
    #[string] value_str: String,
) {
    let mut op_state = state.try_take::<RiftOpState>().unwrap_or_default();
    let config_val = match value_type.as_str() {
        "string" => ConfigValue::String(value_str),
        "number" => {
            let n = value_str.parse::<f64>().unwrap_or(0.0);
            ConfigValue::Number(n)
        }
        "boolean" => {
            let b = value_str.parse::<bool>().unwrap_or(false);
            ConfigValue::Boolean(b)
        }
        _ => return, // Skip unsupported types
    };
    op_state.config.push((key, config_val));
    state.put(op_state);
}

/// Op to register a task
#[op2(fast)]
fn op_rift_register_task(
    state: &mut OpState,
    #[string] name: String,
    #[string] description: String,
    #[string] dependencies_str: String,
    #[string] is_command_str: String,
    #[string] has_action_str: String,
) {
    let mut op_state = state.try_take::<RiftOpState>().unwrap_or_default();

    // Parse dependencies as comma-separated string
    let dependencies: Vec<String> = if dependencies_str.is_empty() {
        Vec::new()
    } else {
        dependencies_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };

    let is_command = is_command_str == "true";
    let has_action = has_action_str == "true";

    op_state.tasks.push(TaskValue {
        name: name.clone(),
        description,
        dependencies,
        is_command,
        action: if has_action { Some("has_action".to_string()) } else { None },
    });
    state.put(op_state);
}

/// Op to define a command (plugin system)
/// Stores the command handler function for later execution
#[op2(fast)]
fn op_rift_define_command(state: &mut OpState, #[string] name: String) {
    // Register the command globally
    crate::plugin::register_plugin_command(name.clone());

    let mut op_state = state.try_take::<RiftOpState>().unwrap_or_default();
    op_state.commands.push(name);
    state.put(op_state);
}

/// Op to subscribe to an event (plugin system)
/// Stores the event subscription for later emit calls
#[op2(fast)]
fn op_rift_on(state: &mut OpState, #[string] event_name: String) {
    let mut op_state = state.try_take::<RiftOpState>().unwrap_or_default();
    op_state.event_subscriptions.push(event_name);
    state.put(op_state);
}

/// Op to emit an event (plugin system)
/// Triggers all handlers subscribed to the event
#[op2(fast)]
fn op_rift_emit(
    _state: &mut OpState,
    #[string] event_name: String,
    #[string] context_json: String,
) {
    // Get the global event bus and emit the event
    // Errors are logged but not propagated to JavaScript
    let bus = crate::plugin::event_bus();
    if let Err(e) = bus.emit(&event_name, &context_json) {
        eprintln!("Error emitting event '{}': {}", event_name, e);
    }
}

/// Op to write a file (for plugin file generation)
/// Returns true on success, false on failure
#[op2(fast)]
fn op_rift_write_file(
    _state: &mut OpState,
    #[string] path: String,
    #[string] content: String,
) -> bool {
    use std::fs::File;
    use std::io::Write;

    // Create parent directories if they don't exist
    let result = (|| -> Result<(), std::io::Error> {
        if let Some(parent) = std::path::PathBuf::from(&path).parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Write the file
        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    })();

    match result {
        Ok(()) => true,
        Err(e) => {
            eprintln!("Error writing file '{}': {}", path, e);
            false
        }
    }
}

deno_core::extension!(
    rift_runtime,
    ops = [
        op_rift_add_dependency,
        op_rift_add_plugin,
        op_rift_set_config,
        op_rift_register_task,
        op_rift_define_command,
        op_rift_on,
        op_rift_emit,
        op_rift_write_file,
    ],
);

pub struct Vm {
    linter: Linter,
    /// Shared runtime for maintaining global state across script executions
    runtime: Option<std::sync::Mutex<deno_core::JsRuntime>>,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            linter: build_strict_linter(),
            runtime: None,
        }
    }

    /// Get or create the shared runtime
    fn get_or_create_runtime(
        &mut self,
    ) -> Result<std::sync::MutexGuard<deno_core::JsRuntime>, anyhow::Error> {
        if self.runtime.is_none() {
            use deno_core::{JsRuntime, RuntimeOptions};
            let runtime = JsRuntime::new(RuntimeOptions {
                extensions: vec![rift_runtime::init()],
                ..Default::default()
            });

            // Initialize console and API
            let mut rt = runtime;
            rt.execute_script(
                "<init_console>",
                "globalThis.console = globalThis.console ?? { log: (..._args) => {}, warn: (..._args) => {}, error: (..._args) => {} };",
            )?;
            rt.execute_script("<rift_api>", RIFT_API_JS)?;

            self.runtime = Some(std::sync::Mutex::new(rt));
        }

        self.runtime
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Runtime creation failed"))
            .map(|m| {
                m.lock()
                    .map_err(|e| anyhow::anyhow!("Runtime lock failed: {}", e))
            })
            .and_then(|g| g.map_err(|e| anyhow::anyhow!("Runtime lock failed: {}", e)))
    }

    /// Execute a script entry with the given context
    pub fn run_entry_with_context(
        &mut self,
        entry: impl AsRef<Path>,
        context: &ScriptContext,
    ) -> Result<RawScriptResult> {
        let graph = self.build_graph(entry.as_ref())?;
        // TODO: Skip linting for now - need to handle global API declarations
        // for module in graph.values() {
        //     self.check_module(module)?;
        // }

        let entry_path = normalize_path(entry.as_ref())?;
        let entry_module = graph
            .get(&entry_path)
            .ok_or_else(|| anyhow!("Entry module not found in graph: {}", entry_path.display()))?;

        let js_code = transpile_typescript_to_javascript(entry_module.parsed.clone())?;

        // Helper function to escape backslashes for JavaScript strings
        let escape_js_string = |s: String| s.replace('\\', "\\\\").replace('"', "\\\"");

        // Build packages map JSON string
        let packages_json = context
            .all_packages
            .iter()
            .map(|(name, path)| {
                format!(
                    r#"["{}", "{}"]"#,
                    name,
                    escape_js_string(path.display().to_string())
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");

        // Build context injection code
        let context_code = format!(
            r#"
globalThis.Rift = {{
    package: {{ name: "{}", path: "{}" }},
    parent: {},
    root: {{ name: "{}", path: "{}" }},
    find: (name) => {{
        return _rift_packages.get(name);
    }}
}};
globalThis._rift_packages = new Map([
{}
]);
"#,
            context.package_name,
            escape_js_string(context.package_path.display().to_string()),
            if let (Some(name), Some(path)) = (&context.parent_name, &context.parent_path) {
                format!(
                    r#"{{ name: "{}", path: "{}" }}"#,
                    name,
                    escape_js_string(path.display().to_string())
                )
            } else {
                "undefined".to_string()
            },
            context.root_name,
            escape_js_string(context.root_path.display().to_string()),
            packages_json
        );

        // Use shared runtime to maintain global state (e.g., plugin APIs)
        let mut runtime = self.get_or_create_runtime()?;

        // Inject context
        runtime.execute_script("<rift_context>", context_code)?;

        // Execute user script
        let _runtime = runtime.execute_script(entry_module.specifier.to_string(), js_code)?;

        // Retrieve the collected results from OpState
        // Use try_take to remove the state so the next execution starts clean
        let op_state = runtime.op_state();
        let mut op_state_ref = op_state.borrow_mut();

        let (dependencies, plugins, config, tasks, commands, event_subscriptions) =
            match op_state_ref.try_take::<RiftOpState>() {
                Some(rift_state) => (
                    rift_state.dependencies,
                    rift_state.plugins,
                    rift_state.config,
                    rift_state.tasks,
                    rift_state.commands,
                    rift_state.event_subscriptions,
                ),
                None => (
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                ),
            };

        drop(op_state_ref); // Release the borrow before runtime is dropped

        Ok(RawScriptResult {
            success: true,
            error: None,
            dependencies,
            plugins,
            config,
            tasks,
            commands,
            event_subscriptions,
        })
    }

    pub fn validate_entry(&self, entry: impl AsRef<Path>) -> Result<()> {
        let graph = self.build_graph(entry.as_ref())?;
        for module in graph.values() {
            self.check_module(module)?;
        }
        Ok(())
    }

    pub fn run_entry(&self, entry: impl AsRef<Path>) -> Result<()> {
        let graph = self.build_graph(entry.as_ref())?;
        for module in graph.values() {
            self.check_module(module)?;
        }

        let entry = normalize_path(entry.as_ref())?;
        let entry_module = graph
            .get(&entry)
            .ok_or_else(|| anyhow!("Entry module not found in graph: {}", entry.display()))?;

        if !entry_module.dependencies.is_empty() {
            return Err(anyhow!(
                "VM runtime currently executes only single-file entry. Import graph validation already passed for: {}",
                entry.display()
            ));
        }

        let js_code = transpile_typescript_to_javascript(entry_module.parsed.clone())?;
        run_embedded_javascript(&entry_module.specifier, js_code)
    }

    fn build_graph(&self, entry: &Path) -> Result<BTreeMap<PathBuf, ModuleUnit>> {
        let mut modules = BTreeMap::new();
        let mut visiting = HashSet::new();

        let normalized = normalize_path(entry)?;
        collect_module_recursive(&normalized, &mut visiting, &mut modules)?;

        Ok(modules)
    }

    fn check_module(&self, module: &ModuleUnit) -> Result<()> {
        // Skip linting for virtual rift: modules (they're embedded JavaScript)
        if module.path.starts_with("@rift/virtual/") {
            return Ok(());
        }

        let lint_diagnostics = self.linter.lint_with_ast(
            &module.parsed,
            LintConfig {
                default_jsx_factory: None,
                default_jsx_fragment_factory: None,
            },
            None,
        );

        let mut violations = Vec::new();

        for diagnostic in lint_diagnostics {
            let (line, column) = diagnostic
                .range
                .map(|range| {
                    let loc = range.text_info.line_and_column_display(range.range.start);
                    (loc.line_number, loc.column_number)
                })
                .unwrap_or((1, 1));

            violations.push(VmViolation {
                file: module.path.clone(),
                line,
                column,
                rule: diagnostic.details.code.to_string(),
                message: diagnostic.details.message,
            });
        }

        violations.extend(find_unknown_type_violations(&module.path, &module.parsed));

        if violations.is_empty() {
            return Ok(());
        }

        let mut rendered = String::from("TypeScript gate failed:\n");
        for violation in violations {
            rendered.push_str(&violation.render());
            rendered.push('\n');
        }
        Err(anyhow!(rendered))
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}

fn collect_module_recursive(
    path: &Path,
    visiting: &mut HashSet<PathBuf>,
    modules: &mut BTreeMap<PathBuf, ModuleUnit>,
) -> Result<()> {
    if modules.contains_key(path) {
        return Ok(());
    }

    if !visiting.insert(path.to_path_buf()) {
        return Err(anyhow!(
            "Circular dependency detected at {}",
            path.display()
        ));
    }

    // Check if this is a virtual rift: module
    let is_virtual = path.starts_with("@rift/virtual/");
    let (code, virtual_specifier) = if is_virtual {
        let specifier = path
            .to_str()
            .unwrap_or("")
            .trim_start_matches("@rift/virtual/");
        (
            get_virtual_module_code(specifier)?,
            Some(specifier.to_string()),
        )
    } else {
        ensure_ts_only(path)?;
        let fs_code = fs::read_to_string(path)
            .map_err(|error| anyhow!("Cannot read file {}: {}", path.display(), error))?;
        ensure_safe_ts_source(path, &fs_code)?;
        (fs_code, None)
    };

    let media_type = MediaType::TypeScript;
    let specifier = if let Some(virt_spec) = virtual_specifier {
        deno_ast::ModuleSpecifier::parse(&format!("rift:{}", virt_spec))
            .map_err(|_| anyhow!("Invalid virtual module specifier: {}", virt_spec))?
    } else {
        deno_ast::ModuleSpecifier::from_file_path(path)
            .map_err(|_| anyhow!("Invalid module path: {}", path.display()))?
    };

    let parsed = parse_typescript_module(specifier.clone(), &code, media_type)?;
    let dependencies = extract_dependencies(path, &parsed)?;

    modules.insert(
        path.to_path_buf(),
        ModuleUnit {
            specifier,
            path: path.to_path_buf(),
            parsed,
            dependencies: dependencies.clone(),
        },
    );

    for dep in dependencies {
        collect_module_recursive(&dep, visiting, modules)?;
    }

    visiting.remove(path);
    Ok(())
}

fn ensure_ts_only(file_path: &Path) -> Result<()> {
    // Allow virtual rift: modules
    if file_path.starts_with("@rift/virtual/") {
        return Ok(());
    }

    if !file_path.is_file() {
        return Err(anyhow!(
            "Only TypeScript source files are allowed, got non-file path: {}",
            file_path.display()
        ));
    }

    let extension = file_path
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or_default();

    if extension != "ts" {
        return Err(anyhow!(
            "Only TypeScript .ts files are allowed, got: {}",
            file_path.display()
        ));
    }

    Ok(())
}

fn ensure_safe_ts_source(file_path: &Path, code: &str) -> Result<()> {
    if code.contains('\0') {
        return Err(anyhow!(
            "Binary-like content is forbidden in TypeScript source: {}",
            file_path.display()
        ));
    }

    if code.starts_with("#!") {
        return Err(anyhow!(
            "Shebang is forbidden in TypeScript source: {}",
            file_path.display()
        ));
    }

    Ok(())
}

fn build_strict_linter() -> Linter {
    let all_rules = get_all_rules();

    // Filter out rules that conflict with Rift's design
    // - no-external-import: we allow rift: imports
    // - prefer-primordials: our virtual module uses built-ins directly
    // - camelcase: internal virtual module uses underscore-prefixed names
    // - ban-untagged-todo: we use TODO comments in virtual module code
    // - no-console: console.log is useful for debugging in scripts
    let excluded_rules = [
        "no-external-import",
        "prefer-primordials",
        "camelcase",
        "ban-untagged-todo",
        "no-console",
    ];

    let allowed_rules = all_rules
        .into_iter()
        .filter(|rule| !excluded_rules.contains(&rule.code()))
        .collect::<Vec<_>>();

    let all_rule_codes = allowed_rules
        .iter()
        .map(|rule| Cow::Borrowed(rule.code()))
        .collect::<HashSet<_>>();

    Linter::new(LinterOptions {
        rules: allowed_rules,
        all_rule_codes,
        custom_ignore_file_directive: None,
        custom_ignore_diagnostic_directive: None,
    })
}

fn parse_typescript_module(
    specifier: deno_ast::ModuleSpecifier,
    code: &str,
    media_type: MediaType,
) -> Result<ParsedSource> {
    if !matches!(media_type, MediaType::TypeScript) {
        return Err(anyhow!("Only TypeScript files (.ts) are allowed!"));
    }

    Ok(deno_ast::parse_module(ParseParams {
        specifier,
        text: code.into(),
        media_type,
        capture_tokens: true,
        scope_analysis: true,
        maybe_syntax: None,
    })?)
}

fn find_unknown_type_violations(path: &Path, parsed: &ParsedSource) -> Vec<VmViolation> {
    let mut visitor = UnknownTypeVisitor { spans: Vec::new() };
    parsed.program_ref().visit_with(&mut visitor);

    visitor
        .spans
        .into_iter()
        .map(|span| {
            let source_pos = SourcePos::unsafely_from_byte_pos(span.lo);
            let line_column = parsed.text_info_lazy().line_and_column_display(source_pos);

            VmViolation {
                file: path.to_path_buf(),
                line: line_column.line_number,
                column: line_column.column_number,
                rule: "no-unknown-type".to_string(),
                message: "`unknown` type is forbidden".to_string(),
            }
        })
        .collect()
}

fn extract_dependencies(base_path: &Path, parsed: &ParsedSource) -> Result<Vec<PathBuf>> {
    let mut dependencies = Vec::new();

    for item in parsed.program_ref().body() {
        let specifier = match item {
            ModuleItemRef::ModuleDecl(ModuleDecl::Import(import_decl)) => {
                Some(import_decl.src.value.to_string_lossy().into_owned())
            }
            ModuleItemRef::ModuleDecl(ModuleDecl::ExportNamed(named_decl)) => named_decl
                .src
                .as_ref()
                .map(|src| src.value.to_string_lossy().into_owned()),
            ModuleItemRef::ModuleDecl(ModuleDecl::ExportAll(all_decl)) => {
                Some(all_decl.src.value.to_string_lossy().into_owned())
            }
            _ => None,
        };

        if let Some(specifier) = specifier {
            let path = resolve_import_specifier(base_path, &specifier)?;
            dependencies.push(path);
        }
    }

    Ok(dependencies)
}

/// Get the code for a virtual rift: module
fn get_virtual_module_code(specifier: &str) -> Result<String> {
    match specifier {
        "rift:api" | "rift:api/index" => Ok(RIFT_API_JS.to_string()),
        _ => Err(anyhow!("Unknown virtual module: {}", specifier)),
    }
}

/// Built-in Rift API JavaScript implementation
/// This provides the runtime implementation of the Rift API exposed to scripts
pub const RIFT_API_JS: &str = r#"
// Rift Built-in API

// Get reference to ops from Deno.core
const ops = globalThis.Deno.core.ops;

// ===== TypeScript-style Functional API =====

// Helper: normalize dependency to object form
function normalizeDependency(dep) {
    if (typeof dep === 'string' || typeof dep === 'undefined') {
        throw new Error('String dependencies are not supported. Use object form: { name: "...", version?: "...", source?: "explicit"|"workspace"|"inherit"|"git"|"path" }');
    }
    return dep;
}

// Add a dependency (supports single or array)
// source can be: "explicit" (default), "workspace", "inherit", "git", "path"
function addDependency(dep) {
    const addOne = (d) => {
        const normalized = normalizeDependency(d);
        const name = normalized.name;
        const version = normalized.version;
        const source = normalized.source || 'explicit';
        const attributes = { ...normalized.attributes } || {};
        const git = normalized.git;
        const path = normalized.path;

        // Handle git source
        if (source === 'git') {
            if (!git || !git.url) {
                throw new Error(`Git dependency "${name}" must have a git.url field`);
            }
            attributes.source = 'git';
            attributes.git = git;
        }
        // Handle path source
        else if (source === 'path') {
            if (!path || !path.path) {
                throw new Error(`Path dependency "${name}" must have a path.path field`);
            }
            attributes.source = 'path';
            attributes.path = path;
        }
        // Set source in attributes for non-explicit sources
        else if (source !== 'explicit') {
            attributes.source = source;
        }

        const attributesJson = Object.keys(attributes).length > 0 ? JSON.stringify(attributes) : undefined;
        ops.op_rift_add_dependency(name, version, attributesJson);
    };

    if (Array.isArray(dep)) {
        dep.forEach(addOne);
    } else {
        addOne(dep);
    }
}

// Exclude a dependency (prevents inheritance)
function excludeDependency(name) {
    if (typeof name !== 'string') {
        throw new Error('excludeDependency() requires a package name string');
    }
    const attributes = { excluded: true };
    ops.op_rift_add_dependency(name, undefined, JSON.stringify(attributes));
}

// Add a plugin (supports single or array)
function addPlugin(plugin) {
    const addOne = (p) => {
        const normalized = normalizeDependency(p);
        const name = normalized.name;
        const version = normalized.version;
        ops.op_rift_add_plugin(name, version);
    };

    if (Array.isArray(plugin)) {
        plugin.forEach(addOne);
    } else {
        addOne(plugin);
    }
}

// Configure the current package
function configurePackage(callback) {
    if (typeof callback === 'function') {
        const config = {
            set: (key, value) => {
                let valueType;
                let valueStr;
                if (typeof value === 'string') {
                    valueType = 'string';
                    valueStr = value;
                } else if (typeof value === 'number') {
                    valueType = 'number';
                    valueStr = String(value);
                } else if (typeof value === 'boolean') {
                    valueType = 'boolean';
                    valueStr = String(value);
                } else {
                    return;
                }
                ops.op_rift_set_config(key, valueType, valueStr);
            }
        };
        callback(config);
    }
}

// ===== Legacy API (for backward compatibility) =====

// Deprecated: Use addDependency({ name: ..., source: "workspace" }) instead
function addWorkspaceRef(name) {
    console.warn('addWorkspaceRef is deprecated. Use addDependency({ name: "' + name + '", source: "workspace" }) instead.');
    addDependency({ name, source: 'workspace' });
}
const dependencies = {
    add: addDependency,
    ref: addWorkspaceRef
};

// Plugins namespace (deprecated - use addPlugin instead)
const plugins = {
    add: addPlugin
};

// Package configuration namespace (deprecated - use configurePackage instead)
const pkgConfig = {
    configure: configurePackage
};

// Tasks API (camelCase) - object configuration style
const tasks = {
    register: (name, options) => {
        // Support both object and function styles for backward compatibility
        let config;

        if (typeof options === 'function') {
            // Old builder style
            config = {
                description: '',
                dependencies: [],
                isCommand: false,
                action: null,

                setDescription: function(desc) {
                    this.description = desc;
                    return this;
                },
                dependsOn: function(deps) {
                    this.dependencies = Array.isArray(deps) ? deps : [deps];
                    return this;
                },
                asCommand: function(value = true) {
                    this.isCommand = value;
                    return this;
                },
                do: function(fn) {
                    this.action = fn;
                    return this;
                }
            };

            options(config);
        } else {
            // New object style
            config = {
                description: options.description || '',
                dependencies: options.dependsOn || options.dependencies || [],
                isCommand: options.command ?? options.asCommand ?? false,
                action: options.run || options.do || null
            };
        }

        // Register the task via op
        const depsStr = Array.isArray(config.dependencies)
            ? config.dependencies.join(',')
            : '';
        const hasAction = typeof config.action === 'function';
        ops.op_rift_register_task(
            name,
            config.description,
            depsStr,
            String(config.isCommand),
            String(hasAction)
        );

        // Store action for later execution
        if (hasAction) {
            globalThis._rift_task_actions = globalThis._rift_task_actions || {};
            globalThis._rift_task_actions[name] = config.action;
        }
    }
};

// ===== PLUGIN API =====

// Event handlers storage
globalThis._rift_event_handlers = globalThis._rift_event_handlers || {};
globalThis._rift_command_handlers = globalThis._rift_command_handlers || {};

// Define a command (plugin system)
// rift.defineCommand(name, handler)
const defineCommand = (name, handler) => {
    if (typeof handler !== 'function') {
        throw new Error(`Command handler must be a function, got ${typeof handler}`);
    }

    // Register via op
    ops.op_rift_define_command(name);

    // Store handler for execution
    globalThis._rift_command_handlers[name] = handler;
};

// Subscribe to an event (plugin system)
// rift.on(eventName, handler)
const on = (eventName, handler) => {
    if (typeof handler !== 'function') {
        throw new Error(`Event handler must be a function, got ${typeof handler}`);
    }

    // Register via op
    ops.op_rift_on(eventName);

    // Store handler
    globalThis._rift_event_handlers[eventName] = globalThis._rift_event_handlers[eventName] || [];
    globalThis._rift_event_handlers[eventName].push(handler);
};

// Emit an event (plugin system)
// rift.emit(eventName, context)
const emit = (eventName, context) => {
    const contextJson = typeof context === 'string' ? context : JSON.stringify(context || {});

    // Emit via op (triggers Rust-side handlers)
    try {
        ops.op_rift_emit(eventName, contextJson);
    } catch (e) {
        console.error(`Error emitting event '${eventName}':`, e);
    }

    // Also trigger JavaScript-side handlers
    const handlers = globalThis._rift_event_handlers[eventName];
    if (handlers) {
        for (const handler of handlers) {
            try {
                handler(context);
            } catch (e) {
                console.error(`Error in handler for event '${eventName}':`, e);
            }
        }
    }
};

// Write file API (for file generation)
// rift.writeFile(path, content)
const writeFile = (path, content) => {
    return ops.op_rift_write_file(path, content || '');
};

// ===== END PLUGIN API =====

// ===== Export API =====

// TypeScript-style functional API (primary)
globalThis.addDependency = addDependency;
globalThis.excludeDependency = excludeDependency;
globalThis.addPlugin = addPlugin;
globalThis.configurePackage = configurePackage;
globalThis.defineCommand = defineCommand;
globalThis.on = on;
globalThis.emit = emit;
globalThis.writeFile = writeFile;
globalThis.tasks = tasks;

// Deprecated aliases (for backward compatibility)
globalThis.addWorkspaceRef = addWorkspaceRef;

// Legacy namespace API (backward compatibility)
globalThis.dependencies = dependencies;
globalThis.plugins = plugins;
globalThis.pkgConfig = pkgConfig;

// PascalCase aliases (backward compatibility)
globalThis.Dependencies = dependencies;
globalThis.Plugins = plugins;
globalThis.PkgConfig = pkgConfig;
globalThis.Package = pkgConfig;
globalThis.Tasks = tasks;
globalThis.DefineCommand = defineCommand;
globalThis.On = on;
globalThis.Emit = emit;
globalThis.WriteFile = writeFile;

// Also provide 'package' as an alias for 'pkgConfig' (for backward compatibility)
// Note: we can't use 'package' directly as it's a reserved keyword in some contexts
Object.defineProperty(globalThis, 'package', {
    value: pkgConfig,
    writable: false,
    configurable: false,
    enumerable: false
});
Object.defineProperty(globalThis, 'Package', {
    value: pkgConfig,
    writable: false,
    configurable: false,
    enumerable: false
});

// Config API (for configure scripts)
// Config API
const config = {
    set: (key, value) => {
        // Determine value type
        let valueType, valueStr;
        if (typeof value === 'string') {
            valueType = 'string';
            valueStr = value;
        } else if (typeof value === 'number') {
            valueType = 'number';
            valueStr = String(value);
        } else if (typeof value === 'boolean') {
            valueType = 'boolean';
            valueStr = String(value);
        } else {
            return; // Unsupported type
        }
        ops.op_rift_set_config(key, valueType, valueStr);
    }
};

// Make config available globally
globalThis.config = config;
globalThis.Config = config;
"#;

#[derive(Default)]
struct UnknownTypeVisitor {
    spans: Vec<Span>,
}

impl Visit for UnknownTypeVisitor {
    fn visit_ts_keyword_type(&mut self, keyword: &TsKeywordType) {
        if matches!(keyword.kind, TsKeywordTypeKind::TsUnknownKeyword) {
            self.spans.push(keyword.span);
        }

        keyword.visit_children_with(self);
    }
}

fn resolve_import_specifier(base_path: &Path, specifier: &str) -> Result<PathBuf> {
    // Handle built-in rift: API imports
    if specifier.starts_with("rift:") {
        // Return a special virtual path for rift: modules
        // These will be handled specially during module loading
        let virtual_path = PathBuf::from(format!("@rift/virtual/{}", specifier));
        return Ok(virtual_path);
    }

    if specifier.starts_with("npm:")
        || specifier.starts_with("node:")
        || specifier.starts_with("http:")
        || specifier.starts_with("https:")
    {
        return Err(anyhow!(
            "Only local TypeScript modules are allowed, got forbidden specifier: {}",
            specifier
        ));
    }

    if !(specifier.starts_with("./") || specifier.starts_with("../") || specifier.starts_with('/'))
    {
        return Err(anyhow!(
            "Package manager / registry imports are forbidden: {}",
            specifier
        ));
    }

    let parent = base_path.parent().unwrap_or_else(|| Path::new("."));
    let mut candidate = if specifier.starts_with('/') {
        PathBuf::from(specifier)
    } else {
        parent.join(specifier)
    };

    if candidate.extension().is_none() {
        let with_ts = candidate.with_extension("ts");
        if with_ts.exists() {
            candidate = with_ts;
        } else {
            let index_ts = candidate.join("index.ts");
            if index_ts.exists() {
                candidate = index_ts;
            }
        }
    }

    let normalized = normalize_path(&candidate)?;
    ensure_ts_only(&normalized)?;
    Ok(normalized)
}

fn normalize_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(anyhow!("Module does not exist: {}", path.display()));
    }
    path.canonicalize()
        .map_err(|error| anyhow!("Cannot normalize path {}: {}", path.display(), error))
}

fn transpile_typescript_to_javascript(parsed_source: ParsedSource) -> Result<String> {
    let emitted = parsed_source
        .transpile(
            &TranspileOptions::default(),
            &TranspileModuleOptions::default(),
            &EmitOptions {
                source_map: SourceMapOption::None,
                ..Default::default()
            },
        )?
        .into_source();

    Ok(emitted.text)
}

fn run_embedded_javascript(specifier: &deno_ast::ModuleSpecifier, js_code: String) -> Result<()> {
    let mut runtime = JsRuntime::new(RuntimeOptions::default());

    // Initialize console
    runtime.execute_script(
        "<init_console>",
        "globalThis.console = globalThis.console ?? { log: (..._args) => {}, warn: (..._args) => {}, error: (..._args) => {} };",
    )?;

    // Initialize Rift context object
    // TODO: Populate with actual package information from Rust
    runtime.execute_script(
        "<init_rift>",
        r#"
        globalThis.Rift = {
            package: { name: "", path: "" },
            parent: undefined,
            root: { name: "", path: "" },
            find: (name) => undefined
        };
        "#,
    )?;

    runtime.execute_script(specifier.to_string(), js_code)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bad_js() {
        let code = "const x = 123;";
        let specifier = deno_ast::ModuleSpecifier::parse("file:///test_bad_js.js").unwrap();

        let result = parse_typescript_module(specifier, code, MediaType::JavaScript);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Only TypeScript files (.ts) are allowed")
        );
    }

    #[test]
    fn test_bad_ts_any_unknown() {
        let code = r#"
        const x: any = 123;
        const y: unknown = x;
        "#;
        let vm = Vm::new();
        let temp = std::env::temp_dir().join("rift_vm_bad_any_unknown.ts");
        fs::write(&temp, code).unwrap();

        let result = vm.validate_entry(&temp);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(error.contains("no-explicit-any"));
        assert!(error.contains("no-unknown-type"));

        let _ = fs::remove_file(temp);
    }

    #[test]
    fn test_good_ts() {
        let code = r#"
        function add(a: number, b: number): number {
            return a + b;
        }
        add(1, 2);
        "#;
        let vm = Vm::new();
        let temp = std::env::temp_dir().join("rift_vm_good.ts");
        fs::write(&temp, code).unwrap();

        let result = vm.validate_entry(&temp);

        assert!(result.is_ok());
        let _ = fs::remove_file(temp);
    }

    #[test]
    fn test_import_graph_validate() {
        let vm = Vm::new();
        let root = std::env::temp_dir().join("rift_vm_graph_test");
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("lib")).unwrap();

        let entry = root.join("main.ts");
        let entry_code = r#"
        import { add } from './lib/math.ts';
        export const value: number = add(1, 2);
        "#;

        let dep = root.join("lib").join("math.ts");
        let dep_code = r#"
        export function add(a: number, b: number): number {
            return a + b;
        }
        "#;

        fs::write(&dep, dep_code).unwrap();
        fs::write(&entry, entry_code).unwrap();

        let result = vm.validate_entry(&entry);

        assert!(result.is_ok());
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn test_rift_api_import() {
        let vm = Vm::new();
        let temp = std::env::temp_dir().join("rift_api_test");
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();

        let entry = temp.join("main.ts");
        let entry_code = r#"
        import { addDependency, addPlugin, configurePackage } from "rift:api";

        // This should compile without errors
        addDependency({ name: "test", version: "1.0.0" });
        addPlugin({ name: "some-plugin" });
        configurePackage((config) => {
            config.set("optimize", "true");
        });
        "#;

        fs::write(&entry, entry_code).unwrap();

        let result = vm.validate_entry(&entry);

        if let Err(e) = &result {
            eprintln!("Error: {}", e);
        }

        assert!(result.is_ok(), "rift:api import should work");
        let _ = fs::remove_dir_all(temp);
    }

    #[test]
    fn test_rift_context_object() {
        let vm = Vm::new();
        let temp = std::env::temp_dir().join("rift_context_test");
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();

        let entry = temp.join("main.ts");
        let entry_code = r#"
        import { Rift } from "rift:api";

        // Rift context object should be available
        const pkgName = Rift.package.name;
        const rootName = Rift.root.name;

        console.log("Package:", pkgName, "Root:", rootName);
        "#;

        fs::write(&entry, entry_code).unwrap();

        let result = vm.validate_entry(&entry);

        if let Err(e) = &result {
            eprintln!("Error: {}", e);
        }

        assert!(result.is_ok(), "Rift context object should be available");
        let _ = fs::remove_dir_all(temp);
    }

    #[test]
    fn test_workspace_ref_api() {
        let vm = Vm::new();
        let temp = std::env::temp_dir().join("rift_ref_test");
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();

        let entry = temp.join("main.ts");
        let entry_code = r#"
        import { addDependency, excludeDependency } from "rift:api";

        // Test workspace reference using source field
        addDependency({ name: "shared-utils", source: "workspace" });
        addDependency({ name: "core-library", source: "workspace" });

        // Test regular dependency with attributes
        addDependency({
            name: "lodash",
            version: "4.17.21",
            attributes: { dev: true }
        });

        // Test inherit source
        addDependency({ name: "config", source: "inherit" });

        // Test exclude
        excludeDependency("unwanted-package");
        "#;

        fs::write(&entry, entry_code).unwrap();

        let result = vm.validate_entry(&entry);

        if let Err(e) = &result {
            eprintln!("Error: {}", e);
        }

        assert!(result.is_ok(), "Dependencies API should work");
        let _ = fs::remove_dir_all(temp);
    }

    #[test]
    fn test_workspace_ref_execution() {
        let mut vm = Vm::new();
        let temp = std::env::temp_dir().join("rift_ref_exec_test");
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();

        let entry = temp.join("main.ts");
        let entry_code = r#"
        // Use the new API with source field
        addDependency({ name: "shared-package", source: "workspace" });
        addDependency({ name: "external-lib", version: "1.0.0" });
        addDependency({ name: "inherited-pkg", source: "inherit" });
        "#;

        fs::write(&entry, entry_code).unwrap();

        // Create a script context for execution
        let mut all_packages = HashMap::new();
        all_packages.insert("my-package".to_string(), temp.clone());
        all_packages.insert("shared-package".to_string(), temp.join("shared"));

        let context = ScriptContext {
            package_name: "my-package".to_string(),
            package_path: temp.clone(),
            parent_name: None,
            parent_path: None,
            root_name: "my-workspace".to_string(),
            root_path: temp.clone(),
            all_packages,
        };

        let result = vm.run_entry_with_context(&entry, &context);

        if let Err(e) = &result {
            eprintln!("Script execution error: {}", e);
        }

        assert!(result.is_ok(), "Script execution should succeed");

        let script_result = result.unwrap();
        assert_eq!(script_result.dependencies.len(), 3);

        // First dependency: workspace reference (source: "workspace" in attributes)
        let ref_dep = &script_result.dependencies[0];
        assert_eq!(ref_dep.name, "shared-package");
        assert_eq!(ref_dep.version, None);
        assert!(ref_dep.attributes.contains_key("source"));
        assert_eq!(
            ref_dep.attributes.get("source"),
            Some(&serde_json::json!("workspace"))
        );

        // Second dependency: external with version
        let ext_dep = &script_result.dependencies[1];
        assert_eq!(ext_dep.name, "external-lib");
        assert_eq!(ext_dep.version, Some("1.0.0".to_string()));

        // Third dependency: inherit
        let inherit_dep = &script_result.dependencies[2];
        assert_eq!(inherit_dep.name, "inherited-pkg");
        assert_eq!(inherit_dep.version, None);
        assert!(inherit_dep.attributes.contains_key("source"));
        assert_eq!(
            inherit_dep.attributes.get("source"),
            Some(&serde_json::json!("inherit"))
        );
        assert!(ext_dep.attributes.is_empty());

        let _ = fs::remove_dir_all(temp);
    }
}
