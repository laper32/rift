use std::{
    env,
    path::{Path, PathBuf},
    rc::Rc,
};

use ops::runtime;

use crate::{
    rift,
    util::{errors::RiftResult, fs::as_posix::PathBufExt},
    workspace::{self, WorkspaceManager},
    Rift,
};

mod loader;
mod ops;

#[derive(Clone, Debug)]
pub enum ManifestScriptKind {
    Workspace,
    Project,
    Folder,
    Target,
    Plugin,
}

/// 只用于记录形如`plugins`, `dependencies`这些字段的信息，方便我们为图排序做准备。
#[derive(Clone, Debug)]
pub struct ManifestScript {
    pub kind: ManifestScriptKind,
    pub path: Option<PathBuf>,
    pub manifest_path: PathBuf,
}

pub fn collect_workspace_metadata_scripts() -> RiftResult<Vec<ManifestScript>> {
    if !WorkspaceManager::instance().is_loaded() {
        return Err(anyhow::format_err!("Workspace is not loaded"));
    }
    let mut ret: Vec<ManifestScript> = Vec::new();
    WorkspaceManager::instance()
        .get_packages()
        .iter()
        .for_each(|pkg| match pkg.1 {
            crate::workspace::MaybePackage::Package(m) => match m.manifest() {
                crate::manifest::Manifest::Project(p) => {
                    ret.push(ManifestScript {
                        kind: ManifestScriptKind::Project,
                        manifest_path: pkg.0.clone(),
                        path: if p.metadata_script_path.is_some() {
                            Some(PathBuf::from(p.metadata_script_path.clone().unwrap()))
                        } else {
                            None
                        },
                    });
                    if p.target.is_some() {
                        let target = p.target.as_ref().unwrap();
                        ret.push(ManifestScript {
                            kind: ManifestScriptKind::Target,
                            manifest_path: pkg.0.clone(),
                            path: if target.metadata_script_path.is_some() {
                                Some(PathBuf::from(target.metadata_script_path.clone().unwrap()))
                            } else {
                                None
                            },
                        })
                    }
                }
                crate::manifest::Manifest::Target(t) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Target,
                    manifest_path: pkg.0.clone(),
                    path: if t.metadata_script_path.is_some() {
                        Some(PathBuf::from(t.metadata_script_path.clone().unwrap()))
                    } else {
                        None
                    },
                }),
            },
            crate::workspace::MaybePackage::Virtual(vm) => match vm {
                crate::manifest::VirtualManifest::Workspace(w) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Workspace,
                    manifest_path: pkg.0.clone(),
                    path: if w.metadata_script_path.is_some() {
                        Some(PathBuf::from(w.metadata_script_path.clone().unwrap()))
                    } else {
                        None
                    },
                }),
                crate::manifest::VirtualManifest::Folder(_) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Folder,
                    manifest_path: pkg.0.clone(),
                    path: None,
                }),
            },
            crate::workspace::MaybePackage::Rift(rm) => match rm {
                crate::manifest::RiftManifest::Plugin(p) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Plugin,
                    manifest_path: pkg.0.clone(),
                    path: if p.metadata_script_path.is_some() {
                        Some(PathBuf::from(p.metadata_script_path.clone().unwrap()))
                    } else {
                        None
                    },
                }),
            },
        });
    Ok(ret)
}

pub fn collect_workspace_plugins_scripts() -> RiftResult<Vec<ManifestScript>> {
    if !WorkspaceManager::instance().is_loaded() {
        return Err(anyhow::format_err!("Workspace is not loaded"));
    }

    let mut ret: Vec<ManifestScript> = Vec::new();
    WorkspaceManager::instance()
        .get_packages()
        .iter()
        .for_each(|pkg| match pkg.1 {
            crate::workspace::MaybePackage::Package(m) => match m.manifest() {
                crate::manifest::Manifest::Project(p) => {
                    ret.push(ManifestScript {
                        kind: ManifestScriptKind::Project,
                        manifest_path: pkg.0.clone(),
                        path: if p.plugins_script_path.is_some() {
                            Some(PathBuf::from(p.plugins_script_path.clone().unwrap()))
                        } else {
                            None
                        },
                    });
                    if p.target.is_some() {
                        let target = p.target.as_ref().unwrap();
                        ret.push(ManifestScript {
                            kind: ManifestScriptKind::Target,
                            manifest_path: pkg.0.clone(),
                            path: if target.plugins_script_path.is_some() {
                                Some(PathBuf::from(target.plugins_script_path.clone().unwrap()))
                            } else {
                                None
                            },
                        })
                    }
                }
                crate::manifest::Manifest::Target(t) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Target,
                    manifest_path: pkg.0.clone(),
                    path: if t.plugins_script_path.is_some() {
                        Some(PathBuf::from(t.plugins_script_path.clone().unwrap()))
                    } else {
                        None
                    },
                }),
            },
            crate::workspace::MaybePackage::Virtual(vm) => match vm {
                crate::manifest::VirtualManifest::Workspace(w) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Workspace,
                    manifest_path: pkg.0.clone(),
                    path: if w.plugins_script_path.is_some() {
                        Some(PathBuf::from(w.plugins_script_path.clone().unwrap()))
                    } else {
                        None
                    },
                }),
                crate::manifest::VirtualManifest::Folder(_) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Folder,
                    manifest_path: pkg.0.clone(),
                    path: None,
                }),
            },
            crate::workspace::MaybePackage::Rift(rm) => match rm {
                crate::manifest::RiftManifest::Plugin(_) => { /* ... */ }
            },
        });
    Ok(ret)
}

