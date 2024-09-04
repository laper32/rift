use std::{collections::HashMap, env, path::PathBuf, rc::Rc};

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
    // 需要执行的脚本路径
    pub path: Option<PathBuf>,

    pub pkg_name: String,
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
                        path: if p.metadata.is_some() {
                            let actual_script_path = PathBuf::from(pkg.0.clone())
                                .parent()
                                .unwrap()
                                .join(p.metadata.clone().unwrap())
                                .as_posix()
                                .unwrap()
                                .to_string();
                            Some(PathBuf::from(actual_script_path))
                        } else {
                            None
                        },
                        pkg_name: p.name.clone(),
                    });
                    if p.target.is_some() {
                        let target = p.target.as_ref().unwrap();
                        ret.push(ManifestScript {
                            kind: ManifestScriptKind::Target,
                            path: if target.metadata.is_some() {
                                let actual_script_path = PathBuf::from(pkg.0.clone())
                                    .parent()
                                    .unwrap()
                                    .join(target.metadata.clone().unwrap())
                                    .as_posix()
                                    .unwrap()
                                    .to_string();
                                Some(PathBuf::from(actual_script_path))
                            } else {
                                None
                            },
                            pkg_name: target.name.clone(),
                        })
                    }
                }
                crate::manifest::Manifest::Target(t) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Target,
                    path: if t.metadata.is_some() {
                        let actual_script_path = PathBuf::from(pkg.0.clone())
                            .parent()
                            .unwrap()
                            .join(t.metadata.clone().unwrap())
                            .as_posix()
                            .unwrap()
                            .to_string();
                        Some(PathBuf::from(actual_script_path))
                    } else {
                        None
                    },
                    pkg_name: t.name.clone(),
                }),
            },
            crate::workspace::MaybePackage::Virtual(vm) => match vm {
                crate::manifest::VirtualManifest::Workspace(w) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Workspace,
                    path: if w.metadata.is_some() {
                        let actual_script_path = PathBuf::from(pkg.0.clone())
                            .parent()
                            .unwrap()
                            .join(w.metadata.clone().unwrap())
                            .as_posix()
                            .unwrap()
                            .to_string();
                        Some(PathBuf::from(actual_script_path))
                    } else {
                        None
                    },
                    pkg_name: w.name.clone(),
                }),
                crate::manifest::VirtualManifest::Folder(f) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Folder,
                    path: None,
                    pkg_name: f.name.clone(),
                }),
            },
            crate::workspace::MaybePackage::Rift(rm) => match rm {
                crate::manifest::RiftManifest::Plugin(p) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Plugin,
                    path: if p.metadata.is_some() {
                        let actual_script_path = PathBuf::from(pkg.0.clone())
                            .parent()
                            .unwrap()
                            .join(p.metadata.clone().unwrap())
                            .as_posix()
                            .unwrap()
                            .to_string();
                        Some(PathBuf::from(actual_script_path))
                    } else {
                        None
                    },
                    pkg_name: p.name.clone(),
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
                        path: if p.plugins.is_some() {
                            let actual_script_path = PathBuf::from(pkg.0.clone())
                                .parent()
                                .unwrap()
                                .join(p.plugins.clone().unwrap())
                                .as_posix()
                                .unwrap()
                                .to_string();
                            Some(PathBuf::from(actual_script_path))
                        } else {
                            None
                        },
                        pkg_name: p.name.clone(),
                    });
                    if p.target.is_some() {
                        let target = p.target.as_ref().unwrap();
                        ret.push(ManifestScript {
                            kind: ManifestScriptKind::Target,
                            path: if target.plugins.is_some() {
                                let actual_script_path = PathBuf::from(pkg.0.clone())
                                    .parent()
                                    .unwrap()
                                    .join(target.plugins.clone().unwrap())
                                    .as_posix()
                                    .unwrap()
                                    .to_string();
                                Some(PathBuf::from(actual_script_path))
                            } else {
                                None
                            },
                            pkg_name: target.name.clone(),
                        })
                    }
                }
                crate::manifest::Manifest::Target(t) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Target,
                    path: if t.plugins.is_some() {
                        let actual_script_path = PathBuf::from(pkg.0.clone())
                            .parent()
                            .unwrap()
                            .join(t.plugins.clone().unwrap())
                            .as_posix()
                            .unwrap()
                            .to_string();

                        Some(PathBuf::from(actual_script_path))
                    } else {
                        None
                    },
                    pkg_name: t.name.clone(),
                }),
            },
            crate::workspace::MaybePackage::Virtual(vm) => match vm {
                crate::manifest::VirtualManifest::Workspace(w) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Workspace,
                    path: if w.plugins.is_some() {
                        let actual_script_path = PathBuf::from(pkg.0.clone())
                            .parent()
                            .unwrap()
                            .join(w.plugins.clone().unwrap())
                            .as_posix()
                            .unwrap()
                            .to_string();

                        Some(PathBuf::from(actual_script_path))
                    } else {
                        None
                    },
                    pkg_name: w.name.clone(),
                }),
                crate::manifest::VirtualManifest::Folder(f) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Folder,
                    path: None,
                    pkg_name: f.name.clone(),
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
                        path: if p.dependencies.is_some() {
                            let actual_script_path = PathBuf::from(pkg.0.clone())
                                .parent()
                                .unwrap()
                                .join(p.dependencies.clone().unwrap())
                                .as_posix()
                                .unwrap()
                                .to_string();
                            Some(PathBuf::from(actual_script_path))
                        } else {
                            None
                        },
                        pkg_name: p.name.clone(),
                    });
                    if p.target.is_some() {
                        let target = p.target.as_ref().unwrap();
                        ret.push(ManifestScript {
                            kind: ManifestScriptKind::Target,
                            path: if target.dependencies.is_some() {
                                let actual_script_path = PathBuf::from(pkg.0.clone())
                                    .parent()
                                    .unwrap()
                                    .join(target.dependencies.clone().unwrap())
                                    .as_posix()
                                    .unwrap()
                                    .to_string();
                                Some(PathBuf::from(actual_script_path))
                            } else {
                                None
                            },
                            pkg_name: target.name.clone(),
                        })
                    }
                }
                crate::manifest::Manifest::Target(t) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Target,
                    path: if t.dependencies.is_some() {
                        let actual_script_path = PathBuf::from(pkg.0.clone())
                            .parent()
                            .unwrap()
                            .join(t.dependencies.clone().unwrap())
                            .as_posix()
                            .unwrap()
                            .to_string();
                        Some(PathBuf::from(actual_script_path))
                    } else {
                        None
                    },
                    pkg_name: t.name.clone(),
                }),
            },
            crate::workspace::MaybePackage::Virtual(vm) => match vm {
                crate::manifest::VirtualManifest::Workspace(w) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Workspace,
                    path: if w.dependencies.is_some() {
                        let actual_script_path = PathBuf::from(pkg.0.clone())
                            .parent()
                            .unwrap()
                            .join(w.dependencies.clone().unwrap())
                            .as_posix()
                            .unwrap()
                            .to_string();
                        Some(PathBuf::from(actual_script_path))
                    } else {
                        None
                    },
                    pkg_name: w.name.clone(),
                }),
                crate::manifest::VirtualManifest::Folder(f) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Folder,
                    path: None,
                    pkg_name: f.name.clone(),
                }),
            },
            crate::workspace::MaybePackage::Rift(rm) => match rm {
                crate::manifest::RiftManifest::Plugin(p) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Plugin,
                    path: if p.dependencies.is_some() {
                        let actual_script_path = PathBuf::from(pkg.0.clone())
                            .parent()
                            .unwrap()
                            .join(p.dependencies.clone().unwrap())
                            .as_posix()
                            .unwrap()
                            .to_string();
                        Some(PathBuf::from(actual_script_path))
                    } else {
                        None
                    },
                    pkg_name: p.name.clone(),
                }),
            },
        });
    Ok(ret)
}

