// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

use clap::ArgAction;

#[allow(dead_code)]

mod engine {
    mod ffi {
        use std::ffi::c_char;

        extern "C" {
            pub fn RiftEngineInit() -> bool;
            pub fn RiftEngineLoad();
            pub fn RiftEngineShutdown();
            pub fn RiftEngineGetTasks() -> *const c_char;
        }
    }
    pub fn init() -> bool {
        unsafe { ffi::RiftEngineInit() }
    }

    pub fn load() {
        unsafe { ffi::RiftEngineLoad() }
    }

    pub fn shutdown() {
        unsafe { ffi::RiftEngineShutdown() }
    }

    pub fn get_tasks() -> String {
        unsafe {
            let c_str = ffi::RiftEngineGetTasks();
            let c_str = std::ffi::CStr::from_ptr(c_str);
            c_str.to_str().unwrap().to_string()
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct CommandedTask {
    name: String,
    about: Option<String>,
    before_help: Option<String>,
    after_help: Option<String>,
    parent: Option<String>,
    sub_tasks: Vec<String>,
    run_tasks: Vec<String>,
    package_name: String,
    args: Vec<CommandedTaskArg>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct CommandedTaskArg {
    name: String,
    short: Option<char>,
    description: Option<String>,
    default: Option<serde_json::Value>,
    conflict_with: Vec<String>,
    heading: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct CommandInstance {
    name: String,
    about: Option<String>,
    before_help: Option<String>,
    after_help: Option<String>,
    sub_tasks: Vec<String>,
    run_tasks: Vec<String>,
    package_name: String,
    args: Vec<CommandArgInstance>,
}

#[derive(Debug, serde::Serialize)]
struct CommandArgInstance {
    name: String,
    short: Option<char>,
    description: Option<String>,
    default: Option<serde_json::Value>,
    conflict_with: Vec<String>,
    heading: Option<String>,
}

struct CommandManager {
    commands: Vec<CommandInstance>,
    commanded_tasks: Vec<CommandedTask>,
}

impl CommandManager {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
            commanded_tasks: Vec::new(),
        }
    }
    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<CommandManager> =
            once_cell::sync::Lazy::new(|| CommandManager::new());
        unsafe { &mut *INSTANCE }
    }
    pub fn collect_commanded_tasks(&mut self) {
        let result = serde_json::from_str::<Vec<CommandedTask>>(&engine::get_tasks());
        match result {
            Ok(tasks) => {
                self.commanded_tasks = tasks;
            }
            Err(_) => {}
        }
    }

    pub fn make_command_instance(&mut self) {
        for task in self.commanded_tasks.iter() {
            let mut cmd = CommandInstance {
                name: task.name.clone(),
                about: task.about.clone(),
                before_help: task.before_help.clone(),
                after_help: task.after_help.clone(),
                sub_tasks: task.sub_tasks.clone(),
                run_tasks: task.run_tasks.clone(),
                package_name: task.package_name.clone(),
                args: Vec::new(),
            };
            task.args.iter().for_each(|arg| {
                let arg_instance = CommandArgInstance {
                    name: arg.name.clone(),
                    short: arg.short.clone(),
                    description: arg.description.clone(),
                    default: arg.default.clone(),
                    conflict_with: arg.conflict_with.clone(),
                    heading: arg.heading.clone(),
                };
                cmd.args.push(arg_instance);
            });
            self.commands.push(cmd);
        }
    }

    /// 消除所有parent的情况。
    /// 如果发现一个task的实体的parent不存在于构造好的指令集，这个task将会被移除。
    pub fn link_command_tree(&mut self) {
        let parent_tasks = self
            .commanded_tasks
            .iter()
            .filter(|t| t.parent.is_some() && t.parent.as_ref().unwrap().len() > 0)
            .collect::<Vec<&CommandedTask>>();
        parent_tasks.iter().for_each(|task| {
            if !CommandManager::instance().has_command(task.parent.as_ref().unwrap()) {
                CommandManager::instance().remove_command(&task.name);
                return;
            }
            match CommandManager::instance().find_command_mut(task.parent.as_ref().unwrap()) {
                Some(parent_cmd) => {
                    parent_cmd.sub_tasks.push(task.name.clone());
                }
                None => { /* Do nothing */ }
            }
        });
    }

    pub fn find_command_mut(&mut self, name: &str) -> Option<&mut CommandInstance> {
        self.commands.iter_mut().find(|c| c.name == name)
    }

    #[allow(dead_code)]
    pub fn find_command(&self, name: &str) -> Option<&CommandInstance> {
        self.commands.iter().find(|c| c.name == name)
    }

    pub fn remove_command(&mut self, name: &str) {
        self.commands.retain(|c| c.name != name);
    }

    pub fn has_command(&self, name: &str) -> bool {
        self.commands.iter().any(|c| c.name == name)
    }

    pub fn to_commands(&self) -> Vec<clap::Command> {
        let mut ret: Vec<clap::Command> = Vec::new();
        self.commands.iter().for_each(|command| {
            let mut cmd = clap::Command::new(command.name.clone());
            if let Some(about) = command.about.clone() {
                cmd = cmd.about(about);
            }
            if let Some(before_help) = command.before_help.clone() {
                cmd = cmd.before_help(before_help);
            }
            if let Some(after_help) = command.after_help.clone() {
                cmd = cmd.after_help(after_help);
            }
            if command.args.len() > 0 {
                let mut args: Vec<clap::Arg> = Vec::new();
                command.args.iter().for_each(|arg| {
                    let mut arg_actual = clap::Arg::new(arg.name.clone()).long(arg.name.clone());
                    if let Some(short) = arg.short {
                        arg_actual = arg_actual.short(short);
                    }
                    if let Some(description) = arg.description.clone() {
                        arg_actual = arg_actual.help(description);
                    }
                    if arg.conflict_with.len() > 0 {
                        arg_actual = arg_actual.conflicts_with_all(&arg.conflict_with);
                    }
                    if let Some(heading) = arg.heading.clone() {
                        arg_actual = arg_actual.help_heading(heading);
                    }

                    if let Some(default) = arg.default.clone() {
                        if default.is_array() {
                            println!("Cannot set default value to array type.");
                            return;
                        }
                        if default.is_object() {
                            println!("Cannot set default value to object type.");
                            return;
                        }
                        arg_actual = arg_actual.default_value(default.to_string());
                    } else {
                        arg_actual = arg_actual.action(ArgAction::SetTrue);
                    }

                    args.push(arg_actual);
                });
                cmd = cmd.args(args);
            }

            ret.push(cmd);
        });
        ret
    }
}

fn main() {
    engine::init();
    CommandManager::instance().collect_commanded_tasks();
    CommandManager::instance().make_command_instance();
    CommandManager::instance().link_command_tree();
    let commands = CommandManager::instance().to_commands();
    let matches = clap::Command::new("rift")
        .about(r#"Rift, a multi-language, cross platform build system."#)
        .version("0.1.0")
        .subcommands(commands)
        .get_matches();
    engine::load();
    // match matches.subcommand() {
    //     Some((name, _)) => {
    //         let command = CommandManager::instance().find_command(name).unwrap();
    //         println!("{:?}", command);
    //     }
    //     None => {
    //         println!("wait what?");
    //     }
    // }

    engine::shutdown();
}
