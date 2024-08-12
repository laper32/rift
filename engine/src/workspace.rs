use crate::errors::{ManifestError, RiftResult, SimpleError};
use crate::package::Package;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use crate::manifest::{Manifest, WorkspaceManifest, MANIFEST_IDENTIFIER};
use crate::schema::{load_manifest, TomlWorkspace};

struct Packages {
    packages: HashMap<PathBuf, Package>,
}

pub struct Workspace {
    current_manifest: PathBuf,
    packages: Packages,
}

trait Node { // TODO: rename
    fn root_manifest(&self) -> Option<PathBuf>;
}

impl Node for Workspace {
    fn root_manifest(&self) -> Option<PathBuf> {
        None
    }
}

impl Workspace {
    pub fn new(manifest_path: &WorkspaceManifest) -> Workspace {

    }
}

pub struct WorkspaceBuilder {
    possible_manifests: HashMap<PathBuf, Manifest>,
    init_path: PathBuf,
}

impl WorkspaceBuilder {
    pub fn new(current_path: &PathBuf) -> Self {
        Self {
            possible_manifests: HashMap::new(),
            init_path: current_path.clone(),
        }
    }

    fn find_root(&self, current_path: &PathBuf) -> Option<PathBuf> {
        let parent_manifest_path = current_path.parent()?.join(MANIFEST_IDENTIFIER);
        if parent_manifest_path.exists() {
            Some(parent_manifest_path)
        } else {
            self.find_root(&current_path.parent()?.to_path_buf())
        }
    }

    pub fn build(self) -> RiftResult<Workspace> {
        let root = match self.find_root(&self.init_path) {
            None => return Err(Box::new(SimpleError::new("Unable to parse manifest toml"))),
            Some(path) => path,
        };
        let root_workspace = load_manifest::<TomlWorkspace>(&root)?;
        todo!()
    }
}