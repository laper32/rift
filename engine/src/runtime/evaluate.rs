use std::{path::Path, rc::Rc};

use anyhow::Context;

use crate::util::errors::RiftResult;

use super::{loader::RiftModuleLoader, specifier::RiftModuleSpecifier};

pub async fn evaluate(to_evaluate_module_path: &Path, export: &str) -> RiftResult<()> {
    let specifier = RiftModuleSpecifier::File {
        path: to_evaluate_module_path.to_path_buf(),
    };
    todo!()
    // evaluate_impl(export.to_string(), specifier).await
}

// async fn evaluate_impl(export: String, to_evaluate_module: RiftModuleSpecifier) -> RiftResult<()> {
//     let (result_tx, result_rx) = tokio::sync::oneshot::channel::<RiftResult<()>>();
//     std::thread::spawn(move || {
//         let runtime = tokio::runtime::Builder::new_current_thread()
//             .enable_all()
//             .build();

//         let runtime = match runtime {
//             Ok(runtime) => runtime,
//             Err(error) => {
//                 let _ = result_tx.send(Err(error.into()));
//                 return;
//             }
//         };

//         // Spawn the main JS task on the new runtime. See this issue for
//         // more context on why this is required:
//         // https://github.com/brioche-dev/brioche/pull/105#issuecomment-2241289605
//         let result = runtime.block_on(async move {
//             let module_loader = RiftModuleLoader::new();
//             let mut js_rt = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
//                 module_loader: Some(Rc::new(module_loader)),
//                 extensions: vec![],
//                 ..Default::default()
//             });
//             let to_eval_module: deno_core::ModuleSpecifier = to_evaluate_module.into();

//             tracing::debug!("evaluating module: {:?}", to_eval_module);
//             let module_id = js_rt.load_main_es_module(&to_eval_module).await;
//             if module_id.is_err() {
//                 return;
//             }
//             let module_id = module_id.unwrap();
//             let result = js_rt.mod_evaluate(module_id);
//             let await_result = js_rt
//                 .run_event_loop(deno_core::PollEventLoopOptions::default())
//                 .await;
//             if await_result.is_err() {
//                 return;
//             }
//             let result = result.await;
//             if result.is_err() {
//                 return;
//             }

//             let module_namespace = js_rt.get_module_namespace(module_id);
//             if module_namespace.is_err() {
//                 return;
//             }
//             let module_namespace = module_namespace.unwrap();

//             /*
//             // Call the provided export from the module
//             let result = {
//                 let mut js_scope = js_runtime.handle_scope();
//                 let mut js_scope = deno_core::v8::TryCatch::new(&mut js_scope);

//                 let module_namespace = deno_core::v8::Local::new(&mut js_scope, module_namespace);

//                 let export_key = deno_core::v8::String::new(&mut js_scope, &export)
//                     .context("failed to create V8 string")?;
//                 let export_value = module_namespace
//                     .get(&mut js_scope, export_key.into())
//                     .with_context(|| format!("expected module to have an export named {export}"))?;
//                 let export_value: deno_core::v8::Local<deno_core::v8::Function> =
//                     export_value
//                         .try_into()
//                         .with_context(|| format!("expected export named {export} to be a function"))?;

//                 tracing::debug!(%main_module, %export, "running exported function");

//                 let result = export_value.call(&mut js_scope, module_namespace.into(), &[]);

//                 let result = match result {
//                     Some(result) => result,
//                     None => {
//                         if let Some(exception) = js_scope.exception() {
//                             return Err(anyhow::anyhow!(
//                                 deno_core::error::JsError::from_v8_exception(&mut js_scope, exception)
//                             ))
//                             .with_context(|| format!("error when calling {export}"));
//                         } else {
//                             anyhow::bail!("unknown error when calling {export}");
//                         }
//                     }
//                 };
//                 deno_core::v8::Global::new(&mut js_scope, result)
//             };
//              */
//             let result = {
//                 let mut js_scope = js_rt.handle_scope();
//                 let mut js_scope = deno_core::v8::TryCatch::new(&mut js_scope);
//                 let module_namespace = deno_core::v8::Local::new(&mut js_scope, module_namespace);
//                 let export_key = deno_core::v8::String::new(&mut js_scope, "default")
//                     .context("failed to create V8 string");
//                 if export_key.is_err() {
//                     return;
//                 }
//                 let export_key = export_key.unwrap();
//                 let export_value = module_namespace
//                     .get(&mut js_scope, export_key.into())
//                     .with_context(|| format!("expected module to have an export named {export}"));
//                 tracing::debug!(%to_eval_module, %export, "running exported function");
//                 if export_value.is_err() {
//                     return;
//                 }

//                 let export_value = export_value.unwrap();
//                 let export_value: deno_core::v8::Local<deno_core::v8::Function> = export_value
//                     .try_into()
//                     .with_context(|| format!("expected export named {export} to be a function"));
//             };
//         });
//     });

//     Ok(())
// }
