use deno_core::{error::AnyError, extension, op2};

use crate::{schema::TomlPluginManifestDeclarator, Rift};

#[op2]
fn op_add_manifest_plugin(#[serde] content: TomlPluginManifestDeclarator) -> Result<(), AnyError> {
    println!(
        "op_add_manifest_plugin: Evaluation: {:?} => {:?}",
        Rift::instance().get_current_evaluation_script(),
        content
    );
    Ok(())
}

extension! {
    workspace,
    ops = [
        op_add_manifest_plugin
    ],
}
