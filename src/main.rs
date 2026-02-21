use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

mod cli;
mod manifest;
mod plugin;
mod schema;
mod tasks;
mod vm;
mod workspace;

use cli::{parse_args, register_command_tasks};
use plugin::{get_plugin_commands, init_event_bus, is_plugin_command};
use vm::Vm;
use workspace::WorkspaceBuilder;
use workspace::executor::{DependencySource, PackageReference, ScriptExecutor, ScriptResult};
use workspace::package::MaybePackage;

fn main() -> Result<()> {
    // Initialize the plugin event bus
    init_event_bus();

    // First pass: quick scan to discover tasks
    // First pass: quick scan to discover tasks
    // This allows us to support direct task execution like `rift app.build`
    let args: Vec<String> = std::env::args().collect();

    // Check if we need to discover tasks first
    // Skip for: scan, tasks, help
    let needs_task_discovery = if args.len() <= 1 {
        false // Will show help
    } else {
        let first = &args[1];
        !matches!(first.as_str(), "scan" | "tasks" | "help" | "--help" | "-h")
    };

    // Discover and register tasks if needed
    if needs_task_discovery {
        let root = PathBuf::from(".").canonicalize()?;
        if let Ok((packages, manifests)) = scan_workspace(&root) {
            if let Ok((executor, _results)) = execute_scripts(&root, &packages, &manifests) {
                let task_manager = executor.task_manager();
                let command_tasks: Vec<String> = task_manager
                    .get_command_tasks()
                    .into_iter()
                    .map(|t| t.name)
                    .collect();
                register_command_tasks(command_tasks);
            }
        }
    }

    // Parse CLI arguments with smart task detection
    let action = parse_args();

    // Execute the action
    match action {
        cli::CliAction::Scan(path) => cmd_scan(path),
        cli::CliAction::Tasks(path) => cmd_tasks(path),
        cli::CliAction::Generate(path) => cmd_generate(path),
        cli::CliAction::ExecuteTask(task_name) => cmd_exec(task_name),
    }
}

fn scan_workspace(
    root: &PathBuf,
) -> Result<(
    std::collections::HashMap<String, workspace::package::MaybePackage>,
    std::collections::HashMap<String, PathBuf>,
)> {
    let builder = workspace::WorkspaceBuilder::new(root.clone());
    let (_workspace, packages, manifests) = builder.build()?;
    Ok((packages, manifests))
}

fn execute_scripts(
    root: &PathBuf,
    packages: &std::collections::HashMap<String, workspace::package::MaybePackage>,
    manifests: &std::collections::HashMap<String, PathBuf>,
) -> Result<(
    workspace::ScriptExecutor,
    std::collections::HashMap<String, workspace::ScriptResult>,
)> {
    let mut executor = workspace::ScriptExecutor::new(root.clone());
    let results = executor.execute_all(packages, manifests)?;
    Ok((executor, results))
}

fn cmd_scan(path: Option<PathBuf>) -> Result<()> {
    let root = path.unwrap_or_else(|| PathBuf::from("."));
    let root = root.canonicalize()?;

    println!("Scanning workspace at: {}", root.display());
    println!();

    let builder = workspace::WorkspaceBuilder::new(root);
    let (workspace, packages, _manifests) = builder.build()?;

    // Print all discovered packages
    println!("Workspace structure:");
    println!();
    print_package(&workspace, 0);

    // Print all other packages
    for (name, pkg) in &packages {
        if name != workspace.name() {
            print_package(pkg, 1);
        }
    }

    // Print summary
    println!();
    println!("Total packages found: {}", packages.len());

    Ok(())
}

