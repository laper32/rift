use serde::{Deserialize, Serialize};

use crate::manifest::{Manifest, RiftManifest, VirtualManifest};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use super::get_actual_script_path;

/// 和[`Package`]对应
/// 语义保持统一
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiftPackage {
    inner: Rc<RiftPackageInner>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RiftPackageInner {
    manifest: RiftManifest,
    manifest_path: PathBuf,
}

impl RiftPackage {
    pub fn new(manifest: RiftManifest, manifest_path: &Path) -> RiftPackage {
        RiftPackage {
            inner: Rc::new(RiftPackageInner {
                manifest,
                manifest_path: manifest_path.to_path_buf(),
            }),
        }
    }
    pub fn manifest(&self) -> &RiftManifest {
        &self.inner.manifest
    }
    pub fn manifest_path(&self) -> &Path {
        &self.inner.manifest_path
    }
    pub fn root(&self) -> &Path {
        &self.inner.manifest_path.parent().unwrap()
    }
    pub fn name(&self) -> String {
        self.manifest().name()
    }

    /// 依赖脚本路径，此时一定是绝对路径
    pub fn dependencies(&self) -> Option<PathBuf> {
        match self.manifest().dependencies() {
            Some(dependencies) => Some(get_actual_script_path(
                self.manifest_path().to_path_buf(),
                &dependencies,
            )),
            None => None,
        }
    }

    /// Metadata脚本路径，此时一定是绝对路径
    pub fn metadata(&self) -> Option<PathBuf> {
        match self.manifest().metadata() {
            Some(metadata) => Some(get_actual_script_path(
                self.manifest_path().to_path_buf(),
                &metadata,
            )),
            None => None,
        }
    }
}

/// 和[`Package`]对应
/// 不然我们每次处理workspace和folder都要match个不停，很烦的好不
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPackage {
    inner: Rc<VirtualPackageInner>,
}

#[derive(Debug, Serialize, Deserialize)]
struct VirtualPackageInner {
    manifest: VirtualManifest,
    manifest_path: PathBuf,
}

impl VirtualPackage {
    pub fn new(manifest: VirtualManifest, manifest_path: &Path) -> VirtualPackage {
        VirtualPackage {
            inner: Rc::new(VirtualPackageInner {
                manifest,
                manifest_path: manifest_path.to_path_buf(),
            }),
        }
    }
    pub fn manifest(&self) -> &VirtualManifest {
        &self.inner.manifest
    }
    pub fn manifest_path(&self) -> &Path {
        &self.inner.manifest_path
    }
    pub fn root(&self) -> &Path {
        &self.inner.manifest_path.parent().unwrap()
    }
    pub fn name(&self) -> String {
        self.manifest().name()
    }
    /// 插件脚本路径，此时一定是绝对路径
    pub fn plugins(&self) -> Option<PathBuf> {
        match self.manifest().plugins() {
            Some(plugins) => Some(get_actual_script_path(
                self.manifest_path().to_path_buf(),
                &plugins,
            )),
            None => None,
        }
    }
    /// 依赖脚本路径，此时一定是绝对路径
    pub fn dependencies(&self) -> Option<PathBuf> {
        match self.manifest().dependencies() {
            Some(dependencies) => Some(get_actual_script_path(
                self.manifest_path().to_path_buf(),
                &dependencies,
            )),
            None => None,
        }
    }

    /// Metadata脚本路径，此时一定是绝对路径
    pub fn metadata(&self) -> Option<PathBuf> {
        match self.manifest().metadata() {
            Some(metadata) => Some(get_actual_script_path(
                self.manifest_path().to_path_buf(),
                &metadata,
            )),
            None => None,
        }
    }
}

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
    pub fn name(&self) -> String {
        self.manifest().name()
    }
    /// 插件脚本路径，此时一定是绝对路径
    pub fn plugins(&self) -> Option<PathBuf> {
        match self.manifest().plugins() {
            Some(plugins) => Some(get_actual_script_path(
                self.manifest_path().to_path_buf(),
                &plugins,
            )),
            None => None,
        }
    }
    /// 依赖脚本路径，此时一定是绝对路径
    pub fn dependencies(&self) -> Option<PathBuf> {
        match self.manifest().dependencies() {
            Some(dependencies) => Some(get_actual_script_path(
                self.manifest_path().to_path_buf(),
                &dependencies,
            )),
            None => None,
        }
    }

    /// Metadata脚本路径，此时一定是绝对路径
    pub fn metadata(&self) -> Option<PathBuf> {
        match self.manifest().metadata() {
            Some(metadata) => Some(get_actual_script_path(
                self.manifest_path().to_path_buf(),
                &metadata,
            )),
            None => None,
        }
    }
}
