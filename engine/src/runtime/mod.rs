use deno_ast::swc::common::plugin;
use ops::runtime;
use std::{env, path::PathBuf, rc::Rc};
use tokio::runtime::Runtime;

use crate::{
    manifest::EitherManifest,
    rift,
    util::errors::RiftResult,
    workspace::{self, plugin_manager::PluginManager, WorkspaceManager},
    CurrentEvaluatingPackage, Rift,
};

mod loader;
mod ops;

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNTIME_SNAPSHOT.bin"));

async fn run_js(file_path: &str) -> RiftResult<()> {
    let main_module = deno_core::resolve_path(file_path, env::current_dir()?.as_path())?;
    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(loader::TsModuleLoader)),
        startup_snapshot: Some(&RUNTIME_SNAPSHOT),
        extensions: vec![runtime::init_ops(), rift::init_ops(), {
            use workspace::ops::workspace;
            workspace::init_ops()
        }],
        ..Default::default()
    });
    let mod_id = js_runtime.load_main_es_module(&main_module).await?;

    // Rift::instance().set_current_evaluating_package(current_evaluaating_package);
    // Rift::instance().set_current_evaluation_script(PathBuf::from(file_path));
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
                if let Err(error) = runtime.block_on(run_js(&plugins.to_str().unwrap())) {
                    eprintln!("error: {error}");
                }
            }
            None => {
                println!("\"{}\" => Plugins => Not found, skip", pkg.pkg().name());
            }
        }

        /* let plugin_path = packages.package_plugins(manifest_path);
        match plugin_path {
            Some(plugin_path) => {
                println!("plugin_path: {}", plugin_path.display());
                let pkg: EitherManifest = WorkspaceManager::instance()
                    .find_package_from_manifest_path(manifest_path)
                    .unwrap()
                    .pkg()
                    .clone()
                    .into();
                Rift::instance().set_current_evaluating_package(CurrentEvaluatingPackage::new(
                    pkg,
                    manifest_path.clone(),
                ));
                if let Err(error) = runtime.block_on(run_js(&plugin_path.to_str().unwrap())) {
                    eprintln!("error: {error}");
                }
            }
            None => {}
        } */
    });
}

pub fn init() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    declare_workspace_plugins(&runtime);
}

pub fn shutdown() {}

#[cfg(test)]
mod test {

    use crate::{
        util::{self},
        workspace::{plugin_manager::PluginManager, WorkspaceManager},
    };

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
                PluginManager::instance().load_plugins();
            }
            Err(error) => {
                println!("error: {:?}", error);
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
                PluginManager::instance().load_plugins();
            }
            Err(error) => {
                eprintln!("{}", error);
            }
        }
        // WorkspaceManager::instance().print_packages();
    }
}
