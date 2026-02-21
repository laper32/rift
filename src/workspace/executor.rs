use anyhow::{Result, anyhow};
use serde_json::json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::vm::{DependencyValue, PluginValue, RawScriptResult, ScriptContext, TaskValue, Vm};
// Re-export ConfigValue for external use
use crate::plugin;
use crate::tasks::{Task, TaskManager};
pub use crate::vm::ConfigValue;
use crate::workspace::package::MaybePackage;

// Include tests from separate file
#[cfg(test)]
#[path = "executor_test.rs"]
mod executor_test;

/// Result of executing a package's scripts
#[derive(Debug, Clone)]
pub struct ScriptResult {
    pub package_name: String,
    pub dependencies: Vec<PackageReference>,
    pub plugins: Vec<PackageReference>,
    pub config: HashMap<String, ConfigValue>,
    pub tasks: Vec<TaskValue>,
}

/// Git source information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitSource {
    /// Git repository URL
    pub url: String,
    /// Branch, tag, or commit hash (optional)
    pub ref_: Option<String>,
}

/// Path source information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathSource {
    /// Filesystem path (relative or absolute)
    pub path: String,
}

/// Dependency source - determines where the version comes from
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencySource {
    /// Explicitly specified version (from `version` field, looked up in registry)
    Explicit,
    /// Reference a package defined in the workspace
    Workspace,
    /// Inherit from parent/folder definitions
    Inherit,
    /// Direct git repository reference
    Git,
    /// Local filesystem path
    Path,
}

impl Default for DependencySource {
    fn default() -> Self {
        Self::Explicit
    }
}

/// A package reference (dependency or plugin)
///
/// Rift is a coordination layer - PackageReference is a simple data carrier
/// that stores package references without interpreting their format.
///
/// Each language plugin (rift.go, rift.ts, etc.) is responsible for parsing
/// and handling its own package format.
///
/// # Examples
/// ```
/// use rift::workspace::executor::{PackageReference, DependencySource, GitSource, PathSource};
/// use std::collections::HashMap;
///
/// // Explicit version (from registry)
/// let dep = PackageReference::new("rift.go", Some("1.0.0".to_string()));
///
/// // Workspace reference
/// let dep = PackageReference::with_source("shared-utils", DependencySource::Workspace);
///
/// // Inherit from parent
/// let dep = PackageReference::with_source("config", DependencySource::Inherit);
///
/// // Git direct reference
/// let dep = PackageReference::with_git(
///     "rift.go",
///     "https://github.com/user/rift-go".to_string(),
///     Some("main".to_string())
/// );
///
/// // Path reference
/// let dep = PackageReference::with_path(
///     "local-pkg",
///     "../local-pkg".to_string()
/// );
///
/// // With attributes for language-specific metadata
/// let mut attrs = HashMap::new();
/// attrs.insert("features".to_string(), serde_json::json!(["full", "macros"]));
/// let dep = PackageReference::with_attributes("tokio", Some("1.28.0".to_string()), attrs);
/// ```
#[derive(Debug, Clone)]
pub struct PackageReference {
    /// Full package name in the format expected by the target plugin
    pub name: String,
    /// Optional version constraint (used when source is Explicit)
    pub version: Option<String>,
    /// Dependency source (defaults to Explicit)
    pub source: DependencySource,
    /// Git source configuration (only used when source is Git)
    pub git: Option<GitSource>,
    /// Path source configuration (only used when source is Path)
    pub path: Option<PathSource>,
    /// Extensible attributes for language-specific metadata
    pub attributes: HashMap<String, serde_json::Value>,
}

impl PackageReference {
    /// Create a new PackageReference (Explicit source by default)
    pub fn new(name: impl Into<String>, version: Option<String>) -> Self {
        Self {
            name: name.into(),
            version,
            source: DependencySource::Explicit,
            git: None,
            path: None,
            attributes: HashMap::new(),
        }
    }

