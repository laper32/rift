use deno_core::{
    error::AnyError,
    op2,
    v8::{self},
};

use crate::{
    plsys::{PluginManager, PluginStatus},
    task::TaskManager,
    workspace::WorkspaceManager,
    Rift,
};

/*
rift.task.add(new rift.TaskDescriptor("task_name").setDescription(""), () => {
    console.log("Hello from task_name");
});
*/

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskDescriptor {
    name: String,
    description: Option<String>,
    export_to_clap: bool,
}

#[op2]
fn op_register_task(
    scope: &mut v8::HandleScope,
    #[serde] descriptor: TaskDescriptor,
    predicate: v8::Local<v8::Function>,
) -> std::result::Result<(), AnyError> {
    let script_path = Rift::instance()
        .get_current_evaluating_script()
        .clone()
        .unwrap();
    match PluginManager::instance().find_plugin_from_script_path(&script_path) {
        Some(plugin) => {
            if !plugin.is_init() {
                return Err(anyhow::anyhow!("Plugin is not initialized"));
            }
            // plugin.status().register_task(descriptor.name.as_str());
            TaskManager::instance().register_task(descriptor.name.as_str());
            let instance = TaskManager::instance()
                .get_task_mut(descriptor.name.as_str())
                .unwrap();
            instance.set_related_package_name(plugin.name());
            match descriptor.description {
                Some(ref description) => {
                    instance.set_description(description.clone());
                }
                None => {}
            }
            if descriptor.export_to_clap {
                instance.mark_as_command();
            }
            instance.register_runtime_fnction(v8::Global::new(scope, predicate));
        }
        None => {}
    }

    match WorkspaceManager::instance().find_package_from_script_path(&script_path) {
        Some(package) => {
            TaskManager::instance().register_task(descriptor.name.as_str());
            let instance = TaskManager::instance()
                .get_task_mut(descriptor.name.as_str())
                .unwrap();
            instance.set_related_package_name(package.name());
            match descriptor.description {
                Some(ref description) => {
                    instance.set_description(description.clone());
                }
                None => {}
            }
            if descriptor.export_to_clap {
                instance.mark_as_command();
            }
            instance.register_runtime_fnction(v8::Global::new(scope, predicate));
        }
        None => {}
    }

    Ok(())
}

deno_core::extension! {
    task,
    ops = [op_register_task]
}

#[cfg(test)]
mod test {
    use deno_core::v8;

    use crate::{
        plsys::PluginManager,
        runtime::{init, ScriptRuntime},
        task::TaskManager,
        util,
        workspace::WorkspaceManager,
    };

    #[test]
    fn test_run_task() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("02_single_target_with_project")
            .join("Rift.toml");
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        match WorkspaceManager::instance().load_packages() {
            Ok(_) => {
                init();
                PluginManager::instance().evaluate_entries();
                PluginManager::instance().activate_instances();
                TaskManager::instance()
                    .tasks
                    .iter()
                    .for_each(|(_, instance)| {
                        let mut scope = ScriptRuntime::instance().js_runtime().handle_scope();
                        let undefined = v8::undefined(&mut scope);
                        let f =
                            v8::Local::new(&mut scope, instance.get_runtime_function().unwrap());
                        f.call(&mut scope, undefined.into(), &[]);
                    });
                println!("{:?}", TaskManager::instance().tasks);
                // PluginManager::instance().deactivate_instances();
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}
