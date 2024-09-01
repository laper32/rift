use serde::{Deserialize, Serialize};

use crate::manifest::Manifest;
use std::path::{Path, PathBuf};
use std::rc::Rc;

/// Information about a package that is available somewhere in the file system.
///
/// A package is a `Rift.toml` file plus all the files that are part of it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    inner: Rc<PackageInner>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PackageInner {
    manifest: Manifest,
    manifest_path: PathBuf,
}

impl Package {
    pub fn new(manifest: Manifest, manifest_path: &Path) -> Package {
        Package {
            inner: Rc::new(PackageInner {
                manifest,
                manifest_path: manifest_path.to_path_buf(),
            }),
        }
    }
    pub fn manifest(&self) -> &Manifest {
        &self.inner.manifest
    }

    pub fn manifest_path(&self) -> &Path {
        &self.inner.manifest_path
    }

    pub fn root(&self) -> &Path {
        &self.inner.manifest_path.parent().unwrap()
    }
}
