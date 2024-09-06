pub mod ops;
pub mod plugin_manager;

use serde::{Deserialize, Serialize};

use crate::util::fs::as_posix::PathExt;
use crate::{
    manifest::{
        find_root_manifest, read_manifest, EitherManifest, Manifest, VirtualManifest,
        MANIFEST_IDENTIFIER,
    },
    package::Package,
    util::{errors::RiftResult, fs::as_posix::PathBufExt},
};
use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
};

pub struct Packages {
    packages: HashMap<PathBuf, MaybePackage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaybePackage {
    Package(Package),
    Virtual(VirtualManifest),
}

impl MaybePackage {
    pub fn name(&self) -> String {
        match self {
            MaybePackage::Package(p) => p.name(),
            MaybePackage::Virtual(v) => v.name(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum WorkspaceStatus {
    Unknown,
    Init,
    PackageLoaded,
    PluginLoaded,
}

pub type PackageMetadata = HashMap<String, serde_json::Value>;
pub type PackageMetadataMap = HashMap<String, PackageMetadata>;

struct PackageDependency {
    name: String,
    version: String,
}

pub type PackageDependencyMap = HashMap<String, Vec<PackageDependency>>;

// 后面改名成WorkspaceManager算了。。。用到workspace的地方那么多，而且按理说也应当只关心项目本身才对
pub struct WorkspaceManager {
    // 我们需要知道是在哪里调用的rift
    current_manifest: PathBuf,

    // 我们需要记录这个workspace的根目录在哪里
    root_manifest: Option<PathBuf>,

    // 有多少包
    packages: Packages,
    status: WorkspaceStatus,
    pkg_metadata: PackageMetadataMap,
    pkg_dependencies: PackageDependencyMap,
}

impl WorkspaceManager {
    fn new() -> Self {
        Self {
            current_manifest: PathBuf::new(),
            root_manifest: None,
            packages: Packages {
                packages: HashMap::new(),
            },
            status: WorkspaceStatus::Unknown,
            pkg_metadata: HashMap::new(),
            pkg_dependencies: HashMap::new(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<WorkspaceManager> =
            once_cell::sync::Lazy::new(|| WorkspaceManager::new());
        unsafe { &mut *INSTANCE }
    }

    /// Set current manifest.
    /// This will also try to find root manifest.
    pub fn set_current_manifest(&mut self, current_manifest: &PathBuf) {
        self.root_manifest = find_root_manifest(&current_manifest);
        self.current_manifest = current_manifest.to_path_buf();
    }

    pub fn root(&self) -> &Path {
        self.root_manifest().parent().unwrap()
    }

    pub fn root_manifest(&self) -> &Path {
        self.root_manifest
            .as_ref()
            .unwrap_or(&self.current_manifest)
    }

    pub fn load_packages(&mut self) {
        self.status = WorkspaceStatus::Init;
        self.packages
            .scan_all_possible_packages(&self.current_manifest);
        self.status = WorkspaceStatus::PackageLoaded;
    }

    pub fn get_package_metadata(&self, pkg_name: String) -> HashMap<String, serde_json::Value> {
        self.pkg_metadata
            .get(&pkg_name)
            .unwrap_or(&HashMap::new())
            .clone()
    }
    pub fn get_all_metadata(&self) -> &PackageMetadataMap {
        &self.pkg_metadata
    }

    pub fn set_package_metadata(
        &mut self,
        pkg_name: String,
        metadata: HashMap<String, serde_json::Value>,
    ) {
        self.pkg_metadata.insert(pkg_name, metadata);
    }

    pub fn get_packages(&self) -> &HashMap<PathBuf, MaybePackage> {
        &self.packages.packages
    }
    pub fn find_package(&self, manifest_path: &PathBuf) -> RiftResult<&MaybePackage> {
        self.packages.packages.get(manifest_path).ok_or_else(|| {
            anyhow::anyhow!("Package not found: {:?}", manifest_path.to_str().unwrap())
        })
    }

    pub fn is_init(&self) -> bool {
        self.status >= WorkspaceStatus::Init
    }

    pub fn is_package_loaded(&self) -> bool {
        self.status == WorkspaceStatus::PackageLoaded
    }

    /// 拿到包的路径
    /// 这时候没有Rift.toml
    pub fn get_package_path_from_name(&self, package_name: &str) -> RiftResult<&Path> {
        match self.get_manifest_path_from_name(package_name) {
            Ok(path) => Ok(path.parent().unwrap()),
            Err(e) => Err(e),
        }
    }

    /// 拿到包的Manifest路径
    /// 注意，这时候是有Rift.toml的
    pub fn get_manifest_path_from_name(&self, package_name: &str) -> RiftResult<&Path> {
        for pkg in &self.packages.packages {
            match pkg.1 /*: MaybePackage */ {
                MaybePackage::Package(p) => match p.manifest() {
                    Manifest::Project(p) => {
                        if p.name == package_name {
                            return Ok(pkg.0.as_path());
                        }
                    }
                    Manifest::Target(t) => {
                        if t.name == package_name {
                            return Ok(pkg.0.as_path());
                        }
                    }
                },
                MaybePackage::Virtual(v) => match v {
                    VirtualManifest::Workspace(w) => {
                        if w.name == package_name {
                            return Ok(pkg.0.as_path());
                        }
                    }
                    VirtualManifest::Folder(f) => {
                        if f.name == package_name {
                            return Ok(pkg.0.as_path());
                        }
                    }
                },
            }
        }
        anyhow::bail!("Package not found: {}", package_name)
    }

    pub fn is_package_exist(&self, package_name: &str) -> bool {
        for pkg in &self.packages.packages {
            match pkg.1 /*: MaybePackage */ {
                MaybePackage::Package(p) => match p.manifest() {
                    Manifest::Project(p) => {
                        if p.name == package_name {
                            return true;
                        }
                    }
                    Manifest::Target(t) => {
                        if t.name == package_name {
                            return true;
                        }
                    }
                },
                MaybePackage::Virtual(v) => match v {
                    VirtualManifest::Workspace(w) => {
                        if w.name == package_name {
                            return true;
                        }
                    }
                    VirtualManifest::Folder(f) => {
                        if f.name == package_name {
                            return true;
                        }
                    }
                },
            }
        }
        false
    }

    pub fn collect_all_manifest_metadata_scripts(
        &self,
    ) -> Option<HashMap<String, Option<PathBuf>>> {
        if !self.is_package_loaded() {
            return None;
        }
        let mut result: HashMap<String, Option<PathBuf>> = HashMap::new();
        self.get_packages().iter().for_each(|pkg| match pkg.1 {
            MaybePackage::Package(p) => match p.manifest() {
                Manifest::Project(p) => {
                    let path = match &p.metadata {
                        Some(path) => Some(get_actual_script_path(pkg.0.clone(), path)),
                        None => None,
                    };
                    result.insert(p.name.clone(), path);
                }
                Manifest::Target(t) => {
                    let path = match &t.metadata {
                        None => None,
                        Some(path) => Some(get_actual_script_path(pkg.0.clone(), path)),
                    };
                    result.insert(t.name.clone(), path);
                }
            },
            MaybePackage::Virtual(v) => match v {
                VirtualManifest::Workspace(w) => {
                    let path = match &w.metadata {
                        None => None,
                        Some(path) => Some(get_actual_script_path(pkg.0.clone(), path)),
                    };
                    result.insert(w.name.clone(), path);
                }
                VirtualManifest::Folder(_) => {}
            },
        });
        Some(result)
    }

    pub fn collect_all_manifest_dependency_scripts(
        &self,
    ) -> Option<HashMap<String, Option<PathBuf>>> {
        if !self.is_package_loaded() {
            return None;
        }
        let mut result: HashMap<String, Option<PathBuf>> = HashMap::new();
        self.get_packages().iter().for_each(|pkg| match pkg.1 {
            MaybePackage::Package(p) => match p.manifest() {
                Manifest::Project(p) => {
                    let path = match &p.dependencies {
                        Some(path) => Some(get_actual_script_path(pkg.0.clone(), path)),
                        None => None,
                    };
                    result.insert(p.name.clone(), path);
                }
                Manifest::Target(t) => {
                    let path = match &t.dependencies {
                        None => None,
                        Some(path) => Some(get_actual_script_path(pkg.0.clone(), path)),
                    };
                    result.insert(t.name.clone(), path);
                }
            },
            MaybePackage::Virtual(v) => match v {
                VirtualManifest::Workspace(w) => {
                    let path = match &w.dependencies {
                        None => None,
                        Some(path) => Some(get_actual_script_path(pkg.0.clone(), path)),
                    };
                    result.insert(w.name.clone(), path);
                }
                VirtualManifest::Folder(_) => {}
            },
        });
        Some(result)
    }

    pub fn collect_all_manifest_plugin_scripts(&self) -> Option<HashMap<String, Option<PathBuf>>> {
        if !self.is_package_loaded() {
            return None;
        }
        let mut result: HashMap<String, Option<PathBuf>> = HashMap::new();
        self.get_packages().iter().for_each(|pkg| match pkg.1 {
            MaybePackage::Package(p) => match p.manifest() {
                Manifest::Project(p) => {
                    let path = match &p.plugins {
                        // N.B. 单Project/Target的情况下，Target将会直接被Project覆盖
                        // 所以我们不需要考虑这种情况下Target的脚本是否需要加载。
                        Some(path) => Some(get_actual_script_path(pkg.0.clone(), path)),
                        None => None,
                    };
                    result.insert(p.name.clone(), path);
                }
                Manifest::Target(t) => {
                    let path = match &t.plugins {
                        Some(path) => Some(get_actual_script_path(pkg.0.clone(), path)),
                        None => None,
                    };
                    result.insert(t.name.clone(), path);
                }
            },
            MaybePackage::Virtual(v) => match v {
                VirtualManifest::Workspace(w) => {
                    let path = match &w.plugins {
                        Some(path) => Some(get_actual_script_path(pkg.0.clone(), path)),
                        None => None,
                    };
                    result.insert(w.name.clone(), path);
                }
                VirtualManifest::Folder(_) => {}
            },
        });

        Some(result)
    }
}

impl Packages {
    pub fn load(&mut self, manifest_path: &Path) {
        let key = manifest_path.parent().unwrap();
        match self.packages.entry(key.to_path_buf()) {
            Entry::Occupied(_) => {}
            Entry::Vacant(_) => {
                let manifest = read_manifest(manifest_path).unwrap();
                match manifest {
                    EitherManifest::Real(m) => match m {
                        Manifest::Project(p) => {
                            let pkg_inner = Package::new(Manifest::Project(p), manifest_path);
                            self.insert_package(
                                manifest_path.to_path_buf(),
                                MaybePackage::Package(pkg_inner),
                            );
                        }
                        Manifest::Target(t) => {
                            let pkg_inner = Package::new(Manifest::Target(t), manifest_path);
                            self.insert_package(
                                manifest_path.to_path_buf(),
                                MaybePackage::Package(pkg_inner),
                            );
                        }
                    },
                    EitherManifest::Virtual(vm) => {
                        self.insert_package(manifest_path.to_path_buf(), MaybePackage::Virtual(vm))
                    }
                    _ => {}
                }
            }
        }
    }

    fn insert_package(&mut self, manifest_path: PathBuf, package: MaybePackage) {
        self.packages.insert(manifest_path, package);
    }

    pub fn scan_all_possible_packages(&mut self, manifest_path: &Path) {
        let manifest = read_manifest(manifest_path);
        match manifest {
            Ok(em) => match em {
                EitherManifest::Real(m) => match m {
                    Manifest::Project(p) => {
                        if self.packages.contains_key(manifest_path) {
                            return;
                        }
                        self.load(manifest_path);
                        if p.members.is_some() {
                            p.members.unwrap().iter().for_each(|member| {
                                let full_path = manifest_path
                                    .parent()
                                    .unwrap()
                                    .join(member)
                                    .join(MANIFEST_IDENTIFIER);
                                self.scan_all_possible_packages(&full_path);
                            });
                        }
                    }
                    Manifest::Target(_) => {
                        if self.packages.contains_key(manifest_path) {
                            return;
                        }
                        self.load(manifest_path);
                    }
                },
                EitherManifest::Virtual(vm) => match vm {
                    VirtualManifest::Workspace(wm) => {
                        if self.packages.contains_key(manifest_path) {
                            return;
                        }
                        wm.members.iter().for_each(|member| {
                            let full_path = manifest_path
                                .parent()
                                .unwrap()
                                .join(member)
                                .join(MANIFEST_IDENTIFIER);
                            self.scan_all_possible_packages(&full_path);
                        });
                        self.load(manifest_path);
                    }
                    VirtualManifest::Folder(fm) => {
                        if self.packages.contains_key(manifest_path) {
                            return;
                        }
                        fm.members.unwrap().iter().for_each(|member| {
                            let full_path = manifest_path
                                .parent()
                                .unwrap()
                                .join(member)
                                .join(MANIFEST_IDENTIFIER);
                            self.scan_all_possible_packages(&full_path);
                        });
                        self.load(manifest_path);
                    }
                },
                EitherManifest::Rift(_) => {
                    if self.packages.contains_key(manifest_path) {
                        return;
                    }
                    self.load(manifest_path);
                }
            },
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
}

fn get_actual_script_path(manifest_path: PathBuf, script_path: &String) -> PathBuf {
    PathBuf::from(
        PathBuf::from(manifest_path)
            .parent()
            .unwrap()
            .join(script_path)
            .as_posix()
            .unwrap()
            .to_string(),
    )
}

#[cfg(test)]
mod test {

    use crate::util;

    use super::WorkspaceManager;

    #[test]
    fn test_load_workspace_happypath() {
        // happypath是什么？
        // 一定在根目录底下，且有Rift.toml
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("04_workspace_and_multiple_projects")
            .join("Rift.toml"); // 这里只有一个Workspace和Project，没有别的东西
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        let expected_root = our_project_root
            .join("sample")
            .join("04_workspace_and_multiple_projects");
        let expected_root_manifest = our_project_root
            .join("sample")
            .join("04_workspace_and_multiple_projects")
            .join("Rift.toml");
        assert_eq!(WorkspaceManager::instance().root(), expected_root);
        assert_eq!(
            WorkspaceManager::instance().root_manifest(),
            expected_root_manifest
        );
    }
    #[test]
    fn test_simple_target() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("01_simple_target")
            .join("Rift.toml");
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        WorkspaceManager::instance().load_packages();
        println!(
            "{}",
            serde_json::to_string_pretty(&WorkspaceManager::instance().packages.packages).unwrap()
        );
    }
    #[test]
    fn test_single_target_with_project() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("02_single_target_with_project")
            .join("Rift.toml");
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        WorkspaceManager::instance().load_packages();
        assert_eq!(WorkspaceManager::instance().packages.packages.len(), 1);
        println!(
            "{}",
            serde_json::to_string_pretty(&WorkspaceManager::instance().packages.packages).unwrap()
        );
    }
    #[test]
    fn test_single_project_with_multiple_targets() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("03_single_project_with_multiple_target")
            .join("Rift.toml"); //
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        WorkspaceManager::instance().load_packages();

        assert_eq!(WorkspaceManager::instance().packages.packages.len(), 5);
        println!(
            "{}",
            serde_json::to_string_pretty(&WorkspaceManager::instance().packages.packages).unwrap()
        );
    }
    #[test]
    fn test_workspace_and_mutiple_projects() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("04_workspace_and_multiple_projects")
            .join("Rift.toml"); //
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        WorkspaceManager::instance().load_packages();
        assert_eq!(WorkspaceManager::instance().packages.packages.len(), 5);
        println!(
            "{}",
            serde_json::to_string_pretty(&WorkspaceManager::instance().packages.packages).unwrap()
        );
    }

    #[test]
    fn test_project_folder_target() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("05_project_folder_target")
            .join("Rift.toml"); //
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        WorkspaceManager::instance().load_packages();
        assert_eq!(WorkspaceManager::instance().packages.packages.len(), 11);
        println!(
            "{}",
            serde_json::to_string_pretty(&WorkspaceManager::instance().packages.packages).unwrap()
        );
    }

    #[test]
    fn test_workspace_folder_project_target() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("06_workspace_folder_project_target")
            .join("Rift.toml"); //
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        WorkspaceManager::instance().load_packages();
        assert_eq!(WorkspaceManager::instance().packages.packages.len(), 33); // ...是巧合吗？
        println!(
            "{}",
            serde_json::to_string_pretty(&WorkspaceManager::instance().packages.packages).unwrap()
        );
    }
}
