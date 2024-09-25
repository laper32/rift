mod ops;

use std::{collections::HashMap, env, path::PathBuf};

use anyhow::Context;
use deno_ast::swc::common::plugin;
use deno_core::v8::{self, Value};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{
    manifest::{
        read_manifest, DependencyManifestDeclarator, EitherManifest, PluginManifest, RiftManifest,
    },
    runtime::ScriptRuntime,
    task::TaskManager,
    util::errors::RiftResult,
    workspace::{package::RiftPackage, WorkspaceManager},
    Rift,
};

pub fn init_ops() -> deno_core::Extension {
    ops::plsys::init_ops()
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PluginStatus {
    None,
    Init,
    Running,
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
    #[serde(skip_deserializing, skip_serializing)]
    on_load_fn: Option<v8::Global<v8::Function>>,
    #[serde(skip_deserializing, skip_serializing)]
    on_all_loaded_fn: Option<v8::Global<v8::Function>>,
    #[serde(skip_deserializing, skip_serializing)]
    on_unload_fn: Option<v8::Global<v8::Function>>,
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
                status: PluginStatus::None,
                on_load_fn: None,
                on_all_loaded_fn: None,
                on_unload_fn: None,
            },
        }
    }

    pub fn status(&self) -> &PluginStatus {
        &self.inner.status
    }

    pub fn is_error(&self) -> bool {
        self.is_failed() || self.is_runtime_error()
    }

    pub fn is_failed(&self) -> bool {
        match self.inner.status {
            PluginStatus::Failed => true,
            _ => false,
        }
    }

    pub fn is_runtime_error(&self) -> bool {
        match self.inner.status {
            PluginStatus::RuntimeError => true,
            _ => false,
        }
    }

    pub fn is_init(&self) -> bool {
        match self.inner.status {
            PluginStatus::Init | PluginStatus::Running => true,
            _ => false,
        }
    }

    pub fn is_running(&self) -> bool {
        match self.inner.status {
            PluginStatus::Running => true,
            _ => false,
        }
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

    fn on_load_fn_ref(&self) -> Option<&v8::Global<v8::Function>> {
        self.inner.on_load_fn.as_ref()
    }

    fn on_all_loaded_fn_ref(&self) -> Option<&v8::Global<v8::Function>> {
        self.inner.on_all_loaded_fn.as_ref()
    }

    fn on_unload_fn_ref(&self) -> Option<&v8::Global<v8::Function>> {
        self.inner.on_unload_fn.as_ref()
    }
}

pub struct PluginManager {
    plugins: HashMap<
        String,         // 插件名
        PluginInstance, // 插件实例信息
    >,