fn cmd_tasks(path: Option<PathBuf>) -> Result<()> {
    let root = path.unwrap_or_else(|| PathBuf::from("."));
    let root = root.canonicalize()?;

    println!("Loading tasks from workspace at: {}", root.display());
    println!();

    let (packages, manifests) = scan_workspace(&root)?;
    let (executor, _results) = execute_scripts(&root, &packages, &manifests)?;

    // Show registered tasks
    let task_manager = executor.task_manager();
    let all_tasks = task_manager.get_all_tasks();
    let command_tasks = task_manager.get_command_tasks();

    println!();
    println!("Registered tasks ({}):", all_tasks.len());
    for task in &all_tasks {
        let cmd_marker = if task.is_command { " [command]" } else { "" };
        println!("  - {}{}: {}", task.name, cmd_marker, task.description);
        if !task.dependencies.is_empty() {
            println!("    depends on: {}", task.dependencies.join(", "));
        }
    }

    println!();
    println!("Command tasks ({}):", command_tasks.len());
    for task in &command_tasks {
        println!("  - {}: {}", task.name, task.description);
    }

    println!();
    println!("Run tasks with: rift <project> <task> or rift <task>");
    println!("Example: rift app build");
    println!("         rift build       (for root project tasks)");

    Ok(())
}

fn cmd_generate(path: Option<PathBuf>) -> Result<()> {
    let root = path.unwrap_or_else(|| PathBuf::from("."));
    let root = root.canonicalize()?;

    println!(
        "Generating project files for workspace at: {}",
        root.display()
    );
    println!();

    let (packages, manifests) = scan_workspace(&root)?;
    let (executor, script_results) = execute_scripts(&root, &packages, &manifests)?;

    println!("Found {} package(s) to process", script_results.len());
    println!();

    // Find the workspace package
    let workspace_name = executor.root_name();
    let workspace_pkg = packages
        .iter()
        .find(|(name, _pkg)| name == &workspace_name)
        .map(|(_, pkg)| pkg);

    if let Some(MaybePackage::Workspace(workspace)) = workspace_pkg {
        println!("Workspace: {}", workspace.name);

        // Generate go.work file (only for actual modules, not folders or workspace)
        let folder_names: std::collections::HashSet<String> = packages
            .iter()
            .filter(|(_, pkg)| matches!(pkg, MaybePackage::Folder(_)))
            .map(|(name, _)| name.clone())
            .collect();
        generate_go_work(
            &root,
            &script_results,
            &workspace_name,
            &folder_names,
            &manifests,
        )?;

        // Generate go.mod files for each package
        for (pkg_name, script_result) in &script_results {
            if let Some(manifest_path) = manifests.get(pkg_name) {
                let pkg_dir = manifest_path.parent().unwrap_or(&root);
                generate_go_mod(
                    pkg_dir,
                    pkg_name,
                    &workspace_name,
                    &script_result.dependencies,
                    &script_results,
                )?;
            }
        }

        println!();
        println!("Generation complete!");
        println!("Generated files:");
        println!("  - go.work");
        for pkg_name in script_results.keys() {
            if pkg_name != &workspace_name {
                println!("  - {}/go.mod", pkg_name);
            }
        }
    } else {
        println!("No workspace found. Please run from a workspace root.");
    }

    Ok(())
}

fn generate_go_work(
    root: &PathBuf,
    script_results: &HashMap<String, workspace::executor::ScriptResult>,
    workspace_name: &str,
    folder_names: &std::collections::HashSet<String>,
    manifests: &HashMap<String, PathBuf>,
) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let go_work_path = root.join("go.work");
    let mut file = File::create(&go_work_path)?;

    writeln!(file, "go 1.22.0")?;
    writeln!(file)?;

    writeln!(file, "use (")?;
    for pkg_name in script_results.keys() {
        // Skip workspace itself and folder packages
        if pkg_name != workspace_name && !folder_names.contains(pkg_name) {
            if let Some(manifest_path) = manifests.get(pkg_name) {
                // Calculate relative path from root to package directory
                let pkg_dir = manifest_path.parent().unwrap_or(root);
                let relative_path = pkg_dir.strip_prefix(root).unwrap_or(pkg_dir);
                // Convert to forward slashes for Go compatibility
                let path_str = relative_path.display().to_string().replace('\\', "/");
                writeln!(file, "\t\"./{}\"", path_str)?;
            }
        }
    }
    writeln!(file, ")")?;

    println!("  Generated: {}", go_work_path.display());
    Ok(())
}

