use std::path::PathBuf;

use crate::{util::errors::RiftResult, workspace::WorkspaceManager};

mod graph;

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
                crate::manifest::Manifest::Project(p) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Project,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if p.metadata.is_some() {
                            Some(PathBuf::from(p.metadata.clone().unwrap()))
                        } else {
                            None
                        }
                    },
                }),
                crate::manifest::Manifest::Target(t) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Target,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if t.metadata.is_some() {
                            Some(PathBuf::from(t.metadata.clone().unwrap()))
                        } else {
                            None
                        }
                    },
                }),
            },
            crate::workspace::MaybePackage::Virtual(vm) => match vm {
                crate::manifest::VirtualManifest::Workspace(w) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Workspace,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if w.metadata.is_some() {
                            Some(PathBuf::from(w.metadata.clone().unwrap()))
                        } else {
                            None
                        }
                    },
                }),
                crate::manifest::VirtualManifest::Folder(f) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Folder,
                    manifest_path: pkg.0.clone(),
                    path: None,
                    // path: {
                    //     if f.dependencies.is_some() {
                    //         Some(PathBuf::from(f.dependencies.clone().unwrap()))
                    //     } else {
                    //         None
                    //     }
                    // },
                }),
            },
            crate::workspace::MaybePackage::Rift(rm) => match rm {
                crate::manifest::RiftManifest::Plugin(p) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Plugin,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if p.metadata.is_some() {
                            Some(PathBuf::from(p.metadata.clone().unwrap()))
                        } else {
                            None
                        }
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
                crate::manifest::Manifest::Project(p) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Project,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if p.plugins.is_some() {
                            Some(PathBuf::from(p.plugins.clone().unwrap()))
                        } else {
                            None
                        }
                    },
                }),
                crate::manifest::Manifest::Target(t) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Target,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if t.plugins.is_some() {
                            Some(PathBuf::from(t.plugins.clone().unwrap()))
                        } else {
                            None
                        }
                    },
                }),
            },
            crate::workspace::MaybePackage::Virtual(vm) => match vm {
                crate::manifest::VirtualManifest::Workspace(w) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Workspace,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if w.plugins.is_some() {
                            Some(PathBuf::from(w.plugins.clone().unwrap()))
                        } else {
                            None
                        }
                    },
                }),
                crate::manifest::VirtualManifest::Folder(f) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Folder,
                    manifest_path: pkg.0.clone(),
                    path: None,
                    // path: {
                    //     if f.dependencies.is_some() {
                    //         Some(PathBuf::from(f.dependencies.clone().unwrap()))
                    //     } else {
                    //         None
                    //     }
                    // },
                }),
            },
            crate::workspace::MaybePackage::Rift(rm) => match rm {
                crate::manifest::RiftManifest::Plugin(p) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Plugin,
                    manifest_path: pkg.0.clone(),
                    path: None,
                }),
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
                crate::manifest::Manifest::Project(p) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Project,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if p.dependencies.is_some() {
                            Some(PathBuf::from(p.dependencies.clone().unwrap()))
                        } else {
                            None
                        }
                    },
                }),
                crate::manifest::Manifest::Target(t) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Target,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if t.dependencies.is_some() {
                            Some(PathBuf::from(t.dependencies.clone().unwrap()))
                        } else {
                            None
                        }
                    },
                }),
            },
            crate::workspace::MaybePackage::Virtual(vm) => match vm {
                crate::manifest::VirtualManifest::Workspace(w) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Workspace,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if w.dependencies.is_some() {
                            Some(PathBuf::from(w.dependencies.clone().unwrap()))
                        } else {
                            None
                        }
                    },
                }),
                crate::manifest::VirtualManifest::Folder(f) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Folder,
                    manifest_path: pkg.0.clone(),
                    path: None,
                    // path: {
                    //     if f.dependencies.is_some() {
                    //         Some(PathBuf::from(f.dependencies.clone().unwrap()))
                    //     } else {
                    //         None
                    //     }
                    // },
                }),
            },
            crate::workspace::MaybePackage::Rift(rm) => match rm {
                crate::manifest::RiftManifest::Plugin(p) => ret.push(ManifestScript {
                    kind: ManifestScriptKind::Plugin,
                    manifest_path: pkg.0.clone(),
                    path: {
                        if p.dependencies.is_some() {
                            Some(PathBuf::from(p.dependencies.clone().unwrap()))
                        } else {
                            None
                        }
                    },
                }),
            },
        });
    Ok(ret)
}

/*
按照一般情况来说，应该是先确定我们要启用什么插件，然后看需要什么依赖，最后再加载对应的metadata？
*/

#[cfg(test)]
mod test {
    use crate::workspace::WorkspaceManager;

    #[test]
    fn workspace_dependencies_scripts() {
        // WorkspaceManager::instance()
    }
}