    curr_plugin_cursor: Option<String>, // 当前正在运行的插件实例（只用于OnLoad, OnAllLoaded, OnUnload）
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
            curr_plugin_cursor: None,
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
        let possible_plugins = self.load_possible_plugins_manifest();
        let declared_plugins = WorkspaceManager::instance().collect_declared_plugins();
        declared_plugins.iter().for_each(|declarator| {
            let plugin = possible_plugins.iter().find(|(_, possible_plugin)| {
                if possible_plugin.name().eq(&declarator.name) {
                    return true;
                }
                return false;
            });
            if plugin.is_some() {
                let (manifest_path, manifest) = plugin.unwrap();
                match manifest {
                    EitherManifest::Rift(rift_manifest) => match rift_manifest {
                        RiftManifest::Plugin(plugin_manifest) => {
                            let instance =
                                PluginInstance::new(plugin_manifest.clone(), manifest_path.clone());
                            self.plugins.insert(instance.name(), instance);
                        }
                    },
                    _ => {}
                }
            }
        });
    }

    pub fn plugins_count(&self) -> usize {
        self.plugins.len()
    }

    fn load_possible_plugins_manifest(&mut self) -> HashMap<PathBuf, EitherManifest> {
        let mut ret = HashMap::new();
        let possible_plugins = self.enumerate_all_possible_plugins().unwrap_or(Vec::new());
        possible_plugins.iter().for_each(|path| {
            let manifest = read_manifest(&path.as_path());
            match manifest {
                Ok(manifest) => match manifest {
                    EitherManifest::Rift(rm) => match rm {
                        crate::manifest::RiftManifest::Plugin(ref pm) => {
                            ret.insert(path.clone(), pm.clone().into());
                        }
                    },
                    _ => {}
                },
                Err(e) => {
                    eprintln!(
                        "Error when parsing manifest: \"{}\". Error: {}",
                        path.display(),
                        e
                    );
                }
            }
        });
        ret
    }

    // fn read_plugin_manifest(&mut self, manifest_path: &PathBuf) -> RiftResult<()> {
    //     let manifest = read_manifest(&manifest_path.as_path());
    //     match manifest {
    //         Ok(manifest) => match manifest {
    //             EitherManifest::Rift(rm) => match rm {
    //                 crate::manifest::RiftManifest::Plugin(ref pm) => {
    //                     let instance = PluginInstance::new(pm.clone(), manifest_path.clone());

    //                     self.plugins.insert(instance.name(), instance);
    //                     Ok(())
    //                 }
    //             },
    //             _ => Ok(()),
    //         },
    //         Err(e) => Err(e).context(format!(
    //             "Error when parsing manifest: \"{}\"",
    //             manifest_path.display()
    //         )),
    //     }
    // }

    pub fn get_manifests(&self) -> Vec<PathBuf> {
        self.plugins
            .values()
            .map(|p| p.manifest_path().clone())
            .collect()
    }

    pub fn find_plugin_from_name(&self, name: &str) -> Option<&PluginInstance> {
        self.plugins.get(name)
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

    pub fn find_plugin_from_script_path(&self, script_path: &PathBuf) -> Option<&PluginInstance> {
        self.plugins
            .values()
            .find(|x| x.entry().unwrap().eq(script_path))
    }

    pub fn evaluate_entries(&mut self) {
        for (_, instance) in &self.plugins {
            let entry = instance.entry();
            match entry {
                Some(entry) => {
                    if let Err(error) = ScriptRuntime::instance().evaluate(|| async move {
                        Rift::instance().set_current_evaluating_script(entry.clone());
                        // plugin entry should be main, or else? Need to test whether have error..
                        let to_eval_entry = deno_core::resolve_path(
                            entry.to_str().unwrap(),
                            env::current_dir()?.as_path(),
                        )?;
                        let mod_id = ScriptRuntime::instance()
                            .js_runtime()
                            .load_side_es_module(&to_eval_entry)
                            .await?;
                        let result = ScriptRuntime::instance().js_runtime().mod_evaluate(mod_id);
                        ScriptRuntime::instance()
                            .js_runtime()
                            .run_event_loop(Default::default())
                            .await?;
                        result.await
                    }) {
                        eprintln!("error: {error}");
                    }
                }
                None => {
                    eprintln!("Plugin {} has no entry", instance.name());
                }
            }
        }
    }

    fn register_instance_load_fn(
        &mut self,
        plugin_name: String,
        on_load_fn: v8::Global<v8::Function>,
    ) {
        self.plugins
            .iter_mut()
            .find(|(_, instance)| instance.name() == plugin_name)
            .map(|(_, instance)| {
                instance.inner.on_load_fn = Some(on_load_fn);
            });
    }

    fn register_instance_unload_fn(
        &mut self,
        plugin_name: String,
        on_unload_fn: v8::Global<v8::Function>,
    ) {
        self.plugins
            .iter_mut()
            .find(|(_, instance)| instance.name() == plugin_name)
            .map(|(_, instance)| {
                instance.inner.on_unload_fn = Some(on_unload_fn);
            });
    }

    fn register_instance_all_loaded_fn(
        &mut self,
        plugin_name: String,
        on_all_loaded_fn: v8::Global<v8::Function>,
    ) {
        self.plugins
            .iter_mut()
            .find(|(_, instance)| instance.name() == plugin_name)
            .map(|(_, instance)| {
                instance.inner.on_all_loaded_fn = Some(on_all_loaded_fn);
            });
    }

    pub fn activate_instances(&mut self) {
        self.load_instances();
        self.on_instances_all_loaded();
    }

    pub fn deactivate_instances(&mut self) {
        self.unload_instances();
    }

    fn load_instances(&mut self) {
        self.plugins.iter_mut().for_each(|(name, instance)| {
            instance.set_status(PluginStatus::Init);
            let mut scope = ScriptRuntime::instance().js_runtime().handle_scope();
            let undefined: v8::Local<Value> = v8::undefined(&mut scope).into();
            match instance.on_load_fn_ref() {
                Some(on_load_fn_ref) => {
                    self.curr_plugin_cursor = Some(name.clone());
                    let on_load_fn = v8::Local::new(&mut scope, on_load_fn_ref);
                    let _ = on_load_fn.call(&mut scope, undefined.into(), &[]);
                }
                None => { /* ... */ }
            }
        });
    }

    fn on_instances_all_loaded(&mut self) {
        self.plugins.iter_mut().for_each(|(name, instance)| {
            let mut scope = ScriptRuntime::instance().js_runtime().handle_scope();
            let undefined: v8::Local<Value> = v8::undefined(&mut scope).into();
            match instance.on_all_loaded_fn_ref() {
                Some(on_all_loaded_fn_ref) => {
                    self.curr_plugin_cursor = Some(name.clone());
                    let on_all_loaded_fn = v8::Local::new(&mut scope, on_all_loaded_fn_ref);
                    let _ = on_all_loaded_fn.call(&mut scope, undefined.into(), &[]);
                    instance.set_status(PluginStatus::Running);
                }
                None => { /* ... */ }
            }
        });
    }

    fn unload_instances(&mut self) {
        self.plugins.iter_mut().for_each(|(name, instance)| {
            let mut scope = ScriptRuntime::instance().js_runtime().handle_scope();
            let undefined: v8::Local<Value> = v8::undefined(&mut scope).into();
            match instance.on_unload_fn_ref() {
                Some(on_unload_fn_ref) => {
                    self.curr_plugin_cursor = Some(name.clone());
                    let on_unload_fn = v8::Local::new(&mut scope, on_unload_fn_ref);
                    let _ = on_unload_fn.call(&mut scope, undefined.into(), &[]);
                    TaskManager::instance().remove_task_from_pkg_name(&name);
                    instance.set_status(PluginStatus::None);
                }
                None => { /* ... */ }
            }
        });
    }

    pub(crate) fn get_current_plugin_cursor(&self) -> Option<&String> {
        self.curr_plugin_cursor.as_ref()
    }
}
