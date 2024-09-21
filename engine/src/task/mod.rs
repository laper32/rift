//! 鉴于我们现在可以让v8常驻后台，因此，现在Task的所有设计都可以直接推倒重来了。
//!

mod alias;
mod ops;

use std::collections::HashMap;

use clap::Command;
use deno_core::v8;

pub fn init_ops() -> deno_core::Extension {
    ops::task::init_ops()
}

#[derive(Debug)]
struct TaskFunction {
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
        self.native_function = Some(f);
    }

    fn get_native_function(&self) -> Option<&fn()> {
        self.native_function.as_ref()
    }

    fn register_runtime_fnction(&mut self, f: v8::Global<v8::Function>) {
        self.runtime_function = Some(f);
    }

    fn get_runtime_function(&self) -> Option<&v8::Global<v8::Function>> {
        self.runtime_function.as_ref()
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
    export_to_clap: bool, // 该Task是否会进指令集
                          // 默认是关的，你可以选择手动开。
}

impl TaskInstance {
    pub fn new(name: String) -> Self {
        Self {
            name,
            related_package_name: None,
            description: None,
            task_fn: TaskFunction::new(),
            export_to_clap: false,
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

    pub fn get_native_function(&self) -> Option<&fn()> {
        self.task_fn.get_native_function()
    }

    pub fn get_runtime_function(&self) -> Option<&v8::Global<v8::Function>> {
        self.task_fn.get_runtime_function()
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
}

pub struct TaskManager {
    tasks: HashMap<
        String, // Task名字，会提供一个是否进入命令行的选项
        TaskInstance,
    >,
}

impl TaskManager {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
        }
    }
    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<TaskManager> =
            once_cell::sync::Lazy::new(|| TaskManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn register_task(&mut self, task_name: &str) {
        if self.tasks.contains_key(task_name) {
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
        let f = v8::Local::new(scope, task_fn.get_runtime_function().unwrap());

        let undefined = v8::undefined(scope);
        let ret = f.call(scope, undefined.into(), &[]);
        let result = undefined
            .to_string(scope)
            .unwrap()
            .to_rust_string_lossy(scope);
        println!(
            "{}",
            ret.unwrap()
                .to_string(scope)
                .unwrap()
                .to_rust_string_lossy(scope)
        );
        println!("{}", result);
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
        let f = task_fn.get_native_function().unwrap();
        f();
    }

    #[test]
    fn example_task() {
        TaskManager::instance().register_task("generate");
        let generate = TaskManager::instance().get_task_mut("generate").unwrap();
        generate.register_function(|| {
            println!("Hello, world!");
        });
        generate.get_native_function().unwrap()();
    }
}
