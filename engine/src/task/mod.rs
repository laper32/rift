mod ops;

use std::collections::HashMap;

use clap::{Arg, Command};
use deno_core::v8;

use crate::{
    manifest::{AliasManifest, TaskManifest},
    runtime::ScriptRuntime,
    util::errors::RiftResult,
};

pub fn init_ops() -> deno_core::Extension {
    ops::task::init_ops()
}

#[derive(Debug)]
pub struct TaskFunction {
    native_function: Option<fn()>,
    runtime_function: Option<v8::Global<v8::Function>>,
}

impl TaskFunction {
    pub fn new() -> Self {
        Self {
            native_function: None,
            runtime_function: None,
        }
    }

    pub fn is_unique(&self) -> bool {
        self.native_function.is_some() ^ self.runtime_function.is_some()
    }

    fn register_function(&mut self, f: fn()) {
        // 防呆就行了
        if self.runtime_function.is_some() {
            return;
        }
        self.native_function = Some(f);
    }

    fn register_runtime_fnction(&mut self, f: v8::Global<v8::Function>) {
        // 防呆就行了
        if self.native_function.is_some() {
            return;
        }
        self.runtime_function = Some(f);
    }
    fn has_impl(&self) -> bool {
        self.native_function.is_some() || self.runtime_function.is_some()
    }

    pub fn invoke(&self) -> RiftResult<()> {
        if !self.has_impl() {
            anyhow::bail!("Task function is not implemented.");
        } else {
            match &self.native_function {
                Some(f) => {
                    f();
                    return Ok(());
                }
                None => { /* This is possible. */ }
            }
            match &self.runtime_function {
                Some(f) => {
                    let mut scope = ScriptRuntime::instance().js_runtime().handle_scope();
                    let undefined = v8::undefined(&mut scope);
                    let f = v8::Local::new(&mut scope, f);
                    f.call(&mut scope, undefined.into(), &[]);
                }
                None => { /* In fact this is impossible. */ }
            }
            Ok(())
        }
    }
}

// Instance很多会和clap对应，因此，我们需要一个is_command来标记这个Task是否会进入命令行
#[derive(Debug)]
pub struct TaskInstance {
    name: String,
    // 对应的包名，如果没有就是builtin，否则就是对应的包
    related_package_name: Option<String>,
    description: Option<String>,
    task_fn: TaskFunction,

    // 该Task是否会进指令集
    // 默认是关的，你可以选择手动开。
    // 在manifest中其为`is_command`
    export_to_clap: bool,

    // 子任务，这里只记录其名字
    sub_tasks: Vec<String>,

    // 指令集，这里只记录其名字，这里的意思是顺序执行Vec内的命令（没错，Make的核心feature，我们收了）
    instruction_set: Vec<String>,
}

