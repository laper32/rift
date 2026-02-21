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
    pub dependencies: Vec<String>,
    /// Whether this task should be exposed as a CLI command
    pub is_command: bool,
    /// Action code to execute (JavaScript function body)
    pub action: Option<String>,
    /// Package that owns this task (for execution in correct directory)
    pub package_name: String,
    /// Path to the package manifest (for determining working directory)
    pub package_path: Option<String>,
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
        }
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
}