    /// Create a PackageReference from DependencyValue (preserving attributes and source)
    pub fn from_dependency_value(mut dep: crate::vm::DependencyValue) -> Self {
        // Extract source from attributes if present
        let source = if let Some(serde_json::Value::String(s)) = dep.attributes.remove("source") {
            match s.as_str() {
                "workspace" => DependencySource::Workspace,
                "inherit" => DependencySource::Inherit,
                "git" => DependencySource::Git,
                "path" => DependencySource::Path,
                _ => DependencySource::Explicit,
            }
        } else {
            DependencySource::Explicit
        };

        // Extract git info from attributes if present
        let git = if let Some(serde_json::Value::Object(git_obj)) = dep.attributes.remove("git") {
            let url = git_obj.get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let ref_ = git_obj.get("ref")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            if !url.is_empty() {
                Some(GitSource { url, ref_: ref_ })
            } else {
                None
            }
        } else {
            None
        };

        // Extract path info from attributes if present
        let path = if let Some(serde_json::Value::String(p)) = dep.attributes.remove("path") {
            Some(PathSource { path: p })
        } else if let Some(serde_json::Value::Object(path_obj)) = dep.attributes.remove("path") {
            if let Some(p) = path_obj.get("path").and_then(|v| v.as_str()) {
                Some(PathSource { path: p.to_string() })
            } else {
                None
            }
        } else {
            None
        };

        // Check if this is an excluded dependency
        if dep.attributes.contains_key("excluded") {
            // Mark as excluded in attributes for later filtering
            dep.attributes
                .insert("excluded".to_string(), serde_json::json!(true));
        }

        Self {
            name: dep.name,
            version: dep.version,
            source,
            git,
            path,
            attributes: dep.attributes,
        }
    }

    /// Create a new PackageReference with a specific source
    pub fn with_source(name: impl Into<String>, source: DependencySource) -> Self {
        Self {
            name: name.into(),
            version: None,
            source,
            git: None,
            path: None,
            attributes: HashMap::new(),
        }
    }

    /// Create a new PackageReference with Git source
    pub fn with_git(name: impl Into<String>, url: String, ref_: Option<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
            source: DependencySource::Git,
            git: Some(GitSource { url, ref_ }),
            path: None,
            attributes: HashMap::new(),
        }
    }

    /// Create a new PackageReference with Path source
    pub fn with_path(name: impl Into<String>, path: String) -> Self {
        Self {
            name: name.into(),
            version: None,
            source: DependencySource::Path,
            git: None,
            path: Some(PathSource { path }),
            attributes: HashMap::new(),
        }
    }

    /// Create a new PackageReference with attributes
    pub fn with_attributes(
        name: impl Into<String>,
        version: Option<String>,
        attributes: HashMap<String, serde_json::Value>,
    ) -> Self {
        Self {
            name: name.into(),
            version,
            source: DependencySource::Explicit,
            git: None,
            path: None,
            attributes,
        }
    }

    /// Add an attribute to the package reference
    pub fn with_attribute(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.attributes.insert(key.into(), value);
        self
    }

    /// Parse a package reference from a string
    /// Only handles "name@version" format - everything else is passed through
    pub fn parse(input: &str) -> Self {
        if let Some((name, version)) = input.split_once('@') {
            Self::new(name, Some(version.to_string()))
        } else {
            Self::new(input, None)
        }
    }
}

impl From<&str> for PackageReference {
    fn from(s: &str) -> Self {
        Self::parse(s)
    }
}

impl From<String> for PackageReference {
    fn from(s: String) -> Self {
        Self::parse(&s)
    }
}

impl<S: Into<String>> From<(S, Option<String>)> for PackageReference {
    fn from((name, version): (S, Option<String>)) -> Self {
        Self::new(name, version)
    }
}

/// Script executor - runs package scripts and collects results
pub struct ScriptExecutor {
    vm: Vm,
    workspace_root: PathBuf,
    /// Root workspace manifest path
    root_manifest_path: PathBuf,
    /// Root workspace name
    root_name: String,
    /// Task manager for registering tasks from scripts
    task_manager: Arc<TaskManager>,
    /// Stored execution results
    script_results: Option<HashMap<String, ScriptResult>>,
}

impl ScriptExecutor {
    pub fn new(workspace_root: PathBuf) -> Self {
        Self {
            vm: Vm::new(),
            workspace_root: workspace_root.clone(),
            root_manifest_path: workspace_root.join("Rift.toml"),
            root_name: String::new(),
            task_manager: Arc::new(TaskManager::new()),
            script_results: None,
        }
    }

    /// Get the task manager
    pub fn task_manager(&self) -> Arc<TaskManager> {
        self.task_manager.clone()
    }

    /// Get the root workspace name
    pub fn root_name(&self) -> &str {
        &self.root_name
    }

