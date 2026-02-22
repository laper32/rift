//! Task management system
//!
//! Tasks are registered from TypeScript scripts and can be executed
//! or exposed as CLI commands.

use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A registered task
#[derive(Debug, Clone)]
pub struct Task {
    /// Task name (e.g., "build", "test", "app.build")
    pub name: String,
    /// Task description
    pub description: String,
    /// List of task names this task depends on
    /// Can include cross-scope references like "packageA:build"
    pub dependencies: Vec<String>,
    /// Whether this task should be exposed as a CLI command
    pub is_command: bool,
    /// Action code to execute (JavaScript function body)
    pub action: Option<String>,
    /// Package that owns this task (for execution in correct directory)
    pub package_name: String,
    /// Path to the package manifest (for determining working directory)
    pub package_path: Option<String>,
    /// Color context for this task (plugin@version:scope)
    /// Used for isolating different versions of the same plugin
    pub color: Option<String>,
    /// Scope (package name) where this task is defined
    pub scope: String,
}

impl Task {
    /// Create a new task
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: String::new(),
            dependencies: Vec::new(),
            is_command: false,
            action: None,
            package_name: String::new(),
            package_path: None,
            color: None,
            scope: String::new(),
        }
    }

    /// Set the color context for this task
    pub fn with_color(mut self, color: String) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the scope for this task
    pub fn with_scope(mut self, scope: String) -> Self {
        self.scope = scope;
        self
    }

    /// Set the package that owns this task
    pub fn with_package(mut self, package_name: String, package_path: Option<String>) -> Self {
        self.package_name = package_name;
        self.package_path = package_path;
        self
    }

    /// Set the task description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Add a dependency
    pub fn with_dependency(mut self, dep: String) -> Self {
        self.dependencies.push(dep);
        self
    }

    /// Mark as a command
    pub fn with_command(mut self, is_command: bool) -> Self {
        self.is_command = is_command;
        self
    }

    /// Set the action code
    pub fn with_action(mut self, action: String) -> Self {
        self.action = Some(action);
        self
    }
}

/// Task manager for registering and executing tasks
#[derive(Debug, Clone)]
pub struct TaskManager {
    tasks: Arc<RwLock<HashMap<String, Task>>>,
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskManager {
    /// Create a new task manager
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register or update a task
    pub fn register_task(&self, name: String, task: Task) -> Result<()> {
        let mut tasks = self
            .tasks
            .write()
            .map_err(|e| anyhow!("Failed to acquire write lock: {}", e))?;
        tasks.insert(name.clone(), task);
        Ok(())
    }

    /// Find a task by name
    pub fn find_task(&self, name: &str) -> Option<Task> {
        let tasks = self.tasks.read().ok()?;
        tasks.get(name).cloned()
    }

    /// Check if a task exists
    pub fn has_task(&self, name: &str) -> bool {
        self.tasks
            .read()
            .ok()
            .and_then(|tasks| Some(tasks.contains_key(name)))
            .unwrap_or(false)
    }

    /// Get all tasks marked as commands
    pub fn get_command_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.read().ok();
        match tasks {
            Some(tasks) => tasks.values().filter(|t| t.is_command).cloned().collect(),
            None => Vec::new(),
        }
    }

    /// Get all registered tasks
    pub fn get_all_tasks(&self) -> Vec<Task> {
        let tasks = self.tasks.read().ok();
        match tasks {
            Some(tasks) => tasks.values().cloned().collect(),
            None => Vec::new(),
        }
    }

    /// Parse a task reference into (scope, task_name) components
    /// Supports:
    /// - "taskname" -> (None, "taskname") - local scope
    /// - "package:taskname" -> (Some("package"), "taskname") - cross-scope
    fn parse_task_reference(reference: &str) -> (Option<String>, String) {
        if let Some((scope, task_name)) = reference.split_once(':') {
            (Some(scope.to_string()), task_name.to_string())
        } else {
            (None, reference.to_string())
        }
    }

