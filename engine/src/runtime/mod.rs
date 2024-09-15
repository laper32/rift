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

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNTIME_SNAPSHOT.bin"));

pub async fn evaluate(file_path: &str) -> RiftResult<()> {
    let main_module = deno_core::resolve_path(file_path, env::current_dir()?.as_path())?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(loader::TsModuleLoader)),
        startup_snapshot: Some(&RUNTIME_SNAPSHOT),
        extensions: init_engine_ops(),
        ..Default::default()
    });
    let mod_id = js_runtime.load_main_es_module(&main_module).await?;

    // 既然没有执行脚本的return value，那么我们就改变策略，重点让大家去调用提供的API.
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(Default::default()).await?;
    let result = result.await;
    result
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

pub fn init() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    declare_workspace_plugins(&runtime);
    PluginManager::instance().load_plugins();
    declare_dependencies(&runtime);
    declare_metadata(&runtime);
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
