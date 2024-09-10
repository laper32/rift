use deno_core::{error::AnyError, extension, op2};
use std::collections::HashMap;

use crate::schema::TomlDependencyManifestDeclarator;
use crate::{schema::TomlPluginManifestDeclarator, workspace::WorkspaceManager, Rift};

#[op2]
fn op_add_manifest_plugin(#[serde] content: TomlPluginManifestDeclarator) -> Result<(), AnyError> {
    println!("content: {:?}", content);
    // 这一步是为了知道现在我们在跑哪个脚本文件。
    let current_evaluation_script = Rift::instance().get_current_evaluation_script();
    // 然后，我们枚举所有的插件脚本, a.k.a: `plugins`这个字段。
    // N.B. 插件系统那边还要走一遍这个流程。
    let scripts = WorkspaceManager::instance().collect_all_manifest_plugin_scripts();
    if scripts.is_none() {
        return Ok(());
    }
    let scripts = scripts.unwrap();
    // 然后，找到这个manifest.
    let current_evaluation_related_manifest = scripts.iter().find(|(_, script)| {
        if script.is_none() {
            return false;
        }
        return match script {
            Some(script) => {
                let is_eq = current_evaluation_script.eq(script);
                is_eq
            }
            None => false,
        };
    });
    if current_evaluation_related_manifest.is_none() {
        return Ok(());
    }
    let current_evaluation_related_manifest = current_evaluation_related_manifest.unwrap();
    // 然后把插件信息加到注册表里
    // PluginManager::instance().register_manifest_plugin(
    //     current_evaluation_related_manifest.0.clone(),
    //     ManifestPluginIdentifier {
    //         name: content.name.clone(),
    //         version: content.version.clone(),
    //     },
    // );

    Ok(())
}

type ScriptMetadataMap = HashMap<String, serde_json::Value>;
#[op2]
fn op_add_manifest_metadata(#[serde] metadata: ScriptMetadataMap) -> Result<(), AnyError> {
    println!("metadata: {:?}", metadata);
    // 这一步是为了知道现在我们在跑哪个脚本文件。
    let current_evaluation_script = Rift::instance().get_current_evaluation_script();
    // 然后，我们枚举所有的Metadata脚本, a.k.a: `metadata`这个字段。
    // N.B. 插件系统那边还要走一遍这个流程。
    let scripts = WorkspaceManager::instance().collect_all_manifest_metadata_scripts();
    if scripts.is_none() {
        return Ok(());
    }
    let scripts = scripts.unwrap();
    // 然后，找到这个manifest.
    let current_evaluation_related_manifest = scripts.iter().find(|(_, script)| {
        if script.is_none() {
            return false;
        }
        return match script {
            Some(script) => {
                let is_eq = current_evaluation_script.eq(script);
                is_eq
            }
            None => false,
        };
    });
    if current_evaluation_related_manifest.is_none() {
        return Ok(());
    }

    let current_evaluation_related_manifest = current_evaluation_related_manifest.unwrap();
    let pkg_name = current_evaluation_related_manifest.0.clone();
    WorkspaceManager::instance().set_package_metadata(pkg_name, metadata);

    Ok(())
}

#[op2]
fn op_add_manifest_dependencies(
    #[serde] dependency: TomlDependencyManifestDeclarator,
) -> Result<(), AnyError> {
    // 这一步是为了知道现在我们在跑哪个脚本文件。
    let current_evaluation_script = Rift::instance().get_current_evaluation_script();
    // 然后，我们枚举所有的Metadata脚本, a.k.a: `metadata`这个字段。
    // N.B. 插件系统那边还要走一遍这个流程。
    let scripts = WorkspaceManager::instance().collect_all_manifest_dependency_scripts();
    if scripts.is_none() {
        return Ok(());
    }
    let scripts = scripts.unwrap();
    // 然后，找到这个manifest.
    let current_evaluation_related_manifest = scripts.iter().find(|(_, script)| {
        if script.is_none() {
            return false;
        }
        return match script {
            Some(script) => {
                let is_eq = current_evaluation_script.eq(script);
                is_eq
            }
            None => false,
        };
    });
    if current_evaluation_related_manifest.is_none() {
        return Ok(());
    }

    let current_evaluation_related_manifest = current_evaluation_related_manifest.unwrap();
    WorkspaceManager::instance().add_package_dependency(
        current_evaluation_related_manifest.0.clone(),
        crate::manifest::DependencyManifestDeclarator {
            name: dependency.name.clone(),
            data: dependency.data.clone(),
        },
    );

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
