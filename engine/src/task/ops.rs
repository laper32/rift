use deno_core::{
    error::AnyError,
    op2,
    v8::{self},
};

use crate::{plsys::PluginManager, task::TaskManager, workspace::WorkspaceManager, Rift};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskDescriptor {
    name: String,
    description: Option<String>,
    export_to_clap: bool,
}

#[op2]
fn op_impl_task(
    scope: &mut v8::HandleScope,
    #[string] task_name: String,
    predicate: v8::Local<v8::Function>,
) -> std::result::Result<(), AnyError> {
    match PluginManager::instance().get_current_plugin_cursor() {
        Some(plugin_name) => {
            let instance = PluginManager::instance()
                .find_plugin_from_name(&plugin_name)
                .expect(format!("Unable to find plugin \"{plugin_name}\"").as_str());
            if !instance.is_init() {
                return Err(anyhow::anyhow!("Plugin is not initialized"));
            }
            let task_instance = TaskManager::instance().get_task_mut(&task_name);
            match task_instance {
                Some(task_instance) => {
                    if !plugin_name.eq(task_instance.related_package_name.clone().unwrap().as_str())
                    {
                        anyhow::bail!("Task \"{task_name}\" references package is not equivalent. Running script's package: \"{}\", Task's package: \"{}\"", plugin_name, task_instance.related_package_name.clone().unwrap());
                    }
                    task_instance.register_runtime_fnction(v8::Global::new(scope, predicate));
                }
                None => anyhow::bail!("Task \"{task_name}\" not found"),
            }
        }
        None => {}
    }

    match WorkspaceManager::instance().find_package_from_script_path(
        &Rift::instance()
            .get_current_evaluating_script()
            .clone()
            .unwrap(),
    ) {
        Some(package) => {
            let instance = TaskManager::instance().get_task_mut(&task_name);
            match instance {
                Some(instance) => {
                    if !package
                        .name()
                        .eq(instance.related_package_name.clone().unwrap().as_str())
                    {
                        anyhow::bail!("Task \"{task_name}\" references package is not equivalent. Running script's package: \"{}\", Task's package: \"{}\"", package.name(), instance.related_package_name.clone().unwrap());
                    }
                    instance.register_runtime_fnction(v8::Global::new(scope, predicate));
                }
                None => anyhow::bail!("Task \"{task_name}\" not found"),
            }
        }
        None => {}
    }
    Ok(())
}

#[op2]
fn op_register_task(
    scope: &mut v8::HandleScope,
    #[serde] descriptor: TaskDescriptor,
    predicate: v8::Local<v8::Function>,
) -> std::result::Result<(), AnyError> {
    // 处理插件系统
    match PluginManager::instance().get_current_plugin_cursor() {
        Some(plugin_name) => {
            let instance = PluginManager::instance()
                .find_plugin_from_name(&plugin_name)
                .expect(format!("Unable to find plugin \"{plugin_name}\"").as_str());
            if !instance.is_init() {
                return Err(anyhow::anyhow!("Plugin is not initialized"));
            }
            TaskManager::instance().register_task(descriptor.name.as_str());
            let instance = TaskManager::instance()
                .get_task_mut(descriptor.name.as_str())
                .unwrap();
            instance.set_related_package_name(instance.get_name().to_string());
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

    // 处理Workspace...
    match WorkspaceManager::instance().find_package_from_script_path(
        &Rift::instance()
            .get_current_evaluating_script()
            .clone()
            .unwrap(),
    ) {
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
    ops = [op_register_task, op_impl_task]
}

#[cfg(test)]
mod test {

    use crate::{
        plsys::PluginManager, runtime::init, task::TaskManager, util, workspace::WorkspaceManager,
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
                TaskManager::instance().tasks.iter().for_each(|(name, _)| {
                    println!("{}", name);
                });

                assert_eq!(TaskManager::instance().tasks.len(), 4);
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}
