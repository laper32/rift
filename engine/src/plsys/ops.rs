use std::{cell::RefCell, rc::Rc};

use deno_core::{error::AnyError, extension, op2, v8, OpState};

#[op2(reentrant)]
fn op_plugin_load_listener<'scope>(
    scope: &mut v8::HandleScope<'scope>,
    _isolate: *mut v8::Isolate,
    _op_state: Rc<RefCell<OpState>>,
    invocation: v8::Local<v8::Function>,
) -> std::result::Result<v8::Local<'scope, v8::Value>, AnyError> {
    let undefined = deno_core::v8::undefined(scope);
    invocation.call(scope, undefined.into(), &[]);
    Ok(undefined.into())
}

#[op2(reentrant)]
fn op_plugin_unload_listener<'scope>(
    scope: &mut v8::HandleScope<'scope>,
    _isolate: *mut v8::Isolate,
    _op_state: Rc<RefCell<OpState>>,
    invocation: v8::Local<v8::Function>,
) -> std::result::Result<v8::Local<'scope, v8::Value>, AnyError> {
    let undefined = deno_core::v8::undefined(scope);
    invocation.call(scope, undefined.into(), &[]);
    Ok(undefined.into())
}

#[op2(reentrant)]
fn op_plugin_all_loaded_listener<'scope>(
    scope: &mut v8::HandleScope<'scope>,
    _isolate: *mut v8::Isolate,
    _op_state: Rc<RefCell<OpState>>,
    invocation: v8::Local<v8::Function>,
) -> std::result::Result<v8::Local<'scope, v8::Value>, AnyError> {
    let undefined = deno_core::v8::undefined(scope);
    invocation.call(scope, undefined.into(), &[]);
    Ok(undefined.into())
}

extension! {
    plsys,
    ops = [
        op_plugin_load_listener,
        op_plugin_all_loaded_listener,
        op_plugin_unload_listener,
    ],
}

#[cfg(test)]
mod test {
    use crate::{plsys::PluginManager, runtime::init, util, workspace::WorkspaceManager};

    /*
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
                PluginManager::instance().register_plugin_listeners();
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    } */
}
