use crate::errors::RiftResult;
use crate::package::Package;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

struct Packages {
    packages: HashMap<PathBuf, MaybePackage>,
}

pub enum MaybePackage {
    Package(Package),
    Virtual,
}

pub struct Workspace {
    current_manifest: PathBuf,
    root_manifest: Option<PathBuf>,
    packages: Packages,
}

impl Workspace {
    pub fn new(manifest_path: &Path) -> RiftResult<Workspace> {
        let mut workspace = Workspace::new_default(manifest_path.to_path_buf());
        Ok(workspace)
    }
    fn new_default(current_manifest: PathBuf) -> Workspace {
        Workspace {
            current_manifest,
            root_manifest: None,
            packages: Packages {
                packages: HashMap::new(),
            },
        }
    }

    pub fn root(&self) -> &Path {
        self.root_manifest().parent().unwrap()
    }
    pub fn root_manifest(&self) -> &Path {
        self.root_manifest
            .as_ref()
            .unwrap_or(&self.current_manifest)
    }
}