fn generate_go_mod(
    pkg_dir: &Path,
    pkg_name: &str,
    workspace_name: &str,
    dependencies: &[PackageReference],
    all_results: &HashMap<String, workspace::executor::ScriptResult>,
) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let go_mod_path = pkg_dir.join("go.mod");
    let mut file = File::create(&go_mod_path)?;

    // Write module declaration with workspace prefix for child packages
    // Root workspace uses just its name, child packages use workspace/package format
    let full_module_name = if pkg_name == workspace_name {
        pkg_name.to_string()
    } else {
        format!("{}/{}", workspace_name, pkg_name)
    };
    writeln!(file, "module {}", full_module_name)?;
    writeln!(file)?;

    writeln!(file, "go 1.22.0")?;
    writeln!(file)?;

    // Collect all dependencies (both workspace-local and external)
    let mut required_deps = std::collections::BTreeSet::new();

    // Helper to find workspace dependencies
    let workspace_result = all_results
        .values()
        .find(|r| r.package_name.ends_with("workspace") || r.package_name.contains("workspace"));

    for dep in dependencies {
        match dep.source {
            DependencySource::Workspace => {
                // Workspace reference: look up the specific dependency in workspace
                // Skip local workspace packages (handled by go.work)
                if dep.name.contains('/') {
                    // External dependency - find version in workspace
                    if let Some(ws_result) = workspace_result {
                        if let Some(workspace_dep) =
                            ws_result.dependencies.iter().find(|d| d.name == dep.name)
                        {
                            let version_str = workspace_dep.version.as_deref().unwrap_or("0.0.0");
                            let version = if version_str.starts_with('v') {
                                version_str.to_string()
                            } else {
                                format!("v{}", version_str)
                            };
                            required_deps.insert(format!("{} {}", workspace_dep.name, version));
                        }
                    }
                }
                // Local packages (without /) are handled by go.work, skip them
            }
            DependencySource::Inherit => {
                // Inherited from parent - use the resolved dependency (now has version)
                let version_str = dep.version.as_deref().unwrap_or("0.0.0");
                let version = if version_str.starts_with('v') {
                    version_str.to_string()
                } else {
                    format!("v{}", version_str)
                };
                required_deps.insert(format!("{} {}", dep.name, version));
            }
            DependencySource::Explicit => {
                // Explicitly declared version
                let version_str = dep.version.as_deref().unwrap_or("0.0.0");
                let version = if version_str.starts_with('v') {
                    version_str.to_string()
                } else {
                    format!("v{}", version_str)
                };
                required_deps.insert(format!("{} {}", dep.name, version));
            }
        }
    }

    // Write require statements
    if !required_deps.is_empty() {
        if required_deps.len() == 1 {
            // Single dependency - use simple format
            for dep in required_deps {
                writeln!(file, "require {}", dep)?;
            }
        } else {
            // Multiple dependencies - use block format
            writeln!(file, "require (")?;
            for dep in required_deps {
                writeln!(file, "\t{}", dep)?;
            }
            writeln!(file, ")")?;
        }
    }

    println!("  Generated: {}", go_mod_path.display());
    Ok(())
}

fn cmd_exec(task_name: String) -> Result<()> {
    let root = PathBuf::from(".").canonicalize()?;

    println!("Loading tasks from workspace at: {}", root.display());
    println!();

    let (packages, manifests) = scan_workspace(&root)?;
    let (executor, script_results) = execute_scripts(&root, &packages, &manifests)?;

    // Convert "app build" to "app.project" for lookup
    let lookup_name = task_name.replace(' ', ".");

    // First, check if it's a plugin command
    if is_plugin_command(&task_name) {
        return execute_plugin_command(&root, &task_name, &packages, &manifests, &script_results);
    }

    // Find and execute the task
    let task_manager = executor.task_manager();
    let task_to_execute = task_manager.find_task(&lookup_name);

    match task_to_execute {
        Some(t) => {
            println!();
            println!("Executing task: {}", t.name);
            println!("Description: {}", t.description);
            if !t.dependencies.is_empty() {
                println!("Dependencies: {}", t.dependencies.join(", "));
            }
            println!();

            // TODO: Execute task action
            // For now, we just print what would be executed
            println!("Task execution not yet implemented.");
            println!("The task '{}' was found and ready to execute.", t.name);

            // Note: To actually execute task actions, we would need to:
            // 1. Store the JavaScript action functions during registration
            // 2. Create a new v8 runtime or reuse existing one
            // 3. Call the stored action function

            Ok(())
        }
        None => {
            println!();
            println!("Error: Task '{}' not found.", task_name);
            println!();
            println!("Available tasks:");
            let all_tasks = task_manager.get_all_tasks();
            for t in &all_tasks {
                let cmd_marker = if t.is_command { " [command]" } else { "" };
                println!("  - {}{}: {}", t.name, cmd_marker, t.description);
            }
            println!();
            println!("Available plugin commands:");
            for cmd in get_plugin_commands() {
                println!("  - {}", cmd);
            }
            Err(anyhow::anyhow!("Task not found: {}", task_name))
        }
    }
}

