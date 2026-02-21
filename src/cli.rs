//! CLI command parsing with dynamic task support
//!
//! This module provides a hybrid command parser that:
//! 1. First tries to match arguments against registered tasks
//! 2. Falls back to standard clap subcommand parsing
//!
//! This enables task execution:
//! - `rift app build` - execute app package's build task
//! - `rift build` - execute root build task
//! - `rift tasks` - use subcommand

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use std::path::PathBuf;
use std::sync::RwLock;

/// Global registry of command tasks
/// Populated during workspace scanning
static COMMAND_TASKS: RwLock<Option<Vec<String>>> = RwLock::new(None);

/// Register command tasks globally
/// Called after workspace scanning to populate available task names
pub fn register_command_tasks(task_names: Vec<String>) {
    let mut tasks = COMMAND_TASKS.write().unwrap();
    *tasks = Some(task_names);
}

/// Get all registered command task names
pub fn get_command_tasks() -> Vec<String> {
    let tasks = COMMAND_TASKS.read().unwrap();
    tasks.as_ref().cloned().unwrap_or_default()
}

/// Check if a string matches a registered task
pub fn is_command_task(name: &str) -> bool {
    let tasks = COMMAND_TASKS.read().unwrap();
    tasks
        .as_ref()
        .map(|t| t.contains(&name.to_string()))
        .unwrap_or(false)
}

#[derive(Parser)]
#[command(name = "rift")]
#[command(about = "A build system glue layer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan and display the workspace structure
    Scan {
        /// Path to the workspace root (default: current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
    /// List all registered tasks
    Tasks {
        /// Path to the workspace root (default: current directory)
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

/// Parse CLI arguments with smart task detection
///
/// Supports:
/// - `rift build` - single task name
/// - `rift app build` - project + task (space-separated)
/// - `rift tasks` - subcommand
pub fn parse_args() -> CliAction {
    let args: Vec<String> = std::env::args().collect();

    // Special case: no arguments or only program name
    if args.len() <= 1 {
        // Show help
        Cli::command().print_help().ok();
        std::process::exit(0);
    }

    let first = &args[1];

    // Check if first arg is a known subcommand
    if is_subcommand(first) {
        let cli = Cli::parse();
        match cli.command {
            Some(Commands::Scan { path }) => CliAction::Scan(path),
            Some(Commands::Tasks { path }) => CliAction::Tasks(path),
            None => {
                Cli::command().print_help().ok();
                std::process::exit(0);
            }
        }
    } else {
        // Not a subcommand - treat as task execution
        // Join remaining args with space for task lookup
        let task_parts: Vec<&str> = args[1..].iter().map(|s| s.as_str()).collect();
        CliAction::ExecuteTask(task_parts.join(" "))
    }
}

/// Check if a string matches a known subcommand
fn is_subcommand(arg: &str) -> bool {
    matches!(
        arg,
        "scan" | "tasks" | "help" | "--help" | "-h"
    )
}

pub enum CliAction {
    /// Execute a task (may be "build" or "app build")
    ExecuteTask(String),
    Scan(Option<PathBuf>),
    Tasks(Option<PathBuf>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subcommand_detection() {
        assert!(is_subcommand("scan"));
        assert!(is_subcommand("tasks"));
        assert!(!is_subcommand("generate"));  // Now a task, not a subcommand
        assert!(!is_subcommand("build"));
        assert!(!is_subcommand("app"));
    }
}
