use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use git::clone_or_update;

mod cli;
mod config;
mod git;
mod index;
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
    let (mut executor, script_results) = execute_scripts(&root, &packages, &manifests)?;

    println!("Found {} package(s) to process", script_results.len());
    println!();

    // Query explicit dependencies from indexes
    query_explicit_dependencies(&root, &script_results)?;

    // Fetch git dependencies
    fetch_git_dependencies(&script_results)?;

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

/// Fetch all git dependencies from resolved script results
fn fetch_git_dependencies(
    script_results: &HashMap<String, workspace::executor::ScriptResult>,
) -> Result<()> {
    use workspace::executor::DependencySource;

    let mut git_deps = Vec::new();

    // Collect all git dependencies
    for (_pkg_name, result) in script_results {
        for dep in &result.dependencies {
            if dep.source == DependencySource::Git {
                if let Some(git_info) = &dep.git {
                    git_deps.push((dep.name.clone(), git_info.url.clone(), git_info.ref_.clone()));
                }
            }
        }
    }

    if git_deps.is_empty() {
        return Ok(());
    }

    println!("Fetching {} git dependenc(ies)...", git_deps.len());
    for (name, url, ref_) in &git_deps {
        println!("  - {} (ref: {:?})", name, ref_.as_deref().unwrap_or("default"));
        match clone_or_update(url, ref_.as_deref()) {
            Ok(path) => {
                println!("    -> Cloned to: {}", path.display());
            }
            Err(e) => {
                println!("    -> Error: {}", e);
            }
        }
    }
    println!();

    Ok(())
}

