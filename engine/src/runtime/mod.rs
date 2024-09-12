use ops::runtime;
use std::{env, path::PathBuf, rc::Rc};
use tokio::runtime::Runtime;

use crate::{
    rift,
    util::errors::RiftResult,
    workspace::{self, plugin_manager::PluginManager, WorkspaceManager},
    CurrentEvaluatingPackage, Rift,
};

mod loader;
mod ops;

static RUNTIME_SNAPSHOT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/RUNTIME_SNAPSHOT.bin"));

async fn run_js(file_path: &str) -> RiftResult<()> {
    fn set_current_evaluating_package(file_path: &str) {
        let pkg =
            WorkspaceManager::instance().find_package_from_script_path(&PathBuf::from(file_path));
        match pkg {
            Some(pkg) => {
                let this_pkg =
                    CurrentEvaluatingPackage::new(pkg.1.clone().into(), pkg.0.to_path_buf());
                Rift::instance().set_current_evaluating_package(this_pkg);
            }
            None => {
                let pl = PluginManager::instance()
                    .find_plugin_from_script_path(&PathBuf::from(file_path));
                match pl {
                    Some(pl) => {
                        let this_pl =
                            CurrentEvaluatingPackage::new(pl.1.clone().into(), pl.0.to_path_buf());
                        Rift::instance().set_current_evaluating_package(this_pl);
                    }
                    None => {
                        println!("No package or plugin found, path => {}", file_path);
                    }
                }
            }
        }
    }

    // let current_evaluaating_package = (|| {
    //     let pkg =
    //         WorkspaceManager::instance().find_package_from_script_path(&PathBuf::from(file_path));
    //     match pkg {
    //         Some(pkg) => Some(CurrentEvaluatingPackage::new(
    //             pkg.1.clone().into(),
    //             pkg.0.to_path_buf(),
    //         )),
    //         None => {
    //             let pl = PluginManager::instance()
    //                 .find_plugin_from_script_path(&PathBuf::from(file_path));
    //             match pl {
    //                 Some(pl) => Some(CurrentEvaluatingPackage::new(
    //                     pl.1.clone().into(),
    //                     pl.0.to_path_buf(),
    //                 )),
    //                 None => None,
    //             }
    //         }
    //     }
    // })()
    // .unwrap();

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

    set_current_evaluating_package(file_path);
    // Rift::instance().set_current_evaluating_package(current_evaluaating_package);
    // Rift::instance().set_current_evaluation_script(PathBuf::from(file_path));
    // 既然没有执行脚本的return value，那么我们就改变策略，重点让大家去调用提供的API.
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(Default::default()).await?;
    let result = result.await;
    result
}

fn execute_workspace_scripts(runtime: &Runtime) {
    let plugin_scripts = WorkspaceManager::instance().collect_all_manifest_plugin_scripts();
    match plugin_scripts {
        Some(scripts) => {
            scripts
                .iter()
                .for_each(|(_, script_path)| match script_path {
                    Some(script_path) => {
                        if let Err(error) = runtime.block_on(run_js(&script_path.to_str().unwrap()))
                        {
                            eprintln!("error: {error}");
                        }
                    }
                    None => {}
                });
        }
        None => {}
    }

    let dependency_scripts = WorkspaceManager::instance().collect_all_manifest_dependency_scripts();
    match dependency_scripts {
        Some(scripts) => {
            scripts
                .iter()
                .for_each(|(_, script_path)| match script_path {
                    Some(script_path) => {
                        if let Err(error) = runtime.block_on(run_js(&script_path.to_str().unwrap()))
                        {
                            eprintln!("error: {error}");
                        }
                    }
                    None => {}
                });
        }
        None => {}
    }

    let metadata_scripts = WorkspaceManager::instance().collect_all_manifest_metadata_scripts();
    match metadata_scripts {
        Some(scripts) => {
            scripts
                .iter()
                .for_each(|(_, script_path)| match script_path {
                    Some(script_path) => {
                        if let Err(error) = runtime.block_on(run_js(&script_path.to_str().unwrap()))
                        {
                            eprintln!("error: {error}");
                        }
                    }
                    None => {}
                });
        }
        None => {}
    }
}

fn execute_plugin_scripts(runtime: &Runtime) {}

pub fn init() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    execute_workspace_scripts(&runtime);
    execute_plugin_scripts(&runtime);
}

pub fn shutdown() {}

#[cfg(test)]
mod test {

    use crate::{
        util::{self},
        workspace::WorkspaceManager,
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
        WorkspaceManager::instance().load_packages();
        init();
        // println!(
        //     "WorkspaceManager::Instance()->GetAllMetadata() => {:?}",
        //     WorkspaceManager::instance().get_all_metadata()
        // );
        // println!(
        //     "WorkspaceManager::Instance()->GetAllPackageDependencies() => {:?}",
        //     WorkspaceManager::instance().get_all_package_dependencies()
        // );
    }
}
