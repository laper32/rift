use std::{collections::HashMap, path::PathBuf};

use walkdir::WalkDir;

use crate::{
    manifest::{read_manifest, PluginManifest},
    util::fs::{as_posix::PathBufExt, NON_INSTALLATION_PATH_NAME},
    Rift,
};

use super::WorkspaceManager;

#[derive(Debug)]
pub struct ManifestPluginIdentifier {
    pub name: String,
    pub version: String,
}

/// 插件系统
pub struct PluginManager {
    pending_load_plugins: HashMap<String, Vec<ManifestPluginIdentifier>>,
}

impl PluginManager {
    fn new() -> Self {
        Self {
            pending_load_plugins: HashMap::new(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<PluginManager> =
            once_cell::sync::Lazy::new(|| PluginManager::new());
        unsafe { &mut *INSTANCE }
    }

    // 首先得知道整个Workspace需要多少插件
    pub fn register_manifest_plugin(
        &mut self,
        pkg_name: String,
        identifier: ManifestPluginIdentifier,
    ) {
        self.pending_load_plugins
            .entry(pkg_name)
            .or_insert(Vec::new())
            .push(identifier);
    }

    fn enumerate_all_possible_plugins(&self) -> Vec<PathBuf> {
        let mut possible_plugins: Vec<PathBuf> = Vec::new();
        let from_installation_path = || -> Option<Vec<PathBuf>> {
            let mut ret: Vec<PathBuf> = Vec::new();
            let installation_path = Rift::instance().installation_path();
            match installation_path {
                Ok(path) => {
                    let plugin_path = path.join("plugins");
                    if !plugin_path.exists() {
                        return None;
                    }
                    let plugin_path = PathBuf::from(plugin_path.as_posix().unwrap().to_string());
                    for entry in WalkDir::new(plugin_path) {
                        match entry {
                            Ok(entry) => {
                                if entry.path().is_file() {
                                    let file = entry.path();
                                    if file.file_name().unwrap() == "Rift.toml" {
                                        ret.push(file.to_path_buf());
                                    }
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(_) => {
                    return None;
                }
            }
            Some(ret)
        };

        let from_user_path = || -> Option<Vec<PathBuf>> {
            let mut ret: Vec<PathBuf> = Vec::new();
            let user_path = Rift::instance().home_path();
            match user_path {
                Ok(path) => {
                    let plugin_path = path.join("plugins");
                    if !plugin_path.exists() {
                        return None;
                    }
                    let plugin_path = PathBuf::from(plugin_path.as_posix().unwrap().to_string());

                    for entry in WalkDir::new(plugin_path) {
                        match entry {
                            Ok(entry) => {
                                if entry.path().is_file() {
                                    let file = entry.path();
                                    if file.file_name().unwrap() == "Rift.toml" {
                                        ret.push(file.to_path_buf());
                                        // possible_plugins.push(file.to_path_buf());
                                    }
                                }
                            }
                            Err(_) => {}
                        }
                    }
                }
                Err(_) => return None,
            }
            Some(ret)
        };
        let from_workspace_root_path = || -> Option<Vec<PathBuf>> {
            if !WorkspaceManager::instance().is_package_loaded() {
                return None;
            }
            let mut ret: Vec<PathBuf> = Vec::new();
            let plugin_path = WorkspaceManager::instance()
                .root()
                .join(NON_INSTALLATION_PATH_NAME)
                .join("plugins");
            if !plugin_path.exists() {
                return None;
            }
            for entry in WalkDir::new(plugin_path) {
                match entry {
                    Ok(entry) => {
                        if entry.path().is_file() {
                            let file = entry.path();
                            if file.file_name().unwrap() == "Rift.toml" {
                                ret.push(file.to_path_buf());
                            }
                        }
                    }
                    Err(_) => {}
                }
            }

            Some(ret)
        };

        match from_installation_path() {
            Some(ret) => possible_plugins.extend(ret),
            None => {}
        }
        match from_user_path() {
            Some(ret) => possible_plugins.extend(ret),
            None => {}
        }
        match from_workspace_root_path() {
            Some(ret) => possible_plugins.extend(ret),
            None => {}
        }

        possible_plugins
    }

    pub fn enumerate_all_pending_load_plugins(&self) -> Vec<PathBuf> {
        let mut pending_load_plugins: Vec<PathBuf> = Vec::new();
        let mut possible_plugins: HashMap<PathBuf, PluginManifest> = HashMap::new();
        for path in self.enumerate_all_possible_plugins() {
            let manifest = read_manifest(&path);
            match manifest {
                Ok(manifest) => match manifest {
                    crate::manifest::EitherManifest::Rift(r) => match r {
                        crate::manifest::RiftManifest::Plugin(p) => {
                            possible_plugins.insert(path.clone(), p.clone());
                        }
                    },
                    _ => {}
                },
                Err(_) => {}
            }
        }
        possible_plugins.iter().for_each(|(path, manifest)| {
            self.pending_load_plugins
                .iter()
                .for_each(|(_, identifiers)| {
                    for identifier in identifiers {
                        if identifier.name.eq(&manifest.name)
                            && identifier.version.eq(&manifest.version)
                        {
                            pending_load_plugins
                                .push(PathBuf::from(path.as_posix().unwrap().to_string()));
                        }
                    }
                });
        });
        pending_load_plugins
    }

    pub fn collect_pending_load_plugins_dependency_manifests(&self) -> Vec<PathBuf> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::{runtime, util, workspace::WorkspaceManager};

    #[test]
    fn test_enumerate_plugins() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("02_single_target_with_project")
            .join("Rift.toml");
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        WorkspaceManager::instance().load_packages();
        runtime::init();
    }
}