/// Query explicit dependencies from configured plugin indexes
/// This resolves versions and provides download URLs for packages
fn query_explicit_dependencies(
    root: &Path,
    script_results: &HashMap<String, workspace::executor::ScriptResult>,
) -> Result<()> {
    use workspace::executor::DependencySource;
    use index::query_package;
    use config::load_config_or_default;

    // Load index sources from config
    let config = load_config_or_default(root);
    let all_indexes = config.index.sources;

    if all_indexes.is_empty() {
        println!("No plugin indexes configured in .rift/config.toml, skipping explicit dependency resolution.");
        return Ok(());
    }

    println!("Using {} plugin index(es):", all_indexes.len());
    for index in &all_indexes {
        println!("  - {}", index);
    }
    println!();

    // Collect all explicit dependencies (those with versions)
    let mut explicit_deps: Vec<(String, Option<String>)> = Vec::new();
    for (_pkg_name, result) in script_results {
        for dep in &result.dependencies {
            if dep.source == DependencySource::Explicit {
                if dep.version.is_some() {
                    explicit_deps.push((dep.name.clone(), dep.version.clone()));
                }
            }
        }
    }

    if explicit_deps.is_empty() {
        println!("No explicit dependencies to resolve.");
        return Ok(());
    }

    println!("Querying {} explicit dependenc(ies)...", explicit_deps.len());
    for (name, version) in &explicit_deps {
        let version_str = version.as_ref().map(|v| v.as_str()).unwrap_or("latest");

        // Try each index until we find the package
        let mut found = false;
        for index_url in &all_indexes {
            match query_package(index_url, name, version_str) {
                Ok(entry) => {
                    println!("  - {}@{} found in {}", name, version_str, index_url);
                    if let Some(url) = entry.url {
                        println!("    -> URL: {}", url);
                    }
                    if let Some(cksum) = entry.cksum {
                        println!("    -> Checksum: {}", cksum);
                    }
                    found = true;
                    break;
                }
                Err(_) => {
                    // Try next index
                    continue;
                }
            }
        }

        if !found {
            println!("  - {}@{} NOT FOUND in any index", name, version_str);
        }
    }
    println!();

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
    let mut replace_deps: std::collections::BTreeSet<(String, String)> = std::collections::BTreeSet::new();

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
            DependencySource::Git => {
                // Git dependencies use replace directive with the cloned path
                if let Some(ref git_info) = dep.git {
                    // Get the cache path for this git repo
                    if let Ok(cache_path) = git::repo_cache_dir(&git_info.url) {
                        replace_deps.insert((dep.name.clone(), cache_path.to_string_lossy().to_string()));
                    }
                }
            }
            DependencySource::Path => {
                // Path dependencies use replace directive
                if let Some(path_info) = &dep.path {
                    replace_deps.insert((dep.name.clone(), path_info.path.clone()));
                }
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

    // Write replace statements for git and path dependencies
    if !replace_deps.is_empty() {
        writeln!(file)?;
        if replace_deps.len() == 1 {
            // Single replace - use simple format
            for (name, path) in &replace_deps {
                writeln!(file, "replace {} => {}", name, path)?;
            }
        } else {
            // Multiple replaces - use block format
            writeln!(file, "replace (")?;
            for (name, path) in &replace_deps {
                writeln!(file, "\t{} => {}", name, path)?;
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
    let (mut executor, script_results) = execute_scripts(&root, &packages, &manifests)?;

    // Convert "app build" to "app.project" for lookup
    let lookup_name = task_name.replace(' ', ".");

    // First, check if it's a plugin command
    if is_plugin_command(&task_name) {
        return execute_plugin_command(&root, &task_name, &packages, &manifests, &script_results);
    }

    // Find and execute the task
    let task_manager = executor.task_manager();

    // Get execution order (including all dependencies)
    let execution_order = task_manager.get_execution_order(&lookup_name);

    match execution_order {
        Ok(tasks) => {
            println!();
            println!("Executing {} task(s) in order:", tasks.len());
            for t in &tasks {
                println!("  - {}", t.name);
            }
            println!();

            // Execute each task in order
            for task in tasks {
                println!("Executing task: {}", task.name);
                if !task.description.is_empty() {
                    println!("  Description: {}", task.description);
                }
                println!();

                // Execute the task action
                execute_task_action(&root, &task, &packages, &manifests, &script_results, executor.vm_mut())?;
                println!();
            }

            Ok(())
        }
        Err(e) => {
            // Task not found or has circular dependency
            let task = task_manager.find_task(&lookup_name);
            if task.is_none() {
                // Special case: for development, provide builtin generate as fallback
                if task_name == "generate" {
                    println!("Note: No 'generate' task provided by plugin. Using builtin generate.");
                    println!("      In the future, this should be provided by rift.go plugin.");
                    println!();
                    return cmd_generate(None);
                }

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
            } else {
                Err(e)
            }
        }
    }
}

/// Execute a single task's action
fn execute_task_action(
    root: &PathBuf,
    task: &tasks::Task,
    packages: &HashMap<String, workspace::package::MaybePackage>,
    manifests: &HashMap<String, PathBuf>,
    script_results: &HashMap<String, workspace::ScriptResult>,
    vm: &mut Vm,
) -> Result<()> {
    use tasks::Task;
    use deno_core::{JsRuntime, RuntimeOptions};
    use deno_ast::{EmitOptions, MediaType, ParseParams, SourceMapOption, TranspileModuleOptions, TranspileOptions};

    // Check if task has an action
    if task.action.is_none() {
        println!("  (Task has no action to execute)");
        return Ok(());
    }

    // Check if action is "has_action" marker (action exists in JS)
    let has_action = task.action.as_ref().map(|a| a == "has_action").unwrap_or(false);
    if !has_action {
        println!("  (Task action not available)");
        return Ok(());
    }

    // Find the package that owns this task
    let pkg_path = if let Some(ref path) = task.package_path {
        Some(PathBuf::from(path))
    } else {
        // Try to find by package name
        manifests.get(&task.package_name).and_then(|m| m.parent()).map(|p| p.to_path_buf())
    };

    // Find and load the tasks script for this package to register task actions
    let tasks_script = if let Some(manifest_path) = manifests.get(&task.package_name) {
        let tasks_path = manifest_path.parent().map(|p| p.join("tasks.ts"));
        if let Some(path) = tasks_path {
            if path.exists() {
                Some(std::fs::read_to_string(&path)?)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    if tasks_script.is_none() {
        println!("  (No tasks script found)");
        return Ok(());
    }

    // Build all_packages map
    let all_packages: HashMap<String, PathBuf> = packages
        .iter()
        .filter_map(|(name, _)| {
            manifests.get(name).and_then(|p| p.parent()).map(|p| (name.clone(), p.to_path_buf()))
        })
        .collect();

    // Build config JSON from script_results
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

    // Use VM's runtime (which already has the exports from plugin loading)
    use crate::vm::RIFT_API_JS;
    let mut runtime = vm.get_or_create_runtime()?;

    // Initialize console (if not already initialized)
    let _ = runtime.execute_script(
        "<init_console>",
        "globalThis.console = globalThis.console ?? { log: (..._args) => {}, warn: (..._args) => {}, error: (..._args) => {} };",
    );

    // Inject Rift API (if not already injected)
    let _ = runtime.execute_script("<rift_api>", RIFT_API_JS);

    // Note: The tasks script was already executed during execute_scripts phase,
    // which registered the task actions. We don't need to re-execute it here.
    // Just execute the task action handler directly.

    // Build packages JSON for the context
    let packages_json = all_packages
        .iter()
        .map(|(k, v)| {
            let path_str = v.display().to_string().replacen("\\?\\", "", 2).replace('\\', "\\\\");
            format!("['{}', '{}']", k, path_str)
        })
        .collect::<Vec<_>>()
        .join(", ");

    // Set working directory if task has a package path
    if let Some(ref path) = pkg_path {
        std::env::set_current_dir(path)?;
    }

    // Combine tasks script + context + action execution in ONE script
    let combined_script = format!(
        r#"
// ===== SET CONTEXT =====
globalThis.Rift = {{
    package: {{ name: "{}", path: "{}" }},
    parent: undefined,
    root: {{ name: "root", path: "{}" }},
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

// ===== EXECUTE TASK ACTION =====
const handler = globalThis._rift_task_actions && globalThis._rift_task_actions['{}'];
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
    console.error("Task action handler not found for: '{}");
}}
"#,
        task.package_name,
        pkg_path.as_ref().map(|p| p.display().to_string().replacen("\\?\\", "", 2).replace('\\', "\\\\")).unwrap_or_default(),
        root.display().to_string().replacen("\\?\\", "", 2).replace('\\', "\\\\"),
        packages_json,
        config_json.replace('\\', "\\\\"),
        task.name,
        root.display().to_string().replacen("\\?\\", "", 2).replace('\\', "\\\\"),
        task.package_name,
        pkg_path.as_ref().map(|p| p.display().to_string().replacen("\\?\\", "", 2).replace('\\', "\\\\")).unwrap_or_default(),
        task.name
    );

    // Debug: print the generated script
    eprintln!("--- DEBUG: Generated task script ---");
    for (i, line) in combined_script.lines().enumerate() {
        eprintln!("{}: {}", i + 1, line);
    }
    eprintln!("--- END DEBUG ---");
    eprintln!();

    runtime.execute_script("<execute_task>", combined_script)?;

    Ok(())
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
    // Also collect plugin metadata (name, version) for color context
    let mut plugin_scripts: Vec<(String, String, String, String)> = Vec::new(); // (path, content, name, version)
    for (name, pkg) in packages {
        if matches!(pkg, workspace::package::MaybePackage::Plugin(_)) {
            if let Some(manifest_path) = manifests.get(name) {
                let index_path = manifest_path.parent().map(|p| p.join("index.ts"));
                if let Some(path) = index_path {
                    if path.exists() {
                        // Get plugin version for color
                        let plugin_version = if let workspace::package::MaybePackage::Plugin(p) = pkg {
                            p.version.clone()
                        } else {
                            "0.0.0".to_string()
                        };

                        // Read the TypeScript file
                        let ts_content = std::fs::read_to_string(&path)?;
                        plugin_scripts.push((path.display().to_string(), ts_content, name.clone(), plugin_version));
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
    for (path_str, ts_content, plugin_name, plugin_version) in plugin_scripts {
        // Set color context before loading the plugin
        let color = format!("{}@{}:{}", plugin_name, plugin_version, plugin_name);
        let color_script = format!("globalThis._rift_current_color = '{}';", color);
        let _ = runtime.execute_script("<set_color>", color_script);

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

// Get the command handler (now stored with color)
const cmdData = globalThis._rift_command_handlers && globalThis._rift_command_handlers['{}'];
const handler = cmdData && cmdData.handler;
if (handler && typeof handler === 'function') {{
    // Set the color context before executing the handler
    // This ensures events emitted during command execution have the correct color
    const originalColor = globalThis._rift_current_color;
    globalThis._rift_current_color = cmdData.color || 'unknown:unknown:unknown';

    const ctx = {{
        workspaceRoot: "{}",
        packageName: "{}",
        packagePath: "{}",
        packages: globalThis._rift_packages,
        config: globalThis._rift_config
    }};
    console.log("Executing command handler (color: " + (cmdData.color || "unknown") + ")");
    handler(ctx);

    // Restore the original color
    globalThis._rift_current_color = originalColor;
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
