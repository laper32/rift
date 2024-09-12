use std::{collections::HashMap, path::PathBuf, rc::Rc};

use crate::manifest::{EitherManifest, PluginManifest, PluginManifestDeclarator};

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
    pub pending_plugins: HashMap<
        String,                        // 需要加载插件的包
        Vec<PluginManifestDeclarator>, // 对应的插件
    >,
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
            pending_plugins: HashMap::new(),
            plugins: HashMap::new(),
        }
    }
    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<PluginManager> =
            once_cell::sync::Lazy::new(|| PluginManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn register_plugin(&mut self, pkg_name: String, plugin: PluginManifestDeclarator) {
        self.pending_plugins
            .entry(pkg_name)
            .or_insert(Vec::new())
            .push(plugin);
    }

    pub fn load_plugin(&mut self) {}

    pub fn find_plugin_from_script_path(
        &self,
        script_path: &PathBuf,
    ) -> Option<(
        PathBuf,        // 插件Manifest路径
        PluginManifest, // 插件Manifest
    )> {
        for pl in &self.plugins {
            let instance = pl.1;
            let script = (|| {
                if instance.manifest().dependencies.is_some() {
                    Some(PathBuf::from(
                        instance.manifest().dependencies.clone().unwrap(),
                    ))
                } else if instance.manifest().metadata.is_some() {
                    Some(PathBuf::from(instance.manifest().metadata.clone().unwrap()))
                } else {
                    return None;
                }
            })();
            match script {
                Some(script) => {
                    if script.eq(script_path) {
                        return Some((
                            instance.manifest_path().clone(),
                            instance.manifest().clone(),
                        ));
                    }
                }
                None => return None,
            }
        }
        None
    }
}