/// Execute a plugin command by reloading plugins and calling the handler
fn execute_plugin_command(
    root: &PathBuf,
    command_name: &str,
    packages: &HashMap<String, workspace::package::MaybePackage>,
    manifests: &HashMap<String, PathBuf>,
    script_results: &HashMap<String, workspace::ScriptResult>,
) -> Result<()> {
    println!("Executing plugin command: {}", command_name);
    println!();

    // Find root workspace
    let (root_name, root_path) = packages
        .iter()
        .find(|(_, pkg)| matches!(pkg, workspace::package::MaybePackage::Workspace(_)))
        .map(|(name, _)| {
            let path = manifests
                .get(name)
                .and_then(|p| p.parent())
                .map(|p| p.to_path_buf())
                .unwrap_or_default();
            (name.clone(), path)
        })
        .unwrap_or_else(|| ("root".to_string(), root.clone()));

    // Build all_packages map
    let all_packages: HashMap<String, PathBuf> = packages
        .iter()
        .filter_map(|(name, _)| {
            manifests
                .get(name)
                .and_then(|p| p.parent())
                .map(|p| (name.clone(), p.to_path_buf()))
        })
        .collect();

    // Collect all plugin index.ts file contents to load
    let mut plugin_scripts: Vec<(String, String)> = Vec::new();
    for (name, pkg) in packages {
        if matches!(pkg, workspace::package::MaybePackage::Plugin(_)) {
            if let Some(manifest_path) = manifests.get(name) {
                let index_path = manifest_path.parent().map(|p| p.join("index.ts"));
                if let Some(path) = index_path {
                    if path.exists() {
                        // Read the TypeScript file
                        let ts_content = std::fs::read_to_string(&path)?;
                        plugin_scripts.push((path.display().to_string(), ts_content));
                    }
                }
            }
        }
    }

    // Create VM and runtime
    use crate::vm::{RIFT_API_JS, rift_runtime};
    use deno_core::{JsRuntime, RuntimeOptions};

    let mut runtime: JsRuntime = JsRuntime::new(RuntimeOptions {
        extensions: vec![rift_runtime::init()],
        ..Default::default()
    });

    // Initialize console
    runtime.execute_script(
        "<init_console>",
        "globalThis.console = globalThis.console ?? { log: (..._args) => {}, warn: (..._args) => {}, error: (..._args) => {} };",
    )?;

    // Inject Rift API
    runtime.execute_script("<rift_api>", RIFT_API_JS)?;

    // Load all plugins to register their handlers
    use deno_ast::{
        EmitOptions, MediaType, ParseParams, SourceMapOption, TranspileModuleOptions,
        TranspileOptions,
    };
    for (path_str, ts_content) in plugin_scripts {
        // Convert path string to PathBuf
        let path = std::path::PathBuf::from(&path_str);

        // Create module specifier from file path
        let specifier = deno_ast::ModuleSpecifier::from_file_path(&path)
            .map_err(|_| anyhow::anyhow!("Invalid module path: {}", path_str))?;

        // Parse the TypeScript
        let parsed = deno_ast::parse_module(ParseParams {
            specifier,
            text: ts_content.into(),
            media_type: MediaType::TypeScript,
            capture_tokens: true,
            scope_analysis: false,
            maybe_syntax: None,
        })
        .map_err(|e| anyhow::anyhow!("Failed to parse plugin: {}", e))?;

        // Transpile to JavaScript
        let emitted = parsed
            .transpile(
                &TranspileOptions::default(),
                &TranspileModuleOptions::default(),
                &EmitOptions {
                    source_map: SourceMapOption::None,
                    ..Default::default()
                },
            )
            .map_err(|e| anyhow::anyhow!("Failed to transpile plugin: {}", e))?;
        let js_code = emitted.into_source().text;

        // Execute the plugin to register handlers
        let _ = runtime.execute_script(path_str, js_code);
    }

    // Build config JSON from script_results (already executed)
    let mut config_entries: Vec<String> = Vec::new();
    for (pkg_name, script_result) in script_results {
        let mut pkg_config: Vec<String> = Vec::new();
        for (key, value) in &script_result.config {
            let value_str = match value {
                workspace::executor::ConfigValue::String(s) => {
                    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
                }
                workspace::executor::ConfigValue::Number(n) => n.to_string(),
                workspace::executor::ConfigValue::Boolean(b) => b.to_string(),
            };
            pkg_config.push(format!("\"{}\": {}", key, value_str));
        }
        if !pkg_config.is_empty() {
            config_entries.push(format!("\"{}\": {{{}}}", pkg_name, pkg_config.join(", ")));
        }
    }
    let config_json = if config_entries.is_empty() {
        "{}".to_string()
    } else {
        format!("{{{}}}", config_entries.join(", "))
    };

    // Build packages JSON for the context
    let packages_json = all_packages
        .iter()
        .map(|(k, v)| {
            // Escape backslashes for JavaScript string literals
            let path_str = v.display().to_string().replace('\\', "\\\\");
            format!("['{}', '{}']", k, path_str)
        })
        .collect::<Vec<_>>()
        .join(", ");

    // Create execution script that calls the plugin command handler
    let execute_script = format!(
        r#"
// Set up context
globalThis.Rift = {{
    package: {{ name: "{}", path: "{}" }},
    parent: undefined,
    root: {{ name: "{}", path: "{}" }},
    find: (name) => {{
        return _rift_packages.get(name);
    }}
}};
globalThis._rift_packages = new Map([{}]);
globalThis._rift_config = {{}};

// Set up configuration from script_results
try {{
    const rawConfig = JSON.parse('{}');
    globalThis._rift_config = rawConfig;
}} catch(e) {{
    // Ignore if no config
}}

// Get the command handler
const handler = globalThis._rift_command_handlers && globalThis._rift_command_handlers['{}'];
if (handler && typeof handler === 'function') {{
    const ctx = {{
        workspaceRoot: "{}",
        packageName: "{}",
        packagePath: "{}",
        packages: globalThis._rift_packages,
        config: globalThis._rift_config
    }};
    handler(ctx);
}} else {{
    console.error("Command handler not found for: '{}'");
}}
"#,
        "root",
        root.display(),
        root_name,
        root_path.display(),
        packages_json,
        config_json,
        command_name,
        root_path.display(),
        root_name,
        root.display(),
        command_name
    );

    // Execute the command script
    runtime.execute_script("<command>", execute_script)?;

    println!();
    println!("Command execution complete.");
    Ok(())
}

fn print_package(pkg: &workspace::package::MaybePackage, indent: usize) {
    let indent_str = "  ".repeat(indent);
    let (pkg_type, name, details) = match pkg {
        workspace::package::MaybePackage::Workspace(p) => (
            "[workspace]",
            &p.name,
            Some(format!(
                "members: {}, exclude: {:?}",
                p.members.join(", "),
                p.exclude
            )),
        ),
        workspace::package::MaybePackage::Folder(p) => (
            "[folder]",
            &p.name,
            Some(format!("members: {}", p.members.join(", "))),
        ),
        workspace::package::MaybePackage::Project(p) => (
            "[project]",
            &p.name,
            Some(format!("version: {}", p.version)),
        ),
        workspace::package::MaybePackage::Target(p) => {
            ("[target]", &p.name, Some(format!("type: {}", p.version)))
        }
        workspace::package::MaybePackage::Plugin(p) => {
            ("[plugin]", &p.name, Some(format!("version: {}", p.version)))
        }
    };

    if let Some(details) = details {
        println!("{}{} {} ({})", indent_str, pkg_type, name, details);
    } else {
        println!("{}{} {}", indent_str, pkg_type, name);
    }
}
