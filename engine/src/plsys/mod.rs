mod ops;

use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{
    manifest::{read_manifest, DependencyManifestDeclarator, EitherManifest, PluginManifest},
    workspace::{package::RiftPackage, WorkspaceManager},
    Rift,
};

pub fn init_ops() -> deno_core::Extension {
    ops::plsys::init_ops()
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PluginStatus {
    Unknown,
    Init,
    Loaded,
    Failed,
    RuntimeError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginInstance {
    inner: PluginInstanceInner,
}

#[derive(Debug, Serialize, Deserialize)]
struct PluginInstanceInner {
    pkg: RiftPackage,
    manifest_path: PathBuf,
    metadata: HashMap<String, serde_json::Value>,
    dependencies: Vec<DependencyManifestDeclarator>,
    status: PluginStatus,
}

impl PluginInstance {
    pub fn new(manifest: PluginManifest, manifest_path: PathBuf) -> Self {
        Self {
            inner: PluginInstanceInner {
                pkg: RiftPackage::new(
                    crate::manifest::RiftManifest::Plugin(manifest),
                    &manifest_path,
                ),
                manifest_path,
                metadata: HashMap::new(),
                dependencies: Vec::new(),
                status: PluginStatus::Unknown,
            },
        }
    }

    pub fn status(&self) -> &PluginStatus {
        &self.inner.status
    }

    pub fn set_status(&mut self, status: PluginStatus) {
        self.inner.status = status;
    }

    pub fn pkg(&self) -> &RiftPackage {
        &self.inner.pkg
    }
    pub fn entry(&self) -> Option<PathBuf> {
        self.inner.pkg.entry()
    }

    pub fn manifest_path(&self) -> &PathBuf {
        &self.inner.manifest_path
    }

    pub fn name(&self) -> String {
        self.inner.pkg.name()
    }
    pub fn dependencies(&self) -> &Vec<DependencyManifestDeclarator> {
        &self.inner.dependencies
    }
    pub fn add_dependency(&mut self, dependency: DependencyManifestDeclarator) {
        self.inner.dependencies.push(dependency);
    }
    pub fn metadata(&self) -> &HashMap<String, serde_json::Value> {
        &self.inner.metadata
    }
    pub fn add_metadata(&mut self, metadata: HashMap<String, serde_json::Value>) {
        metadata.iter().for_each(|(k, v)| {
            self.inner.metadata.insert(k.clone(), v.clone());
        });
    }
}

pub struct PluginManager {
    plugins: HashMap<
        String,         // 插件名
        PluginInstance, // 插件实例信息
    >,
}

impl Into<EitherManifest> for PluginManifest {
    fn into(self) -> EitherManifest {
        EitherManifest::Rift(crate::manifest::RiftManifest::Plugin(self))
    }
}

impl PluginManager {
    fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<PluginManager> =
            once_cell::sync::Lazy::new(|| PluginManager::new());
        unsafe { &mut *INSTANCE }
    }
    /// 列举所有可能的插件包目录
    /// 只处理如下目录:
    /// - ${env:HOME}/.rift/plugins
    /// - ${installationPath}/plugins
    /// - ${workspaceRoot}/.rift/plugins
    fn enumerate_all_possible_plugins(&self) -> Option<Vec<PathBuf>> {
        if !WorkspaceManager::instance().is_package_loaded() {
            return None;
        }
        let mut ret: Vec<PathBuf> = Vec::new();
        let installation_path = Rift::instance()
            .rift_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("plugins");
        let home_path = Rift::instance().home_path().unwrap().join("plugins");
        let workspace_root = WorkspaceManager::instance()
            .root()
            .join(".rift")
            .join("plugins");

        let mut enumerate_impl = |path_for_enumerate: PathBuf| {
            for dir in WalkDir::new(path_for_enumerate) {
                match dir {
                    Ok(dir) => {
                        if dir.file_type().is_file() {
                            if dir.path().ends_with("Rift.toml") {
                                ret.push(dir.into_path());
                            }
                        }
                    }
                    Err(_) => continue,
                }
            }
        };
        enumerate_impl(installation_path);
        enumerate_impl(home_path);
        enumerate_impl(workspace_root);
        Some(ret)
    }

    pub fn load_plugins(&mut self) {
        let possible_plugins = self.enumerate_all_possible_plugins().unwrap_or(Vec::new());

        possible_plugins.iter().for_each(|path| {
            self.read_plugin_manifest(path);
        });
    }

    pub fn plugins_count(&self) -> usize {
        self.plugins.len()
    }

    fn read_plugin_manifest(&mut self, manifest_path: &PathBuf) {
        let manifest = read_manifest(&manifest_path.as_path());
        match manifest {
            Ok(manifest) => match manifest {
                EitherManifest::Rift(rm) => match rm {
                    crate::manifest::RiftManifest::Plugin(ref pm) => {
                        let instance = PluginInstance::new(pm.clone(), manifest_path.clone());
                        self.plugins.insert(instance.name(), instance);
                    }
                },
                _ => { /* Do nothing */ }
            },
            Err(_) => { /* Do nothing */ }
        }
    }

    pub fn get_manifests(&self) -> Vec<PathBuf> {
        self.plugins
            .values()
            .map(|p| p.manifest_path().clone())
            .collect()
    }
    pub fn find_plugin_from_manifest_path(
        &self,
        manifest_path: &PathBuf,
    ) -> Option<&PluginInstance> {
        self.plugins
            .values()
            .find(|x| x.manifest_path().eq(manifest_path))
    }

    pub fn print_plugins(&self) {
        println!("{}", serde_json::to_string_pretty(&self.plugins).unwrap());
    }
    pub fn add_dependency_for_plugin(
        &mut self,
        plugin_name: String,
        dependency: DependencyManifestDeclarator,
    ) {
        self.plugins
            .iter_mut()
            .find(|(_, instance)| instance.name() == plugin_name)
            .map(|(_, instance)| instance.add_dependency(dependency));
    }

    pub fn add_metadata_for_plugin(
        &mut self,
        plugin_name: String,
        metadata: HashMap<String, serde_json::Value>,
    ) {
        self.plugins
            .iter_mut()
            .find(|(_, instance)| instance.name() == plugin_name)
            .map(|(_, instance)| instance.add_metadata(metadata));
    }

    pub fn activate_plugins(&self) {
        self.plugins.iter().for_each(|(_, instance)| {
            let entry = instance.entry();
            match entry {
                Some(entry) => {

                    /* let _ = std::process::Command::new("node")
                    .arg(entry)
                    .spawn()
                    .expect("Failed to start plugin"); */
                }
                None => {
                    eprintln!("Plugin {} has no entry", instance.name());
                }
            }
        });
    }
}
