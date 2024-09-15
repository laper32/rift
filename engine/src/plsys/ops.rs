use std::{cell::RefCell, rc::Rc};

use deno_core::{error::AnyError, extension, op2, v8, OpState};

#[op2(reentrant)]
fn op_on_plugin_load<'scope>(
    scope: &mut v8::HandleScope<'scope>,
    _isolate: *mut v8::Isolate,
    _op_state: Rc<RefCell<OpState>>,
    invocation: v8::Local<'scope, v8::Function>,
) -> std::result::Result<v8::Local<'scope, v8::Value>, AnyError> {
    let null = deno_core::v8::null(scope);
    let undefined = deno_core::v8::undefined(scope);
    invocation.call(scope, undefined.into(), &[]);
    Ok(null.into())
}

#[op2(reentrant)]
fn op_on_plugin_unload<'scope>(
    scope: &mut v8::HandleScope<'scope>,
    _isolate: *mut v8::Isolate,
    _op_state: Rc<RefCell<OpState>>,
    invocation: v8::Local<'scope, v8::Function>,
) -> std::result::Result<v8::Local<'scope, v8::Value>, AnyError> {
    let null = deno_core::v8::null(scope);
    let undefined = deno_core::v8::undefined(scope);
    invocation.call(scope, undefined.into(), &[]);
    Ok(null.into())
}

#[op2(reentrant)]
fn op_on_plugin_all_loaded<'scope>(
    scope: &mut v8::HandleScope<'scope>,
    _isolate: *mut v8::Isolate,
    _op_state: Rc<RefCell<OpState>>,
    invocation: v8::Local<'scope, v8::Function>,
) -> std::result::Result<v8::Local<'scope, v8::Value>, AnyError> {
    let null = deno_core::v8::null(scope);
    let undefined = deno_core::v8::undefined(scope);
    invocation.call(scope, undefined.into(), &[]);
    Ok(null.into())
}

extension! {
    plsys,
    ops = [
        op_on_plugin_load,
        op_on_plugin_all_loaded,
        op_on_plugin_unload,
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
                PluginManager::instance().activate_plugins();
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
}
