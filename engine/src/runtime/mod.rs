use deno_ast::ModuleSpecifier;
use deno_core::error::AnyError;
use ops::runtime;
use std::{env, rc::Rc};
use tokio::runtime::Runtime;

use crate::{
    manifest::{self, EitherManifest},
    plsys::{self, PluginManager},
    rift,
    util::errors::RiftResult,
    workspace::{self, WorkspaceManager},
    CurrentEvaluatingPackage, Rift,
};

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
}

impl ScriptRuntime {
    fn new() -> Self {
        Self {
            js_runtime: deno_core::JsRuntime::new(deno_core::RuntimeOptions {
                module_loader: Some(Rc::new(loader::TsModuleLoader)),
                startup_snapshot: Some(&RUNTIME_SNAPSHOT),
                extensions: vec![runtime::init_ops(), rift::init_ops(), {
                    use workspace::ops::workspace;
                    workspace::init_ops()
                }],
                ..Default::default()
            }),
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

    pub async fn run_event_loop(&mut self) -> RiftResult<()> {
        self.js_runtime.run_event_loop(Default::default()).await
    }

    pub async fn mod_evaluate(&mut self, id: deno_core::ModuleId) -> RiftResult<()> {
        self.js_runtime.mod_evaluate(id).await
    }

    pub async fn load_main_es_module(
        &mut self,
        module: &ModuleSpecifier,
    ) -> RiftResult<deno_core::ModuleId> {
        self.js_runtime.load_main_es_module(module).await
    }
    pub async fn eval(&mut self, module: &ModuleSpecifier) -> RiftResult<()> {
        let id = self.load_main_es_module(module).await?;
        self.mod_evaluate(id).await
    }
}

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNTIME_SNAPSHOT.bin"));

async fn run_js(file_path: &str) -> RiftResult<()> {
    let module = deno_core::resolve_path(file_path, env::current_dir()?.as_path())?;
    ScriptRuntime::instance().eval(&module).await
}

fn declare_workspace_plugins(runtime: &Runtime) {
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
                if let Err(error) = runtime.block_on(evaluate(&plugins.to_str().unwrap())) {
                    eprintln!("error: {error}");
                }
            }
            None => {}
        }
    });
}

fn declare_dependencies(runtime: &Runtime) {
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
                if let Err(error) = runtime.block_on(evaluate(&dependencies.to_str().unwrap())) {
                    eprintln!("error: {error}");
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
                if let Err(error) = runtime.block_on(evaluate(&dependencies.to_str().unwrap())) {
                    eprintln!("error: {error}");
                }
            }
            None => {}
        }
    });
}

fn declare_metadata(runtime: &Runtime) {
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
                if let Err(error) = runtime.block_on(evaluate(&metadata.to_str().unwrap())) {
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
                if let Err(error) = runtime.block_on(evaluate(&metadata.to_str().unwrap())) {
                    eprintln!("error: {error}");
                }
            }
            None => {}
        }
    });
}

static SHOULD_STOP: bool = false;

pub fn init() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    std::thread::spawn(move || {
        runtime.block_on(async move {
            let _ = ScriptRuntime::instance().run_event_loop().await;
        });
    });
    /*     declare_workspace_plugins(&runtime);
    PluginManager::instance().load_plugins();
    declare_dependencies(&runtime);
    declare_metadata(&runtime); */
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

        // WorkspaceManager::instance().print_packages();
        PluginManager::instance().print_plugins();
    }
}