    /// Get all package configurations as a JSON string for JavaScript
    pub fn get_config_json(&self) -> String {
        if let Some(ref results) = self.script_results {
            let mut config_pairs: Vec<String> = Vec::new();
            for (pkg_name, script_result) in results {
                for (key, value) in &script_result.config {
                    let value_str = match value {
                        crate::vm::ConfigValue::String(s) => {
                            format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
                        }
                        crate::vm::ConfigValue::Number(n) => n.to_string(),
                        crate::vm::ConfigValue::Boolean(b) => b.to_string(),
                    };
                    config_pairs.push(format!("\"{}\": {}", key, value_str));
                }
            }
            if config_pairs.is_empty() {
                "{}".to_string()
            } else {
                format!("{{{}}}", config_pairs.join(", "))
            }
        } else {
            "{}".to_string()
        }
    }

    /// Get config per package for JavaScript
    pub fn get_package_configs(&self) -> HashMap<String, HashMap<String, String>> {
        let mut result = HashMap::new();
        if let Some(ref results) = self.script_results {
            for (pkg_name, script_result) in results {
                let mut pkg_config = HashMap::new();
                for (key, value) in &script_result.config {
                    let value_str = match value {
                        crate::vm::ConfigValue::String(s) => s.clone(),
                        crate::vm::ConfigValue::Number(n) => n.to_string(),
                        crate::vm::ConfigValue::Boolean(b) => b.to_string(),
                    };
                    pkg_config.insert(key.clone(), value_str);
                }
                if !pkg_config.is_empty() {
                    result.insert(pkg_name.clone(), pkg_config);
                }
            }
        }
        result
    }

    /// Execute all scripts for the given packages
    /// manifest_paths maps package name to its manifest file path
    /// Returns results indexed by package name
    pub fn execute_all(
        &mut self,
        packages: &HashMap<String, MaybePackage>,
        manifest_paths: &HashMap<String, PathBuf>,
    ) -> Result<HashMap<String, ScriptResult>> {
        // Find root workspace
        for (name, pkg) in packages {
            if matches!(pkg, MaybePackage::Workspace(_)) {
                self.root_name = name.clone();
                if let Some(path) = manifest_paths.get(name) {
                    self.root_manifest_path = path.clone();
                }
                break;
            }
        }

        let root_dir = self
            .root_manifest_path
            .parent()
            .ok_or_else(|| anyhow!("Root manifest has no parent directory"))?
            .to_path_buf();

        let mut results = HashMap::new();

        // First pass: execute workspace scripts
        for (name, pkg) in packages {
            if matches!(pkg, MaybePackage::Workspace(_)) {
                let manifest_path = manifest_paths
                    .get(name)
                    .ok_or_else(|| anyhow!("Manifest path not found for: {}", name))?;
                let result =
                    self.execute_package(pkg, manifest_path, packages, manifest_paths, &root_dir)?;
                results.insert(name.clone(), result);
            }
        }

        // Second pass: execute folder scripts
        for (name, pkg) in packages {
            if matches!(pkg, MaybePackage::Folder(_)) {
                let manifest_path = manifest_paths
                    .get(name)
                    .ok_or_else(|| anyhow!("Manifest path not found for: {}", name))?;
                let result =
                    self.execute_package(pkg, manifest_path, packages, manifest_paths, &root_dir)?;
                results.insert(name.clone(), result);
            }
        }

        // Third pass: execute plugin scripts first (to export their APIs)
        for (name, pkg) in packages {
            if matches!(pkg, MaybePackage::Plugin(_)) {
                let manifest_path = manifest_paths
                    .get(name)
                    .ok_or_else(|| anyhow!("Manifest path not found for: {}", name))?;
                let result =
                    self.execute_package(pkg, manifest_path, packages, manifest_paths, &root_dir)?;
                results.insert(name.clone(), result);
            }
        }

        // Fourth pass: execute project/target scripts (plugins are now loaded)
        for (name, pkg) in packages {
            if matches!(pkg, MaybePackage::Project(_) | MaybePackage::Target(_)) {
                let manifest_path = manifest_paths
                    .get(name)
                    .ok_or_else(|| anyhow!("Manifest path not found for: {}", name))?;
                let result =
                    self.execute_package(pkg, manifest_path, packages, manifest_paths, &root_dir)?;
                results.insert(name.clone(), result);
            }
        }

        // Store results for resolution
        self.script_results = Some(results.clone());

        // Resolve workspace/inherit references
        let resolved_results = self.resolve_dependencies(packages, &results, &self.root_name)?;

        Ok(resolved_results)
    }

