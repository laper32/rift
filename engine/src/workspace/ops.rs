use deno_core::{error::AnyError, extension, op2};
use std::collections::HashMap;

use crate::manifest::DependencyManifestDeclarator;
use crate::manifest::PluginManifestDeclarator;
use crate::schema::TomlDependencyManifestDeclarator;
use crate::schema::TomlPluginManifestDeclarator;
use crate::workspace::WorkspaceManager;
use crate::Rift;

#[op2]
fn op_add_manifest_plugin(#[serde] content: TomlPluginManifestDeclarator) -> Result<(), AnyError> {
    match Rift::instance().get_current_evaluating_package() {
        Some(pkg) => {
            // WIP
            println!(
                "op_add_manifest_plugin => current_evaluating_package: {}",
                pkg.manifest().name()
            );
            WorkspaceManager::instance().add_plugin_for_package(
                pkg.manifest().name(),
                PluginManifestDeclarator {
                    name: content.name.clone(),
                    data: content.data.clone(),
                },
            );
        }
        None => println!("No current evaluating package, impossible!"),
    }
    Ok(())
}

type ScriptMetadataMap = HashMap<String, serde_json::Value>;
#[op2]
fn op_add_manifest_metadata(#[serde] metadata: ScriptMetadataMap) -> Result<(), AnyError> {
    match Rift::instance().get_current_evaluating_package() {
        Some(pkg) => {
            // WIP
            println!(
                "op_add_manifest_metadata => current_evaluating_package: {}",
                pkg.manifest().name()
            );
            WorkspaceManager::instance()
                .add_metadata_for_package(pkg.manifest().name(), metadata.clone());
        }
        None => println!("No current evaluating package, impossible!"),
    }
    Ok(())
}

#[op2]
fn op_add_manifest_dependencies(
    #[serde] dependency: TomlDependencyManifestDeclarator,
) -> Result<(), AnyError> {
    match Rift::instance().get_current_evaluating_package() {
        Some(pkg) => {
            // WIP
            println!(
                "op_add_manifest_dependencies => current_evaluating_package: {}",
                pkg.manifest().name()
            );
            WorkspaceManager::instance().add_dependency_for_package(
                pkg.manifest().name(),
                DependencyManifestDeclarator {
                    name: dependency.name.clone(),
                    data: dependency.data.clone(),
                },
            );
        }
        None => println!("No current evaluating package, impossible!"),
    }
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