pub fn collect_all_workspace_scripts() -> Vec<ManifestScript> {
    let mut ret: Vec<ManifestScript> = Vec::new();
    match collect_workspace_dependencies_scripts() {
        Ok(dependencies_scripts) => ret.extend(dependencies_scripts),
        Err(_) => {}
    }
    match collect_workspace_metadata_scripts() {
        Ok(metadata_scripts) => ret.extend(metadata_scripts),
        Err(_) => {}
    }
    match collect_workspace_plugins_scripts() {
        Ok(plugins_scripts) => ret.extend(plugins_scripts),
        Err(_) => {}
    }
    ret
}

fn check_workspace_execution_scripts_unique() -> RiftResult<()> {
    let execution_scripts = collect_all_workspace_scripts();

    // make sure all script unique
    // otherwise false
    let mut occurence_map: HashMap<PathBuf, u32> = HashMap::new();
    for script in execution_scripts {
        if script.path.is_some() {
            occurence_map
                .entry(script.path.clone().unwrap())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
    if occurence_map.values().any(|&count| count > 1) {
        // get count > 1's keys
        let mut keys: Vec<PathBuf> = Vec::new();
        for (key, value) in occurence_map.iter() {
            if *value > 1 {
                keys.push(key.clone());
            }
        }
        let fmt_message = keys
            .iter()
            .map(|key| key.to_string_lossy())
            .collect::<Vec<_>>()
            .join("\n");
        anyhow::bail!("Execution scripts are not unique, these are: \n{fmt_message}");
    } else {
        Ok(())
    }
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
    match check_workspace_execution_scripts_unique() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {e}");
            return;
        }
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let plugin_scripts = collect_workspace_plugins_scripts().unwrap();
    let dependencies_scripts = collect_workspace_dependencies_scripts().unwrap();
    let metadata_scripts = collect_workspace_metadata_scripts().unwrap();

    plugin_scripts
        .iter()
        .for_each(|script| match script.path.clone() {
            Some(path) => {
                if let Err(error) = runtime.block_on(run_js(&path.to_str().unwrap())) {
                    eprintln!("error: {error}");
                }
            }
            None => return,
        });
    dependencies_scripts
        .iter()
        .for_each(|script| match script.path.clone() {
            Some(path) => {
                if let Err(error) = runtime.block_on(run_js(&path.to_str().unwrap())) {
                    eprintln!("error: {error}");
                }
            }
            None => return,
        });

    metadata_scripts
        .iter()
        .for_each(|script| match script.path.clone() {
            Some(path) => {
                if let Err(error) = runtime.block_on(run_js(&path.to_str().unwrap())) {
                    eprintln!("error: {error}");
                }
            }
            None => return,
        });
}

pub fn shutdown() {}

#[cfg(test)]
mod test {

    use crate::{
        util::{self},
        workspace::{plugin_manager::PluginManager, WorkspaceManager},
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
        init();
        PluginManager::instance().print();
        // println!("{:?}", PluginManager::instance().print());
    }
}
