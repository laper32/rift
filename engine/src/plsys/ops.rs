use std::{cell::RefCell, rc::Rc, usize};

use deno_core::{error::AnyError, extension, op2, v8, OpState};

use crate::{plsys::PluginManager, Rift};

#[op2]
fn op_register_plugin_load_listener(
    scope: &mut v8::HandleScope,
    on_load_fn: v8::Local<v8::Function>,
) -> std::result::Result<(), AnyError> {
    let script_path = Rift::instance()
        .get_current_evaluating_script()
        .clone()
        .unwrap();
    match PluginManager::instance().find_plugin_from_script_path(&script_path) {
        Some(plugin) => {
            PluginManager::instance()
                .register_instance_load_fn(plugin.name(), v8::Global::new(scope, on_load_fn));
        }
        None => {}
    }
    Ok(())
}

#[op2]
fn op_register_plugin_unload_listener(
    scope: &mut v8::HandleScope,
    on_unload_fn: v8::Local<v8::Function>,
) -> std::result::Result<(), AnyError> {
    let script_path = Rift::instance()
        .get_current_evaluating_script()
        .clone()
        .unwrap();
    match PluginManager::instance().find_plugin_from_script_path(&script_path) {
        Some(plugin) => {
            PluginManager::instance()
                .register_instance_unload_fn(plugin.name(), v8::Global::new(scope, on_unload_fn));
        }
        None => {}
    }
    Ok(())
}

#[op2]
fn op_register_plugin_all_loaded_listener(
    scope: &mut v8::HandleScope,
    on_all_loaded_fn: v8::Local<v8::Function>,
) -> std::result::Result<(), AnyError> {
    let script_path = Rift::instance()
        .get_current_evaluating_script()
        .clone()
        .unwrap();
    match PluginManager::instance().find_plugin_from_script_path(&script_path) {
        Some(plugin) => {
            PluginManager::instance().register_instance_all_loaded_fn(
                plugin.name(),
                v8::Global::new(scope, on_all_loaded_fn),
            );
        }
        None => {}
    }
    Ok(())
}

extension! {
    plsys,
    ops = [
        op_register_plugin_load_listener,
        op_register_plugin_all_loaded_listener,
        op_register_plugin_unload_listener,
    ],
}

#[cfg(test)]
mod test {
    use crate::{plsys::PluginManager, runtime::init, util, workspace::WorkspaceManager};

    #[test]
    fn test_load_plugins() {
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
                PluginManager::instance().load_instances();
                PluginManager::instance().on_instances_all_loaded();
                PluginManager::instance().unload_instances();
                // PluginManager::instance().register_plugin_listeners();
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}
