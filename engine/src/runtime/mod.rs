use std::path::{Path, PathBuf};

use crate::workspace::WorkspaceManager;

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
    pub path: PathBuf,
}

pub fn collect_manifest_scripts(manifest_path: &Path) -> Vec<ManifestScript> {
    WorkspaceManager::instance().set_current_manifest(&manifest_path.to_path_buf());
    WorkspaceManager::instance().load_packages();
    WorkspaceManager::instance()
        .get_packages()
        .iter()
        .for_each(|pkg| {});
    todo!()
}
