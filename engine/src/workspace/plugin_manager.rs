use std::{collections::HashMap, path::PathBuf, rc::Rc};

use walkdir::WalkDir;

use crate::{
    manifest::{EitherManifest, PluginManifest},
    Rift,
};

use super::WorkspaceManager;

pub struct PluginInstance {
    inner: Rc<PluginInstanceInner>,
}

struct PluginInstanceInner {
    manifest: PluginManifest,
    manifest_path: PathBuf,
}

impl PluginInstance {
    pub fn new(manifest: PluginManifest, manifest_path: PathBuf) -> Self {
        Self {
            inner: Rc::new(PluginInstanceInner {
                manifest,
                manifest_path,
            }),
        }
    }
    pub fn manifest(&self) -> &PluginManifest {
        &self.inner.manifest
    }

    pub fn manifest_path(&self) -> &PathBuf {
        &self.inner.manifest_path
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

        for dir in WalkDir::new(installation_path) {
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
        for dir in WalkDir::new(home_path) {
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
        for dir in WalkDir::new(workspace_root) {
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
        Some(ret)
    }

    pub fn load_plugins(&mut self) {
        let possible_plugins = self.enumerate_all_possible_plugins().unwrap_or(Vec::new());

        possible_plugins.iter().for_each(|path| {
            println!("plugin_path: {}", path.display());
        });
    }
}
