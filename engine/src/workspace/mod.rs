pub mod ops;
mod package;
pub mod plugin_manager;

use package::{Package, VirtualPackage};
use serde::{Deserialize, Serialize};

use crate::manifest::{DependencyManifestDeclarator, PluginManifestDeclarator};
use crate::{
    manifest::{
        find_root_manifest, read_manifest, EitherManifest, Manifest, VirtualManifest,
        MANIFEST_IDENTIFIER,
    },
    util::{errors::RiftResult, fs::as_posix::PathBufExt},
};
use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
};

pub type PackageMetadata = HashMap<String, serde_json::Value>;

pub struct Packages {
    packages: HashMap<
        PathBuf,         // Manifest路径
        PackageInstance, // 不用说？
    >,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInstance {
    pkg: MaybePackage,
    metadata: PackageMetadata,
    dependencies: Vec<DependencyManifestDeclarator>,
    plugins: Vec<PluginManifestDeclarator>,
}

impl PackageInstance {
    fn new(pkg: MaybePackage) -> Self {
        Self {
            pkg,
            metadata: HashMap::new(),
            dependencies: Vec::new(),
            plugins: Vec::new(),
        }
    }
    pub fn pkg(&self) -> &MaybePackage {
        &self.pkg
    }
    pub fn name(&self) -> String {
        self.pkg.name()
    }