pub fn collect_workspace_dependencies_scripts() -> RiftResult<Vec<ManifestScript>> {
    if !WorkspaceManager::instance().is_loaded() {
        return Err(anyhow::format_err!("Workspace is not loaded"));
    }
    let mut ret: Vec<ManifestScript> = Vec::new();
    WorkspaceManager::instance()
        .get_packages()
        .iter()
        .for_each(|pkg| match pkg.1 {
            crate::workspace::MaybePackage::Package(m) => match m.manifest() {
                crate::manifest::Manifest::Project(p) => {
                    ret.push(ManifestScript {
                        kind: ManifestScriptKind::Project,
                        manifest_path: pkg.0.clone(),
                        path: if p.dependencies_script_path.is_some() {
                            Some(PathBuf::from(p.dependencies_script_path.clone().unwrap()))
                        } else {
                            None
                        },
                    });
                    if p.target.is_some() {
                        let target = p.target.as_ref().unwrap();
                        ret.push(ManifestScript {
                            kind: ManifestScriptKind::Target,
                            manifest_path: pkg.0.clone(),
                            path: if target.dependencies_script_path.is_some() {
                                Some(PathBuf::from(
                                    target.dependencies_script_path.clone().unwrap(),
                                ))
                            } else {
                                None
                            },
                        })
                    }
                }
                crate::manifest::Manifest::Target(t) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Target,
                    manifest_path: pkg.0.clone(),
                    path: if t.dependencies_script_path.is_some() {
                        Some(PathBuf::from(t.dependencies_script_path.clone().unwrap()))
                    } else {
                        None
                    },
                }),
            },
            crate::workspace::MaybePackage::Virtual(vm) => match vm {
                crate::manifest::VirtualManifest::Workspace(w) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Workspace,
                    manifest_path: pkg.0.clone(),
                    path: if w.dependencies_script_path.is_some() {
                        Some(PathBuf::from(w.dependencies_script_path.clone().unwrap()))
                    } else {
                        None
                    },
                }),
                crate::manifest::VirtualManifest::Folder(f) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Folder,
                    manifest_path: pkg.0.clone(),
                    path: None,
                }),
            },
            crate::workspace::MaybePackage::Rift(rm) => match rm {
                crate::manifest::RiftManifest::Plugin(p) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Plugin,
                    manifest_path: pkg.0.clone(),
                    path: if p.dependencies_script_path.is_some() {
                        Some(PathBuf::from(p.dependencies_script_path.clone().unwrap()))
                    } else {
                        None
                    },
                }),
            },
        });
    Ok(ret)
}

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNJS_SNAPSHOT.bin"));

async fn run_js(file_path: &str) -> RiftResult<()> {
    let main_module = deno_core::resolve_path(file_path, env::current_dir()?.as_path())?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(loader::TsModuleLoader)),
        startup_snapshot: Some(&RUNTIME_SNAPSHOT),
        extensions: vec![runtime::init_ops(), rift::init_ops(), {
            use workspace::ops::workspace;
            workspace::init_ops()
        }],
        ..Default::default()
    });
    let mod_id = js_runtime.load_main_es_module(&main_module).await?;
    Rift::instance().set_current_evaluation_script(PathBuf::from(file_path));
    // 既然没有执行脚本的return value，那么我们就改变策略，重点让大家去调用提供的API.
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(Default::default()).await?;
    let result = result.await;
    result
}

pub fn init() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let plugin_scripts = collect_workspace_plugins_scripts().unwrap();
    let dependencies_scripts = collect_workspace_dependencies_scripts().unwrap();
    let metadata_scripts = collect_workspace_metadata_scripts().unwrap();

    plugin_scripts.iter().for_each(|script| {
        let binding = script.manifest_path.clone();
        let current_manifest_path = binding.parent();
        match current_manifest_path {
            Some(current_manifest_path) => {
                let script_path = script.path.clone();
                match script_path {
                    Some(script_path) => {
                        let actual_script_path = PathBuf::from(current_manifest_path)
                            .join(script_path)
                            .as_posix()
                            .unwrap()
                            .to_string();
                        if let Err(error) = runtime.block_on(run_js(&actual_script_path)) {
                            eprintln!("error: {error}");
                        }
                    }
                    None => return,
                }
            }
            None => return,
        }
    });

    dependencies_scripts.iter().for_each(|script| {
        let binding = script.manifest_path.clone();
        let current_manifest_path = binding.parent();
        match current_manifest_path {
            Some(current_manifest_path) => {
                let script_path = script.path.clone();
                match script_path {
                    Some(script_path) => {
                        let actual_script_path = PathBuf::from(current_manifest_path)
                            .join(script_path)
                            .as_posix()
                            .unwrap()
                            .to_string();
                        if let Err(error) = runtime.block_on(run_js(&actual_script_path)) {
                            eprintln!("error: {error}");
                        }
                    }
                    None => return,
                }
            }
            None => return,
        }
    });

    metadata_scripts.iter().for_each(|script| {
        let binding = script.manifest_path.clone();
        let current_manifest_path = binding.parent();
        match current_manifest_path {
            Some(current_manifest_path) => {
                let script_path = script.path.clone();
                match script_path {
                    Some(script_path) => {
                        let actual_script_path = PathBuf::from(current_manifest_path)
                            .join(script_path)
                            .as_posix()
                            .unwrap()
                            .to_string();
                        if let Err(error) = runtime.block_on(run_js(&actual_script_path)) {
                            eprintln!("error: {error}");
                        }
                    }
                    None => return,
                }
            }
            None => return,
        }
    });
}

pub fn shutdown() {}

#[cfg(test)]
mod test {

    use crate::{
        util::{self},
        workspace::WorkspaceManager,
    };

    use super::init;

    #[test]
    fn workspace_dependencies_scripts() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("02_single_target_with_project")
            .join("Rift.toml");
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        WorkspaceManager::instance().load_packages();
        init()
    }
}
