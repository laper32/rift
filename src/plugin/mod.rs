//! Plugin event bus system
//!
//! Provides event-driven plugin architecture:
//! - Plugins can define commands (e.g., "build", "generate")
//! - Plugins can emit events (e.g., "onBuild", "onGenerate")
//! - Plugins can subscribe to events with handlers
//!
//! Example flow:
//! 1. rift.generate plugin defines "generate" command and emits "onGenerate" event
//! 2. rift.go plugin subscribes to "onGenerate" and provides handler
//! 3. User runs "rift generate" → event emitted → all handlers called

use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Event handler function type
/// Handlers receive event name and context (serialized JSON)
pub type EventHandler = Box<dyn Fn(&str, &str) -> Result<()> + Send + Sync>;

/// Command handler function type
/// Command handlers receive the command context (package, dependencies, etc.)
pub type CommandHandler = Box<dyn Fn(&CommandContext) -> Result<()> + Send + Sync>;

/// Context passed to command handlers
#[derive(Debug, Clone)]
pub struct CommandContext {
    pub workspace_root: String,
    pub package_name: String,
    pub package_path: String,
    pub dependencies: Vec<String>,
}

/// Event bus for plugin communication
pub struct EventBus {
    /// Event name -> List of handlers
    events: RwLock<HashMap<String, Vec<EventHandler>>>,
    /// Command name -> Handler
    commands: RwLock<HashMap<String, CommandHandler>>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        Self {
            events: RwLock::new(HashMap::new()),
            commands: RwLock::new(HashMap::new()),
        }
    }

    /// Subscribe to an event
    pub fn on<F>(&self, event_name: &str, handler: F) -> Result<()>
    where
        F: Fn(&str, &str) -> Result<()> + Send + Sync + 'static,
    {
        let mut events = self.events.write().unwrap();
        events
            .entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(handler));
        Ok(())
    }

    /// Emit an event to all subscribers
    pub fn emit(&self, event_name: &str, context: &str) -> Result<()> {
        let events = self.events.read().unwrap();
        if let Some(handlers) = events.get(event_name) {
            for handler in handlers {
                handler(event_name, context)?;
            }
        }
        Ok(())
    }

    /// Register a command handler
    pub fn define_command<F>(&self, command_name: &str, handler: F) -> Result<()>
    where
        F: Fn(&CommandContext) -> Result<()> + Send + Sync + 'static,
    {
        let mut commands = self.commands.write().unwrap();
        commands.insert(command_name.to_string(), Box::new(handler));
        Ok(())
    }

    /// Execute a command by name
    pub fn execute_command(&self, command_name: &str, context: &CommandContext) -> Result<()> {
        let commands = self.commands.read().unwrap();
        if let Some(handler) = commands.get(command_name) {
            handler(context)?;
            Ok(())
        } else {
            anyhow::bail!("Command not found: {}", command_name);
        }
    }

    /// Check if a command is registered
    pub fn has_command(&self, command_name: &str) -> bool {
        let commands = self.commands.read().unwrap();
        commands.contains_key(command_name)
    }

    /// Get all registered command names
    pub fn get_commands(&self) -> Vec<String> {
        let commands = self.commands.read().unwrap();
        commands.keys().cloned().collect()
    }
}

/// Global event bus instance
static GLOBAL_EVENT_BUS: RwLock<Option<Arc<EventBus>>> = RwLock::new(None);

use std::sync::OnceLock;

/// Global registry of plugin commands (registered from JavaScript)
static PLUGIN_COMMANDS: OnceLock<std::sync::Mutex<std::collections::HashSet<String>>> =
    OnceLock::new();

fn get_plugin_commands_lock() -> &'static std::sync::Mutex<std::collections::HashSet<String>> {
    PLUGIN_COMMANDS.get_or_init(|| std::sync::Mutex::new(std::collections::HashSet::new()))
}

/// Register a plugin command (called from JavaScript op)
pub fn register_plugin_command(name: String) {
    let mut commands = get_plugin_commands_lock().lock().unwrap();
    commands.insert(name);
}

/// Check if a command is a registered plugin command
pub fn is_plugin_command(name: &str) -> bool {
    let commands = get_plugin_commands_lock().lock().unwrap();
    commands.contains(name)
}

/// Get all registered plugin commands
pub fn get_plugin_commands() -> Vec<String> {
    let commands = get_plugin_commands_lock().lock().unwrap();
    commands.iter().cloned().collect()
}

/// Clear all registered plugin commands (for testing)
pub fn clear_plugin_commands() {
    let mut commands = get_plugin_commands_lock().lock().unwrap();
    commands.clear();
}

/// Initialize the global event bus
pub fn init_event_bus() {
    let mut bus = GLOBAL_EVENT_BUS.write().unwrap();
    if bus.is_none() {
        *bus = Some(Arc::new(EventBus::new()));
    }
}

/// Get the global event bus
pub fn event_bus() -> Arc<EventBus> {
    let bus = GLOBAL_EVENT_BUS.read().unwrap();
    bus.as_ref().expect("Event bus not initialized").clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_event_emit() {
        let bus = EventBus::new();
        let called = Arc::new(AtomicBool::new(false));
        let called_clone = called.clone();

        bus.on("test", move |_event, _ctx| {
            called_clone.store(true, Ordering::SeqCst);
            Ok(())
        })
        .unwrap();

        bus.emit("test", "{}").unwrap();
        assert!(called.load(Ordering::SeqCst));
    }

    #[test]
    fn test_command_define_execute() {
        let bus = EventBus::new();
        let executed = Arc::new(AtomicBool::new(false));
        let executed_clone = executed.clone();

        bus.define_command("test", move |_ctx| {
            executed_clone.store(true, Ordering::SeqCst);
            Ok(())
        })
        .unwrap();

        assert!(bus.has_command("test"));

        let ctx = CommandContext {
            workspace_root: "/test".to_string(),
            package_name: "test".to_string(),
            package_path: "/test".to_string(),
            dependencies: vec![],
        };

        bus.execute_command("test", &ctx).unwrap();
        assert!(executed.load(Ordering::SeqCst));
    }

    #[test]
    fn test_command_not_found() {
        let bus = EventBus::new();
        let ctx = CommandContext {
            workspace_root: "/test".to_string(),
            package_name: "test".to_string(),
            package_path: "/test".to_string(),
            dependencies: vec![],
        };

        let result = bus.execute_command("nonexistent", &ctx);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
