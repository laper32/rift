use deno_core::{error::AnyError, extension, op2};

use crate::{
    runtime::collect_all_workspace_scripts,
    schema::TomlPluginManifestDeclarator,
    workspace::plugin_manager::{ManifestPluginIdentifer, PluginManager},
    Rift,
};

#[op2]
fn op_add_manifest_plugin(#[serde] content: TomlPluginManifestDeclarator) -> Result<(), AnyError> {
    let current_evaluation_script = Rift::instance().get_current_evaluation_script();
    let scripts = collect_all_workspace_scripts();
    let current_evaluation_related_manifest = scripts.iter().find(|script| {
        let to_check = script.path.as_ref();
        if to_check.is_none() {
            return false;
        }
        let to_check = to_check.unwrap();
        let is_eq = current_evaluation_script.eq(to_check);
        is_eq
    });
    match current_evaluation_related_manifest {
        Some(m) => {
            PluginManager::instance().register_manifest_plugin(
                m.pkg_name.clone(),
                ManifestPluginIdentifer {
                    name: content.name.clone(),
                    version: content.version.clone(),
                },
            );
        }
        None => {}
    }

    Ok(())
}

#[op2(fast)]
fn op_add_manifest_metadata() -> Result<(), AnyError> {
    Ok(())
}

#[op2(fast)]
fn op_add_manifest_dependencies() -> Result<(), AnyError> {
    Ok(())
}

extension! {
    workspace,
    ops = [
        op_add_manifest_plugin,
        op_add_manifest_metadata,
        op_add_manifest_dependencies,
    ],
}
