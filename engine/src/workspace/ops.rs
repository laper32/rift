use deno_core::{error::AnyError, extension, op2};
use std::collections::HashMap;

use crate::manifest::DependencyManifestDeclarator;
use crate::manifest::EitherManifest;
use crate::manifest::PluginManifestDeclarator;
use crate::manifest::RiftManifest;
use crate::schema::TomlDependencyManifestDeclarator;
use crate::schema::TomlPluginManifestDeclarator;
use crate::workspace::plugin_manager::PluginManager;
use crate::workspace::WorkspaceManager;
use crate::Rift;

#[op2]
fn op_add_manifest_plugin(#[serde] content: TomlPluginManifestDeclarator) -> Result<(), AnyError> {
    match Rift::instance().get_current_evaluating_package() {
        Some(pkg) => match &pkg.manifest {
            EitherManifest::Real(_) | EitherManifest::Virtual(_) => {
                WorkspaceManager::instance().add_plugin_for_package(
                    pkg.manifest().name(),
                    PluginManifestDeclarator {
                        name: content.name.clone(),
                        data: content.data.clone(),
                    },
                );
            }
            _ => {}
        },
        None => println!("No current evaluating package, impossible!"),
    }
    Ok(())
}

#[op2]
fn op_add_manifest_metadata(
    #[serde] metadata: HashMap<String, serde_json::Value>,
) -> Result<(), AnyError> {
    match Rift::instance().get_current_evaluating_package() {
        Some(pkg) => match &pkg.manifest {
            EitherManifest::Real(_) | EitherManifest::Virtual(_) => {
                WorkspaceManager::instance()
                    .add_metadata_for_package(pkg.manifest().name(), metadata.clone());
            }
            EitherManifest::Rift(r) => match r {
                RiftManifest::Plugin(_) => {
                    PluginManager::instance()
                        .add_metadata_for_plugin(pkg.manifest().name(), metadata.clone());
                }
            },
            _ => {}
        },
        None => println!("No current evaluating package, impossible!"),
    }
    Ok(())
}

#[op2]
fn op_add_manifest_dependencies(
    #[serde] dependency: TomlDependencyManifestDeclarator,
) -> Result<(), AnyError> {
    match Rift::instance().get_current_evaluating_package() {
        Some(pkg) => match &pkg.manifest {
            EitherManifest::Real(_) | EitherManifest::Virtual(_) => {
                WorkspaceManager::instance().add_dependency_for_package(
                    pkg.manifest().name(),
                    DependencyManifestDeclarator {
                        name: dependency.name.clone(),
                        data: dependency.data.clone(),
                    },
                );
            }
            EitherManifest::Rift(r) => match r {
                RiftManifest::Plugin(_) => {
                    PluginManager::instance().add_dependency_for_plugin(
                        pkg.manifest().name(),
                        DependencyManifestDeclarator {
                            name: dependency.name.clone(),
                            data: dependency.data.clone(),
                        },
                    );
                }
            },
        },
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
