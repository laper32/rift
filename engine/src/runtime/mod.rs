use anyhow::Error;
use ops::runtime;
use std::future::Future;
use std::{env, rc::Rc};

use crate::manifest::EitherManifest;
use crate::plsys::PluginManager;
use crate::workspace::WorkspaceManager;
use crate::{
    manifest::{self},
    plsys::{self},
    rift,
    util::errors::RiftResult,
    workspace::{self},
};
use crate::{CurrentEvaluatingPackage, Rift};

mod loader;
mod ops;

fn init_engine_ops() -> Vec<deno_core::Extension> {
    vec![
        runtime::init_ops(),
        rift::init_ops(),
        manifest::init_ops(),
        plsys::init_ops(),
        workspace::init_ops(),
    ]
}

pub struct ScriptRuntime {
    js_runtime: deno_core::JsRuntime,
    tokio: tokio::runtime::Runtime,
}

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNTIME_SNAPSHOT.bin"));
impl ScriptRuntime {
    fn new() -> Self {
        Self {
            js_runtime: deno_core::JsRuntime::new(deno_core::RuntimeOptions {
                module_loader: Some(Rc::new(loader::TsModuleLoader)),
                startup_snapshot: Some(&RUNTIME_SNAPSHOT),
                extensions: init_engine_ops(),
                ..Default::default()
            }),
            tokio: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<ScriptRuntime> =
            once_cell::sync::Lazy::new(|| ScriptRuntime::new());
        unsafe { &mut *INSTANCE }
    }
    pub fn js_runtime(&mut self) -> &mut deno_core::JsRuntime {
        &mut self.js_runtime
    }

    fn eval_manifest_script(&mut self, script: &str) -> RiftResult<()> {
        self.tokio.block_on(async {
            Rift::instance().set_current_evaluating_script(script.into());
            let to_eval_manifest_script =
                deno_core::resolve_path(script, env::current_dir()?.as_path())?;
            let mod_id = self
                .js_runtime
                .load_side_es_module(&to_eval_manifest_script)
                .await?;
            let result = self.js_runtime.mod_evaluate(mod_id);
            self.js_runtime.run_event_loop(Default::default()).await?;
            result.await
        })
    }

    /// ...
    /// 这里不处理timed out逻辑，如果你需要的话自己包装。
    pub fn evaluate<T, F, U>(&mut self, f: F) -> RiftResult<T>
    where
        U: std::future::Future<Output = RiftResult<T>>,
        F: FnOnce() -> U,
    {
        self.tokio.block_on(async { f().await })
    }

    /// ...
    /// 这里不处理timed out逻辑，如果你需要的话自己包装。
    pub async fn evaluate_async<T, F, U>(&mut self, f: F) -> RiftResult<T>
    where
        U: std::future::Future<Output = RiftResult<T>>,
        F: FnOnce() -> U,
    {
        f().await
    }
}

fn declare_workspace_plugins() {
    let packages = WorkspaceManager::instance().get_packages();
    let manifests = packages.get_manifest_paths();
    // Plugins
    manifests.iter().for_each(|manifest_path| {
        let pkg = WorkspaceManager::instance()
            .find_package_from_manifest_path(manifest_path)
            .unwrap();

        match pkg.pkg().plugins() {
            Some(plugins) => {
                Rift::instance().set_current_evaluating_package(CurrentEvaluatingPackage::new(
                    pkg.pkg().clone().into(),
                    manifest_path.clone(),
                ));
                if let Err(error) =
                    ScriptRuntime::instance().eval_manifest_script(&plugins.to_str().unwrap())
                {
                    eprintln!("error: {error}");
                }
            }
            None => {}
        }
    });
}

fn declare_dependencies() {
    // Workspace dependencies
    let packages = WorkspaceManager::instance().get_packages();
    let manifests = packages.get_manifest_paths();
    manifests.iter().for_each(|manifest_path| {
        let pkg = WorkspaceManager::instance()
            .find_package_from_manifest_path(manifest_path)
            .unwrap();
        match pkg.pkg().dependencies() {
            Some(dependencies) => {
                Rift::instance().set_current_evaluating_package(CurrentEvaluatingPackage::new(
                    pkg.pkg().clone().into(),
                    manifest_path.clone(),
                ));
                if let Err(err) =
                    ScriptRuntime::instance().eval_manifest_script(&dependencies.to_str().unwrap())
                {
                    eprintln!("error: {err}");
                }
            }
            None => {}
        }
    });
    // plugin dependencies, since plugins have been already declared.
    let manifests = PluginManager::instance().get_manifests();
    manifests.iter().for_each(|manifest_path| {
        let pkg = PluginManager::instance()
            .find_plugin_from_manifest_path(manifest_path)
            .unwrap();
        match pkg.pkg().dependencies() {
            Some(dependencies) => {
                let evaluating: EitherManifest = pkg.pkg().manifest().clone().into();
                Rift::instance().set_current_evaluating_package(CurrentEvaluatingPackage::new(
                    evaluating,
                    manifest_path.clone(),
                ));
                if let Err(error) =
                    ScriptRuntime::instance().eval_manifest_script(&dependencies.to_str().unwrap())
                {
                    eprintln!("error: {error}");
                }
            }
            None => {}
        }
    });
}

fn declare_metadata() {
    // Workspace metadaata.
    let packages = WorkspaceManager::instance().get_packages();
    let manifests = packages.get_manifest_paths();
    manifests.iter().for_each(|manifest_path| {
        let pkg = WorkspaceManager::instance()
            .find_package_from_manifest_path(manifest_path)
            .unwrap();
        match pkg.pkg().metadata() {
            Some(metadata) => {
                Rift::instance().set_current_evaluating_package(CurrentEvaluatingPackage::new(
                    pkg.pkg().clone().into(),
                    manifest_path.clone(),
                ));
                if let Err(error) =
                    ScriptRuntime::instance().eval_manifest_script(&metadata.to_str().unwrap())
                {
                    eprintln!("error: {error}");
                }
            }
            None => {}
        }
    });

    // plugin metadata, since plugins has been already declared.
    // plugin dependencies, since plugins have been already declared.
    let manifests = PluginManager::instance().get_manifests();
    manifests.iter().for_each(|manifest_path| {
        let pkg = PluginManager::instance()
            .find_plugin_from_manifest_path(manifest_path)
            .unwrap();
        match pkg.pkg().metadata() {
            Some(metadata) => {
                let evaluating: EitherManifest = pkg.pkg().manifest().clone().into();
                Rift::instance().set_current_evaluating_package(CurrentEvaluatingPackage::new(
                    evaluating,
                    manifest_path.clone(),
                ));
                if let Err(error) =
                    ScriptRuntime::instance().eval_manifest_script(&metadata.to_str().unwrap())
                {
                    eprintln!("error: {error}");
                }
            }
            None => {}
        }
    });
}

pub fn init() {
    declare_workspace_plugins();
    PluginManager::instance().load_plugins();
    declare_dependencies();
    declare_metadata();
}

pub fn shutdown() {}

#[cfg(test)]
mod test {

    use crate::{plsys::PluginManager, util, workspace::WorkspaceManager};

    use super::init;

    #[test]
    fn workspace_dependencies_scripts() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("02_single_target_with_project")
            .join("Rift.toml");
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        match WorkspaceManager::instance().load_packages() {
            Ok(_) => {
                init();
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
        WorkspaceManager::instance().print_packages();
    }
    #[test]
    fn workspace_with_project_folder_target() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("05_project_folder_target")
            .join("Rift.toml");
        WorkspaceManager::instance().set_current_manifest(&simple_workspace);
        match WorkspaceManager::instance().load_packages() {
            Ok(_) => {
                init();
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }

        WorkspaceManager::instance().print_packages();
        PluginManager::instance().print_plugins();
    }
}