impl TaskInstance {
    pub fn new(name: String) -> Self {
        Self {
            name,
            related_package_name: None,
            description: None,
            task_fn: TaskFunction::new(),
            export_to_clap: false,
            sub_tasks: Vec::new(),
            instruction_set: Vec::new(),
        }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_related_package_name(&mut self, package_name: String) {
        self.related_package_name = Some(package_name);
    }
    pub fn get_related_package_name(&self) -> Option<&String> {
        self.related_package_name.as_ref()
    }

    pub fn register_function(&mut self, f: fn()) {
        self.task_fn.register_function(f);
    }

    pub fn register_runtime_fnction(&mut self, f: v8::Global<v8::Function>) {
        self.task_fn.register_runtime_fnction(f);
    }

    pub fn get_fn(&self) -> &TaskFunction {
        &self.task_fn
    }

    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn is_command(&self) -> bool {
        self.export_to_clap
    }

    pub fn mark_as_command(&mut self) {
        self.export_to_clap = true;
    }

    pub fn get_sub_tasks(&self) -> &Vec<String> {
        &self.sub_tasks
    }
}

struct AliasInstance {
    name: String,
    referred_pkg_name: Option<String>, // 没名字就是内置的alias.
}

impl AliasInstance {
    fn new(name: String) -> Self {
        Self {
            name,
            referred_pkg_name: None,
        }
    }
    fn set_referred_pkg_name(&mut self, pkg_name: String) {
        self.referred_pkg_name = Some(pkg_name);
    }
    fn name(&self) -> &String {
        &self.name
    }
}

pub struct TaskManager {
    tasks: HashMap<
        String, // Task名字，会提供一个是否进入命令行的选项
        TaskInstance,
    >,
    aliases: HashMap<String, AliasInstance>,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            aliases: HashMap::new(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<TaskManager> =
            once_cell::sync::Lazy::new(|| TaskManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn has_task(&self, task_name: &str) -> bool {
        self.tasks.contains_key(task_name)
    }

    pub fn has_alias(&self, alias_name: &str) -> bool {
        self.aliases.contains_key(alias_name)
    }

    pub fn register_task(&mut self, task_name: &str) {
        if self.has_task(task_name) {
            return;
        }
        self.tasks.insert(
            task_name.to_string().clone(),
            TaskInstance::new(task_name.to_string()),
        );
    }

    pub fn get_task(&self, task_name: &str) -> Option<&TaskInstance> {
        self.tasks.get(task_name)
    }

    pub fn get_task_mut(&mut self, task_name: &str) -> Option<&mut TaskInstance> {
        self.tasks.get_mut(task_name)
    }

    pub fn to_commands(&self) -> Vec<Command> {
        let mut commands = Vec::new();
        for (_, task) in self.tasks.iter() {
            if task.is_command() {
                let mut command = Command::new(task.get_name());
                if let Some(description) = task.get_description() {
                    command = command.about(description);
                }
                commands.push(command);
            }
        }
        commands
    }

    pub fn remove_task(&mut self, task_name: &str) {
        self.tasks.remove(task_name);
    }

    pub fn remove_task_from_pkg_name(&mut self, pkg_name: &str) {
        let mut to_remove = Vec::new();
        for (task_name, task) in self.tasks.iter() {
            if let Some(related_pkg_name) = task.get_related_package_name() {
                if related_pkg_name == pkg_name {
                    to_remove.push(task_name.clone());
                }
            }
        }
        for task_name in to_remove {
            self.remove_task(&task_name);
        }
    }

    pub fn load_task_from_manifest(&mut self, manifest: &TaskManifest) {
        for (task_name, task) in manifest {
            if self.has_task(&task_name) {
                continue;
            }
            self.register_task(&task_name);
            let instance = self.get_task_mut(task_name).unwrap();
            if let Some(description) = &task.description {
                instance.set_description(description.clone());
            }
            if task.is_command {
                instance.mark_as_command();
            }
            instance.related_package_name = Some(task.referred_pkg_name.clone());
            let mut args: Vec<Arg> = Vec::new();
            if task.args.is_some() {
                let task_args = task.args.clone().unwrap();
                for arg in task_args {
                    let mut to_insert_arg = Arg::new(&arg.name);
                    to_insert_arg = to_insert_arg.long(arg.name.clone());
                    if arg.short.is_some() {
                        let short = arg.short.clone().unwrap();
                        let short = short.chars().next().unwrap();
                        to_insert_arg = to_insert_arg.short(short);
                    }
                    if arg.description.is_some() {
                        let arg_description = arg.description.clone().unwrap();
                        to_insert_arg = to_insert_arg.help(arg_description);
                    }
                    if arg.heading.is_some() {
                        let heading = arg.heading.clone().unwrap();
                        to_insert_arg = to_insert_arg.help_heading(heading);
                    }
                    if arg.conflict_with.is_some() {
                        let conflict_with = arg.conflict_with.clone().unwrap();
                        for conflict in conflict_with {
                            if task.has_flag(&conflict) {
                                to_insert_arg = to_insert_arg.conflicts_with_all([conflict]);
                            }
                        }
                    }
                    args.push(to_insert_arg);
                }
            }
        }
    }

    pub fn load_alias_from_manifest(&mut self, manifest: &AliasManifest) {
        for (alias_name, alias) in manifest {
            if self.has_alias(&alias_name) {
                continue;
            }
            self.aliases.insert(
                alias_name.clone(),
                AliasInstance {
                    name: alias_name.clone(),
                    referred_pkg_name: Some(alias.referred_pkg_name.clone()),
                },
            );
        }
    }
}

#[cfg(test)]
mod test {
    use deno_core::v8;

    use crate::runtime::ScriptRuntime;

    use super::TaskManager;

    #[test]
    fn test_is_task_fn_unique() {
        let context = ScriptRuntime::instance().js_runtime().main_context();
        let scope = &mut ScriptRuntime::instance().js_runtime().handle_scope();
        let context_local = v8::Local::new(scope, context);
        let global_obj = context_local.global(scope);
        let bootstrap_str = v8::String::new_external_onebyte_static(scope, b"rift").unwrap();
        let bootstrap_ns: v8::Local<v8::Object> = global_obj
            .get(scope, bootstrap_str.into())
            .unwrap()
            .try_into()
            .unwrap();
        let main_runtime_str =
            v8::String::new_external_onebyte_static(scope, b"getHomePath").unwrap();
        let bootstrap_fn = bootstrap_ns.get(scope, main_runtime_str.into()).unwrap();
        let bootstrap_fn = v8::Local::<v8::Function>::try_from(bootstrap_fn).unwrap();
        let global_fn = v8::Global::new(scope, bootstrap_fn);
        println!("{:?}", bootstrap_fn);
        let task_fn = super::TaskFunction {
            native_function: Some(|| {}),
            runtime_function: Some(global_fn),
        };
        assert!(!task_fn.is_unique()); // should be false
    }
    #[test]
    fn test_is_task_fn_unique_rt_unique() {
        let context = ScriptRuntime::instance().js_runtime().main_context();
        let scope = &mut ScriptRuntime::instance().js_runtime().handle_scope();
        let context_local = v8::Local::new(scope, context);
        let global_obj = context_local.global(scope);
        let bootstrap_str = v8::String::new_external_onebyte_static(scope, b"rift").unwrap();
        let bootstrap_ns: v8::Local<v8::Object> = global_obj
            .get(scope, bootstrap_str.into())
            .unwrap()
            .try_into()
            .unwrap();
        let main_runtime_str =
            v8::String::new_external_onebyte_static(scope, b"getHomePath").unwrap();
        let bootstrap_fn = bootstrap_ns.get(scope, main_runtime_str.into()).unwrap();
        let bootstrap_fn = v8::Local::<v8::Function>::try_from(bootstrap_fn).unwrap();
        let global_fn = v8::Global::new(scope, bootstrap_fn);

        let task_fn = super::TaskFunction {
            native_function: None,
            runtime_function: Some(global_fn),
        };
        assert!(task_fn.is_unique()); // should be true
    }

    #[test]
    fn native_task() {
        let task_fn = super::TaskFunction {
            native_function: Some(|| {
                println!("Hello, world!");
            }),
            runtime_function: None,
        };
        assert!(task_fn.is_unique()); // should be true
    }

    #[test]
    fn example_task() {
        TaskManager::instance().register_task("generate");
        let generate = TaskManager::instance().get_task_mut("generate").unwrap();
        generate.register_function(|| {
            println!("Hello, world!");
        });
        generate.get_fn().invoke();
    }

    #[test]
    fn test_load_task_config_file() {}
}
