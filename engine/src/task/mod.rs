// TODO: 从配置文件中加载
// 1. Builtin
// 2. 插件

mod alias;

use std::{collections::HashMap, path::PathBuf};

use alias::AliasManager;
use clap::Arg;
use serde::{Deserialize, Serialize};

use crate::errors::RiftResult;

pub struct Task {
    inner: TaskInner,
}

struct TaskInner {
    name: String,
    manifest: TaskManifest,
    invocation: fn(),
}

pub struct TaskManager {
    tasks: Vec<Task>,
    possible_tasks: HashMap<String, TaskManifest>,
}

type TomlTaskManifestInner = HashMap<
    // section name
    String,
    // specific task
    TomlTask,
>;

#[derive(Debug, Serialize, Deserialize)]
pub struct TomlTaskManifest(TomlTaskManifestInner);

#[derive(Debug, Deserialize, Serialize)]
pub struct TomlTaskFlag {
    name: String,
    short: String,
    conflict_with: String,
    help: String,
    help_heading: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TomlTask {
    about: String,
    flags: Vec<TomlTaskFlag>,
    after_help: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskFlagManifest {
    name: String,
    short: String,
    conflict_with: String,
    help: String,
    help_heading: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskManifest {
    about: String,
    flags: Vec<TaskFlagManifest>,
    after_help: String,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            possible_tasks: HashMap::new(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<TaskManager> =
            once_cell::sync::Lazy::new(|| TaskManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn add_possible_tasks(&mut self, task_manifest_path: &PathBuf) {
        let data = std::fs::read_to_string(task_manifest_path).unwrap();
        self.add_manifest_from_string(data);
    }

    fn add_manifest_from_string(&mut self, data: String) {
        let manifest = toml::from_str::<TomlTaskManifest>(&data);
        match manifest {
            Ok(manifest) => {
                let all_possible_tasks = manifest.0;
                for (task_name, task) in all_possible_tasks {
                    let task_manifest = TaskManifest {
                        about: task.about,
                        flags: task
                            .flags
                            .iter()
                            .map(|flag| TaskFlagManifest {
                                name: flag.name.clone(),
                                short: flag.short.clone(),
                                conflict_with: flag.conflict_with.clone(),
                                help: flag.help.clone(),
                                help_heading: flag.help_heading.clone(),
                            })
                            .collect(),
                        after_help: task.after_help,
                    };
                    self.possible_tasks.insert(task_name, task_manifest);
                }
            }
            Err(e) => println!("Task manifest must be the section-based. {:?}", e),
        }
    }

    pub fn possible_tasks(&self) -> &HashMap<String, TaskManifest> {
        &self.possible_tasks
    }

    pub fn add_task(&mut self, name: String, invocation: fn()) {
        let name = name.to_owned();
        let manifest = self.possible_tasks.get(&name).unwrap().clone();
        self.tasks.push(Task {
            inner: TaskInner {
                name: name.clone(),
                manifest,
                invocation,
            },
        });
        self.possible_tasks.remove(&name);
    }

    pub fn has_task(&self, name: &str) -> bool {
        self.tasks.iter().any(|task| task.inner.name == name)
    }

    pub fn run_task(&self, name: &str) {
        if let Some(task) = self.find_task(name) {
            (task.inner.invocation)();
        }
    }

    fn find_task(&self, name: &str) -> Option<&Task> {
        self.tasks.iter().find(|task| task.inner.name == name)
    }

    pub fn to_command(&self, name: &str) -> RiftResult<clap::Command> {
        match self.find_task(name) {
            Some(task) => {
                let mut args: Vec<Arg> = Vec::new();
                for flag in &task.inner.manifest.flags {
                    let short = if flag.short.is_empty() {
                        None
                    } else {
                        if flag.short.len() > 1 {
                            anyhow::bail!("Short flag must be a single character: {}", flag.short);
                        }
                        let chars: Vec<char> = flag.short.chars().collect();
                        Some(chars[0])
                    };

                    let arg = clap::Arg::new(flag.name.clone())
                        .help(flag.help.clone())
                        .short(short.unwrap())
                        // .short(flag.short.to_owned())
                        .conflicts_with(flag.conflict_with.clone())
                        .help_heading(flag.help_heading.clone());
                    args.push(arg);
                }

                let command = clap::Command::new(name.to_owned())
                    .about(task.inner.manifest.about.to_owned())
                    .args(args)
                    .after_help(task.inner.manifest.after_help.to_owned());
                Ok(command)
            }

            None => anyhow::bail!("Task not found: {}", name),
        }
    }

    pub fn to_commands(&self) -> Vec<clap::Command> {
        let mut ret: Vec<clap::Command> = Vec::new();
        for task in &self.tasks {
            match self.to_command(&task.inner.name) {
                Ok(command) => {
                    ret.push(command);
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    continue;
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod test {
    use crate::util::get_cargo_project_root;

    use super::TaskManager;

    #[test]
    fn test_add_new_task() {
        let path = get_cargo_project_root()
            .unwrap()
            .join("sample")
            .join("07_task")
            .join("Task.toml");
        TaskManager::instance().add_possible_tasks(&path);
        TaskManager::instance().add_task("build".to_owned(), || {
            println!("TaskManager::Instance::AddTask::Invocation (Build)");
        });
        TaskManager::instance().run_task("build");

        // TaskManager::instance().add_task(super::Task {
        //     inner: super::TaskInner {
        //         name: "test".to_owned(),
        //         manifest: super::TaskManifest {
        //             about: "test".to_owned(),
        //             flags: Vec::new(),
        //             after_help: "test".to_owned(),
        //         },
        //         invocation: || {
        //             // !!
        //             println!("TaskManager::Instance::AddTask::Invocation");
        //         },
        //     },
        // });
        // let task = TaskManager::instance().find_task("test").unwrap();
        // (task.inner.invocation)();
        // let cmd = TaskManager::instance().to_command("test").unwrap();
        // println!("{:?}", cmd);
    }
    #[test]
    fn test_load_config() {
        let path = get_cargo_project_root()
            .unwrap()
            .join("sample")
            .join("07_task")
            .join("Task.toml");
        TaskManager::instance().add_possible_tasks(&path);
        TaskManager::instance()
            .possible_tasks()
            .iter()
            .for_each(|kv| {
                println!("{}", serde_json::to_string_pretty(&kv).unwrap());
            });
    }
}
