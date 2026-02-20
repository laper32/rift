use anyhow::{Result, anyhow};
use deno_ast::swc::ast::{ModuleDecl, TsKeywordType, TsKeywordTypeKind};
use deno_ast::swc::common::Span;
use deno_ast::swc::ecma_visit::{Visit, VisitWith};
use deno_ast::{
    EmitOptions, MediaType, ParseParams, ParsedSource, SourceMapOption, TranspileModuleOptions,
    TranspileOptions,
};
use deno_ast::{ModuleItemRef, SourcePos};
use deno_core::{JsRuntime, RuntimeOptions};
use deno_lint::linter::{LintConfig, Linter, LinterOptions};
use deno_lint::rules::get_all_rules;

use std::borrow::Cow;
use std::collections::{BTreeMap, HashSet};
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

pub struct Vm {
    linter: Linter,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            linter: build_strict_linter(),
        }
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

    ensure_ts_only(path)?;

    let code = fs::read_to_string(path)
        .map_err(|error| anyhow!("Cannot read file {}: {}", path.display(), error))?;
    ensure_safe_ts_source(path, &code)?;

    let media_type = MediaType::from_path(path);
    if !matches!(media_type, MediaType::TypeScript) {
        return Err(anyhow!(
            "Only TypeScript files (.ts) are allowed: {}",
            path.display()
        ));
    }

    let specifier = deno_ast::ModuleSpecifier::from_file_path(path)
        .map_err(|_| anyhow!("Invalid module path: {}", path.display()))?;

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
    let all_rule_codes = all_rules
        .iter()
        .map(|rule| Cow::Borrowed(rule.code()))
        .collect::<HashSet<_>>();

    Linter::new(LinterOptions {
        rules: all_rules,
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

    runtime.execute_script(
        "<init_console>",
        "globalThis.console = globalThis.console ?? { log: (..._args) => {}, warn: (..._args) => {}, error: (..._args) => {} };",
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
}