    /// Resolve dependency sources (workspace, inherit, exclude)
    fn resolve_dependencies(
        &self,
        packages: &HashMap<String, MaybePackage>,
        results: &HashMap<String, ScriptResult>,
        workspace_name: &str,
    ) -> Result<HashMap<String, ScriptResult>> {
        // Build parent lookup map and collect all package names for validation
        let mut parent_map: HashMap<String, Option<String>> = HashMap::new();
        let mut all_package_names: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        for (name, pkg) in packages {
            parent_map.insert(name.clone(), pkg.parent_name().map(|s| s.to_string()));
            all_package_names.insert(name.clone());
        }

        let mut resolved_results = HashMap::new();

        // For each package, resolve its dependencies
        for (pkg_name, script_result) in results.iter() {
            let mut resolved_deps = Vec::new();

            for dep in &script_result.dependencies {
                // Skip excluded dependencies
                if dep.attributes.contains_key("excluded") {
                    continue;
                }

                match dep.source {
                    DependencySource::Explicit => {
                        // Keep explicit dependencies as-is (to be looked up in registry)
                        resolved_deps.push(dep.clone());
                    }
                    DependencySource::Git => {
                        // Keep git dependencies as-is (direct git reference)
                        resolved_deps.push(dep.clone());
                    }
                    DependencySource::Path => {
                        // Keep path dependencies as-is (language plugins handle path resolution)
                        resolved_deps.push(dep.clone());
                    }
                    DependencySource::Workspace => {
                        // Look up in workspace results first
                        if let Some(workspace_result) = results.get(workspace_name) {
                            // Find the dependency in workspace's dependencies
                            if let Some(workspace_dep) = workspace_result
                                .dependencies
                                .iter()
                                .find(|d| d.name == dep.name)
                            {
                                // Use workspace's dependency definition (including version and attributes)
                                resolved_deps.push(workspace_dep.clone());
                            } else {
                                // Not in workspace dependencies - check if it's a local workspace package
                                if all_package_names.contains(&dep.name) {
                                    // It's a local package reference, keep it as-is
                                    resolved_deps.push(dep.clone());
                                } else {
                                    // Package not found anywhere
                                    return Err(anyhow!(
                                        "Package '{}' is referenced with source: \"workspace\" \
                                        but is not defined in workspace dependencies. \
                                        Please add it to the workspace's dependencies.ts first.",
                                        dep.name
                                    ));
                                }
                            }
                        } else {
                            // Workspace result not found (workspace has no dependencies script)
                            // Check if it's a local workspace package
                            if all_package_names.contains(&dep.name) {
                                resolved_deps.push(dep.clone());
                            } else {
                                return Err(anyhow!(
                                    "Package '{}' is referenced with source: \"workspace\" \
                                    but workspace has no dependencies.ts defined.",
                                    dep.name
                                ));
                            }
                        }
                    }
                    DependencySource::Inherit => {
                        // Look up in parent results
                        if let Some(parent_opt) = parent_map.get(pkg_name) {
                            if let Some(parent_name) = parent_opt {
                                if let Some(parent_result) = results.get(parent_name) {
                                    // Find the dependency in parent's dependencies
                                    if let Some(parent_dep) = parent_result
                                        .dependencies
                                        .iter()
                                        .find(|d| d.name == dep.name)
                                    {
                                        resolved_deps.push(parent_dep.clone());
                                    } else {
                                        // Parent doesn't have this dependency
                                        return Err(anyhow!(
                                            "Package '{}' is referenced with source: \"inherit\" \
                                            but parent '{}' doesn't have this dependency.",
                                            dep.name,
                                            parent_name
                                        ));
                                    }
                                } else {
                                    // Parent result not found
                                    return Err(anyhow!(
                                        "Cannot inherit dependency '{}' from parent '{}': \
                                        parent script results not found.",
                                        dep.name,
                                        parent_name
                                    ));
                                }
                            } else {
                                // No parent for this package
                                return Err(anyhow!(
                                    "Package '{}' has no parent, cannot use source: \"inherit\" for dependency '{}'.",
                                    pkg_name,
                                    dep.name
                                ));
                            }
                        } else {
                            // No parent info available
                            return Err(anyhow!(
                                "Package '{}' parent information not found, cannot use source: \"inherit\".",
                                pkg_name
                            ));
                        }
                    }
                }
            }

            resolved_results.insert(
                pkg_name.clone(),
                ScriptResult {
                    dependencies: resolved_deps,
                    plugins: script_result.plugins.clone(),
                    config: script_result.config.clone(),
                    tasks: script_result.tasks.clone(),
                    package_name: script_result.package_name.clone(),
                },
            );
        }

        Ok(resolved_results)
    }