    /// Find a task by name, optionally in a specific scope
    /// If scope is None, searches all scopes and returns the first match
    pub fn find_task_in_scope(&self, task_name: &str, scope: Option<&str>) -> Option<Task> {
        let tasks = self.tasks.read().ok()?;
        if let Some(target_scope) = scope {
            // Search in specific scope
            tasks.values().find(|t| t.name == task_name && t.scope == target_scope).cloned()
        } else {
            // Search all scopes, return first match
            tasks.values().find(|t| t.name == task_name).cloned()
        }
    }
    /// Get tasks in execution order (topological sort based on dependencies)
    /// Supports cross-scope task references using "package:task" syntax
    pub fn get_execution_order(&self, task_name: &str) -> Result<Vec<Task>> {
        let tasks = self.tasks.read()
            .map_err(|e| anyhow!("Failed to acquire read lock: {}", e))?;

        // Collect all tasks that need to be executed
        let mut to_execute: Vec<String> = Vec::new();
        let mut visited: std::collections::HashSet<String> = std::collections::HashSet::new();

        // Recursive DFS to collect dependencies
        fn collect_deps(
            task_name: &str,
            tasks: &HashMap<String, Task>,
            to_execute: &mut Vec<String>,
            visited: &mut std::collections::HashSet<String>,
            visiting: &mut std::collections::HashSet<String>
        ) -> Result<()> {
            if visited.contains(task_name) {
                return Ok(());
            }

            if visiting.contains(task_name) {
                return Err(anyhow!("Circular dependency detected involving task: {}", task_name));
            }

            visiting.insert(task_name.to_string());

            if let Some(task) = tasks.get(task_name) {
                for dep in &task.dependencies {
                    // Parse dependency reference (e.g., "packageA:build" or just "build")
                    let (scope, dep_task_name) = TaskManager::parse_task_reference(dep);

                    // Resolve the actual task name to use
                    // If scope is specified, look for task in that scope
                    let resolved_task_name = if let Some(target_scope) = scope {
                        // Find task in specific scope
                        let found = tasks.values().find(|t| t.name == dep_task_name && t.scope == target_scope);
                        if let Some(found_task) = found {
                            found_task.name.clone()
                        } else {
                            return Err(anyhow!(
                                "Cross-scope dependency '{}' not found in scope '{}'",
                                dep_task_name,
                                target_scope
                            ));
                        }
                    } else {
                        // No scope specified - use the task name as-is
                        // This will look up the task in the global registry
                        dep_task_name.clone()
                    };

                    collect_deps(&resolved_task_name, tasks, to_execute, visited, visiting)?;
                }
            }

            visiting.remove(task_name);
            visited.insert(task_name.to_string());
            to_execute.push(task_name.to_string());

            Ok(())
        }

        let mut visiting: std::collections::HashSet<String> = std::collections::HashSet::new();
        collect_deps(task_name, &tasks, &mut to_execute, &mut visited, &mut visiting)?;

        // Convert task names to Task objects
        let mut result: Vec<Task> = Vec::new();
        for name in to_execute {
            if let Some(task) = tasks.get(&name) {
                result.push(task.clone());
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_find_task() {
        let manager = TaskManager::new();

        let task = Task::new("build".to_string())
            .with_description("Build the project".to_string())
            .with_command(true);

        manager.register_task("build".to_string(), task).unwrap();

        assert!(manager.has_task("build"));
        let found = manager.find_task("build").unwrap();
        assert_eq!(found.name, "build");
        assert_eq!(found.description, "Build the project");
        assert!(found.is_command);
    }

    #[test]
    fn test_get_command_tasks() {
        let manager = TaskManager::new();

        let build_task = Task::new("build".to_string()).with_command(true);
        let test_task = Task::new("test".to_string()).with_command(true);
        let internal_task = Task::new("internal".to_string()).with_command(false);

        manager
            .register_task("build".to_string(), build_task)
            .unwrap();
        manager
            .register_task("test".to_string(), test_task)
            .unwrap();
        manager
            .register_task("internal".to_string(), internal_task)
            .unwrap();

        let commands = manager.get_command_tasks();
        assert_eq!(commands.len(), 2);
        assert!(commands.iter().any(|t| t.name == "build"));
        assert!(commands.iter().any(|t| t.name == "test"));
        assert!(!commands.iter().any(|t| t.name == "internal"));
    }

    #[test]
    fn test_parse_task_reference() {
        // Local scope
        let (scope, task) = TaskManager::parse_task_reference("build");
        assert!(scope.is_none());
        assert_eq!(task, "build");

        // Cross-scope
        let (scope, task) = TaskManager::parse_task_reference("packageA:build");
        assert_eq!(scope, Some("packageA".to_string()));
        assert_eq!(task, "build");

        // Cross-scope with dots in name
        let (scope, task) = TaskManager::parse_task_reference("my.package:test");
        assert_eq!(scope, Some("my.package".to_string()));
        assert_eq!(task, "test");
    }

    #[test]
    fn test_cross_scope_task_dependencies() {
        let manager = TaskManager::new();

        // Register tasks in different scopes
        let package_a_build = Task::new("build".to_string())
            .with_description("Build package A".to_string())
            .with_scope("packageA".to_string())
            .with_color("unknown@unknown:packageA".to_string());

        let package_b_test = Task::new("test".to_string())
            .with_description("Test package B".to_string())
            .with_scope("packageB".to_string())
            .with_color("unknown@unknown:packageB".to_string())
            .with_dependency("packageA:build".to_string());

        manager.register_task("build".to_string(), package_a_build).unwrap();
        manager.register_task("test".to_string(), package_b_test).unwrap();

        // Get execution order for packageB's test task
        // Should execute packageA:build first, then packageB:test
        let order = manager.get_execution_order("test").unwrap();
        assert_eq!(order.len(), 2);
        assert_eq!(order[0].name, "build");
        assert_eq!(order[0].scope, "packageA");
        assert_eq!(order[1].name, "test");
        assert_eq!(order[1].scope, "packageB");
    }

    #[test]
    fn test_cross_scope_dependency_not_found() {
        let manager = TaskManager::new();

        let task = Task::new("test".to_string())
            .with_description("Test".to_string())
            .with_scope("packageB".to_string())
            .with_color("unknown@unknown:packageB".to_string())
            .with_dependency("nonexistent:build".to_string());

        manager.register_task("test".to_string(), task).unwrap();

        // Should fail because the cross-scope dependency doesn't exist
        let result = manager.get_execution_order("test");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found in scope"));
    }
}