    pub fn dependencies(&self) -> &Vec<DependencyManifestDeclarator> {
        &self.dependencies
    }
    pub fn add_dependency(&mut self, dependency: DependencyManifestDeclarator) {
        self.dependencies.push(dependency);
    }
    pub fn metadata(&self) -> &PackageMetadata {
        &self.metadata
    }
    pub fn add_metadata(&mut self, metadata: HashMap<String, serde_json::Value>) {
        metadata.iter().for_each(|(k, v)| {
            self.metadata.insert(k.clone(), v.clone());
        });
    }
    pub fn plugins(&self) -> &Vec<PluginManifestDeclarator> {
        &self.plugins
    }
    pub fn add_plugin(&mut self, plugin: PluginManifestDeclarator) {
        self.plugins.push(plugin);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaybePackage {
    Package(Package),
    Virtual(VirtualPackage),
}

impl Into<EitherManifest> for MaybePackage {
    fn into(self) -> EitherManifest {
        match self {
            MaybePackage::Package(p) => match p.manifest() {
                Manifest::Project(p) => EitherManifest::Real(Manifest::Project(p.clone())),
                Manifest::Target(t) => EitherManifest::Real(Manifest::Target(t.clone())),
            },
            MaybePackage::Virtual(v) => match v.manifest() {
                VirtualManifest::Workspace(w) => {
                    EitherManifest::Virtual(VirtualManifest::Workspace(w.clone()))
                }
                VirtualManifest::Folder(f) => {
                    EitherManifest::Virtual(VirtualManifest::Folder(f.clone()))
                }
            },
        }
    }
}

impl MaybePackage {
    pub fn name(&self) -> String {
        match self {
            MaybePackage::Package(p) => p.name(),
            MaybePackage::Virtual(v) => v.name(),
        }
    }
    pub fn dependencies(&self) -> Option<PathBuf> {
        match self {
            MaybePackage::Package(p) => p.dependencies(),
            MaybePackage::Virtual(v) => v.dependencies(),
        }
    }
    pub fn plugins(&self) -> Option<PathBuf> {
        match self {
            MaybePackage::Package(p) => p.plugins(),
            MaybePackage::Virtual(v) => v.plugins(),
        }
    }
    pub fn metadata(&self) -> Option<PathBuf> {
        match self {
            MaybePackage::Package(p) => p.metadata(),
            MaybePackage::Virtual(v) => v.metadata(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum WorkspaceStatus {
    Unknown,
    Init,
    PackageLoaded,
    PluginLoaded,
    Failed,
}

// 后面改名成WorkspaceManager算了。。。用到workspace的地方那么多，而且按理说也应当只关心项目本身才对
pub struct WorkspaceManager {
    // 我们需要知道是在哪里调用的rift
    current_manifest: PathBuf,

    // 我们需要记录这个workspace的根目录在哪里
    root_manifest: Option<PathBuf>,

    // 有多少包
    packages: Packages,
    status: WorkspaceStatus,
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

    pub fn load_packages(&mut self) -> RiftResult<()> {
        self.status = WorkspaceStatus::Init;
        self.packages
            .scan_all_possible_packages(&self.current_manifest);
        match self.is_package_name_unique() {
            Some(conflict_package) => {
                self.status = WorkspaceStatus::Failed;
                let error_message = conflict_package
                    .iter()
                    .map(|(package_name, paths)| {
                        format!(
                            "Package name conflict: {}\n{}",
                            package_name,
                            paths.iter().fold(String::new(), |acc, path| {
                                format!("{}  - {}\n", acc, path.display())
                            })
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                anyhow::bail!("Package name conflict: {}", error_message);
            }
            None => {
                self.status = WorkspaceStatus::PackageLoaded;
                Ok(())
            }
        }
    }

    pub fn get_packages(&self) -> &Packages {
        &self.packages
    }
    pub fn print_packages(&self) {
        println!(
            "{}",
            serde_json::to_string_pretty(&self.packages.packages).unwrap()
        );
    }

    fn is_package_name_unique(&self) -> Option<HashMap<String, Vec<PathBuf>>> {
        let mut name_counts: HashMap<String, i32> = HashMap::new();
        for (_, instance) in &self.packages.packages {
            let name = instance.name();
            let count = name_counts.entry(name).or_insert(0);
            *count += 1;
        }
        let mut name_manifest: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for (package_name, count) in &name_counts {
            if *count <= 1 {
                continue;
            }
            self.packages
                .packages
                .iter()
                .for_each(|(manifest_path, instance)| {
                    if instance.name().eq(package_name) {
                        let paths = name_manifest
                            .entry(package_name.clone())
                            .or_insert(Vec::new());
                        paths.push(manifest_path.clone());
                    }
                });
        }
        if name_manifest.is_empty() {
            None
        } else {
            Some(name_manifest)
        }
    }

    pub fn find_package_from_manifest_path(
        &self,
        manifest_path: &PathBuf,
    ) -> RiftResult<&PackageInstance> {
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
        for (manifest_path, instance) in &self.packages.packages {
            if instance.name() == package_name {
                return Ok(manifest_path.as_path());
            }
        }
        anyhow::bail!("Package not found: {}", package_name)
    }

    pub fn is_package_exist(&self, package_name: &str) -> bool {
        for (_, instance) in &self.packages.packages {
            if instance.name() == package_name {
                return true;
            }
        }
        false
    }

    pub fn add_dependency_for_package(
        &mut self,
        package_name: String,
        dependency: DependencyManifestDeclarator,
    ) {
        self.packages
            .packages
            .iter_mut()
            .find(|(_, instance)| instance.name() == package_name)
            .map(|(_, instance)| instance.add_dependency(dependency));
    }

    pub fn add_plugin_for_package(
        &mut self,
        package_name: String,
        plugin: PluginManifestDeclarator,
    ) {
        self.packages
            .packages
            .iter_mut()
            .find(|(_, instance)| instance.name() == package_name)
            .map(|(_, instance)| instance.add_plugin(plugin));
    }
    pub fn add_metadata_for_package(
        &mut self,
        package_name: String,
        metadata: HashMap<String, serde_json::Value>,
    ) {
        self.packages
            .packages
            .iter_mut()
            .find(|(_, instance)| instance.name() == package_name)
            .map(|(_, instance)| instance.add_metadata(metadata));
    }
}

impl Packages {
    pub fn get_manifest_paths(&self) -> Vec<PathBuf> {
        self.packages.keys().cloned().collect()
    }

    pub fn package_name(&self, manifest_path: &PathBuf) -> RiftResult<String> {
        match self.packages.get(manifest_path) {
            Some(pkg) => Ok(pkg.name()),
            None => anyhow::bail!(
                "Impossible case: Package not found. Manifest path: {:?}",
                manifest_path
            ),
        }
    }

    pub fn load(&mut self, manifest_path: &Path) {
        let key = manifest_path.parent().unwrap();
        match self.packages.entry(key.to_path_buf()) {
            Entry::Occupied(_) => {}
            Entry::Vacant(_) => {
                let manifest = read_manifest(manifest_path).unwrap();
                match manifest {
                    EitherManifest::Real(m) => {
                        let pkg_inner = Package::new(m.clone(), manifest_path);
                        self.insert_package(
                            manifest_path.to_path_buf(),
                            MaybePackage::Package(pkg_inner),
                        );
                    }

                    EitherManifest::Virtual(vm) => {
                        let pkg_inner = VirtualPackage::new(vm.clone(), manifest_path);
                        self.insert_package(
                            manifest_path.to_path_buf(),
                            MaybePackage::Virtual(pkg_inner),
                        );
                    }
                    _ => {}
                }
            }
        }
    }

    fn insert_package(&mut self, manifest_path: PathBuf, package: MaybePackage) {
        self.packages
            .insert(manifest_path, PackageInstance::new(package));
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