    /// Execute scripts for a single package
    fn execute_package(
        &mut self,
        pkg: &MaybePackage,
        manifest_path: &Path,
        _packages: &HashMap<String, MaybePackage>,
        manifest_paths: &HashMap<String, PathBuf>,
        root_dir: &Path,
    ) -> Result<ScriptResult> {
        let name = pkg.name();
        let (plugins_opt, configure_opt, dependencies_opt, tasks_opt) = match pkg {
            MaybePackage::Workspace(p) => (
                p.plugins.as_ref(),
                p.configure.as_ref(),
                p.dependencies.as_ref(),
                p.tasks.as_ref(),
            ),
            MaybePackage::Folder(p) => (
                p.plugins.as_ref(),
                p.configure.as_ref(),
                p.dependencies.as_ref(),
                p.tasks.as_ref(),
            ),
            MaybePackage::Project(p) => (
                p.plugins.as_ref(),
                p.configure.as_ref(),
                p.dependencies.as_ref(),
                p.tasks.as_ref(),
            ),
            MaybePackage::Target(p) => (
                p.plugins.as_ref(),
                p.configure.as_ref(),
                p.dependencies.as_ref(),
                p.tasks.as_ref(),
            ),
            MaybePackage::Plugin(p) => (
                None,
                p.configure.as_ref(),
                p.dependencies.as_ref(),
                p.tasks.as_ref(),
            ),
        };

        let manifest_dir = manifest_path
            .parent()
            .ok_or_else(|| anyhow!("Manifest has no parent directory"))?;

        // Build parent information
        let (parent_name, parent_path) = if let Some(p_name) = pkg.parent_name() {
            if let Some(p_manifest) = manifest_paths.get(p_name) {
                let p_dir = p_manifest
                    .parent()
                    .ok_or_else(|| anyhow!("Parent manifest has no parent directory"))?;
                (Some(p_name.to_string()), Some(p_dir.to_path_buf()))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        // Build all_packages map for Rift.find()
        let mut all_packages: HashMap<String, PathBuf> = HashMap::new();
        for (pkg_name, pkg_manifest) in manifest_paths {
            if let Some(dir) = pkg_manifest.parent() {
                all_packages.insert(pkg_name.clone(), dir.to_path_buf());
            }
        }

        // Collect results from all script executions
        let mut all_dependencies = Vec::new();
        let mut all_plugins = Vec::new();
        let mut all_config = HashMap::new();
        let mut all_tasks = Vec::new();

        // Build script context
        let context = ScriptContext {
            package_name: name.to_string(),
            package_path: manifest_dir.to_path_buf(),
            parent_name: parent_name.clone(),
            parent_path: parent_path.clone(),
            root_name: self.root_name.clone(),
            root_path: root_dir.to_path_buf(),
            all_packages: all_packages.clone(),
        };

        // Special handling for plugins: execute index.ts if it exists
        // Plugins use index.ts as entry point (like front-end packages)
        let is_plugin = matches!(pkg, MaybePackage::Plugin(_));
        if is_plugin {
            let index_path = manifest_dir.join("index.ts");
            if index_path.exists() {
                println!("Loading plugin: {} from {}", name, index_path.display());
                let result = self.vm.run_entry_with_context(&index_path, &context)?;

                // Collect commands and event subscriptions from the plugin
                // These are stored in the RiftOpState after executing index.ts
                for cmd in &result.commands {
                    println!("  - Command registered: {}", cmd);
                }
                for event in &result.event_subscriptions {
                    println!("  - Event subscription: {}", event);
                }

                // Collect tasks from plugin
                for task in result.tasks {
                    all_tasks.push(task);
                }

                // Register all collected tasks with the task manager
                for task_value in &all_tasks {
                    let task = Task::new(task_value.name.clone())
                        .with_description(task_value.description.clone())
                        .with_command(task_value.is_command);
                    let task = task_value
                        .dependencies
                        .iter()
                        .fold(task, |t, dep| t.with_dependency(dep.clone()));
                    let _ = self
                        .task_manager
                        .register_task(task_value.name.clone(), task);
                }

                return Ok(ScriptResult {
                    package_name: name.to_string(),
                    dependencies: all_dependencies,
                    plugins: all_plugins,
                    config: all_config,
                    tasks: all_tasks,
                });
            }
        }

        // Execute plugins script
        if let Some(plugins_path) = plugins_opt {
            let full_path = manifest_dir.join(plugins_path);
            if !full_path.exists() {
                return Err(anyhow!("Plugin script not found: {}", full_path.display()));
            }

            let context = ScriptContext {
                package_name: name.to_string(),
                package_path: manifest_dir.to_path_buf(),
                parent_name: parent_name.clone(),
                parent_path: parent_path.clone(),
                root_name: self.root_name.clone(),
                root_path: root_dir.to_path_buf(),
                all_packages: all_packages.clone(),
            };

            let result = self.vm.run_entry_with_context(&full_path, &context)?;
            // Collect plugins from result
            for plugin in result.plugins {
                all_plugins.push(PackageReference::new(plugin.name, plugin.version));
            }
            // Collect tasks from result
            // Collect tasks from result
            for task in result.tasks {
                all_tasks.push(task);
            }
        }

        // Execute dependencies script
        if let Some(deps_path) = dependencies_opt {
            let full_path = manifest_dir.join(deps_path);
            if !full_path.exists() {
                return Err(anyhow!(
                    "Dependencies script not found: {}",
                    full_path.display()
                ));
            }

            let context = ScriptContext {
                package_name: name.to_string(),
                package_path: manifest_dir.to_path_buf(),
                parent_name: parent_name.clone(),
                parent_path: parent_path.clone(),
                root_name: self.root_name.clone(),
                root_path: root_dir.to_path_buf(),
                all_packages: all_packages.clone(),
            };

            let result = self.vm.run_entry_with_context(&full_path, &context)?;
            // Collect dependencies from result
            for dep in result.dependencies {
                all_dependencies.push(PackageReference::from_dependency_value(dep));
            }
            // Collect tasks from result
            for task in result.tasks {
                all_tasks.push(task);
            }
        }

        // Execute configure script
        if let Some(configure_path) = configure_opt {
            let full_path = manifest_dir.join(configure_path);
            if !full_path.exists() {
                return Err(anyhow!(
                    "Configure script not found: {}",
                    full_path.display()
                ));
            }

            let context = ScriptContext {
                package_name: name.to_string(),
                package_path: manifest_dir.to_path_buf(),
                parent_name: parent_name.clone(),
                parent_path: parent_path.clone(),
                root_name: self.root_name.clone(),
                root_path: root_dir.to_path_buf(),
                all_packages: all_packages.clone(),
            };

            let result = self.vm.run_entry_with_context(&full_path, &context)?;
            // Collect config from result
            for (key, value) in result.config {
                all_config.insert(key, value);
            }
            // Collect tasks from result
            for task in result.tasks {
                all_tasks.push(task);
            }
        }

        // Execute tasks script
        if let Some(tasks_path) = tasks_opt {
            let full_path = manifest_dir.join(tasks_path);
            if !full_path.exists() {
                return Err(anyhow!("Tasks script not found: {}", full_path.display()));
            }

            let context = ScriptContext {
                package_name: name.to_string(),
                package_path: manifest_dir.to_path_buf(),
                parent_name: parent_name.clone(),
                parent_path: parent_path.clone(),
                root_name: self.root_name.clone(),
                root_path: root_dir.to_path_buf(),
                all_packages: all_packages.clone(),
            };

            let result = self.vm.run_entry_with_context(&full_path, &context)?;
            // Collect tasks from result
            for task in result.tasks {
                all_tasks.push(task);
            }
        }

        // Register all collected tasks with the task manager
        let package_path_str = manifest_dir.display().to_string();
        for task_value in &all_tasks {
            let task = Task::new(task_value.name.clone())
                .with_description(task_value.description.clone())
                .with_command(task_value.is_command)
                .with_package(name.to_string(), Some(package_path_str.clone()));
            let task = task_value
                .dependencies
                .iter()
                .fold(task, |t, dep| t.with_dependency(dep.clone()));
            // Include action if present
            let task = if let Some(ref action) = task_value.action {
                task.with_action(action.clone())
            } else {
                task
            };
            let _ = self
                .task_manager
                .register_task(task_value.name.clone(), task);
        }

        // Return collected results
        Ok(ScriptResult {
            package_name: name.to_string(),
            dependencies: all_dependencies,
            plugins: all_plugins,
            config: all_config,
            tasks: all_tasks,
        })
    }
}
