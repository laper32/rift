use std::{
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};

use anyhow::Context;
use deno_ast::ModuleSpecifier;
use deno_core::{
    v8::{self, Value},
    Extension, RuntimeOptions,
};
use deno_runtime::{
    deno_fs::RealFs,
    permissions::RuntimePermissionDescriptorParser,
    worker::{WorkerOptions, WorkerServiceOptions},
};
use engine::{shared::errors::RiftResult, ENGINE_SNAPSHOT};

use crate::loader;

fn init_ops() -> Vec<Extension> {
    let mut ret: Vec<Extension> = Vec::new();
    for op in engine::init_ops() {
        ret.push(op);
    }
    ret
}

pub struct ScriptRuntime {
    main_worker: deno_runtime::worker::MainWorker,
    // js_runtime: deno_core::JsRuntime,
    tokio: tokio::runtime::Runtime,
    runtime_init_fn: Option<v8::Global<v8::Function>>,
    runtime_shutdown_fn: Option<v8::Global<v8::Function>>,
}

impl ScriptRuntime {
    fn new() -> Self {
        let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("js/dist/index.js");
        let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();
        let fs = Arc::new(RealFs);
        let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));
        /*

        Self {
            main_worker: deno_runtime::worker::MainWorker::bootstrap_from_options(
                main_module.clone(),
                WorkerServiceOptions {
                    blob_store: Default::default(),
                    broadcast_channel: Default::default(),
                    feature_checker: Default::default(),
                    fs,
                    module_loader: Rc::new(FsModuleLoader),
                    node_services: None,
                    npm_process_state_provider: None,
                    permissions: PermissionsContainer::allow_all(permission_desc_parser),
                    root_cert_store_provider: Default::default(),
                    shared_array_buffer_store: Default::default(),
                    compiled_wasm_module_store: None,
                    v8_code_cache: Default::default(),
                },
                WorkerOptions {
                    extensions: vec![],
                    ..Default::default()
                },
            ),
        }
         */

        Self {
            main_worker: deno_runtime::worker::MainWorker::bootstrap_from_options(
                main_module,
                WorkerServiceOptions {
                    blob_store: todo!(),
                    broadcast_channel: todo!(),
                    feature_checker: todo!(),
                    fs: todo!(),
                    module_loader: todo!(),
                    node_services: todo!(),
                    npm_process_state_provider: todo!(),
                    permissions: todo!(),
                    root_cert_store_provider: todo!(),
                    shared_array_buffer_store: todo!(),
                    compiled_wasm_module_store: todo!(),
                    v8_code_cache: todo!(),
                },
                WorkerOptions {
                    extensions: init_ops(),
                    ..Default::default()
                },
            ),

            tokio: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            runtime_init_fn: None,
            runtime_shutdown_fn: None,
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<ScriptRuntime> =
            once_cell::sync::Lazy::new(|| ScriptRuntime::new());
        unsafe { &mut *INSTANCE }
    }

    // pub fn js_runtime(&mut self) -> &'static mut deno_core::JsRuntime {
    //     &mut Self::instance().js_runtime
    // }

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
    pub async fn load_bootstarp(&mut self) -> RiftResult<()> {
        // let entry_path: deno_core::ModuleSpecifier = "rift:dist/index.js"
        //     .parse()
        //     .expect("failed to parse specifier");
        // let module_id = self.js_runtime.load_main_es_module(&entry_path).await?;
        // self.js_runtime.mod_evaluate(module_id).await?;
        // let module_namespace = self.js_runtime.get_module_namespace(module_id)?;
        // let mut scope = self.js_runtime.handle_scope();
        // let mut scope = v8::TryCatch::new(&mut scope);
        // let module_namespace = v8::Local::new(&mut scope, module_namespace);
        // let bootstrap_key =
        //     v8::String::new(&mut scope, "bootstrap").context("Failed to create v8 string")?;
        // let bootstrap_ns = module_namespace
        //     .get(&mut scope, bootstrap_key.into())
        //     .unwrap()
        //     .to_object(&mut scope)
        //     .unwrap();
        // tracing::info!("{:?}", bootstrap_ns);

        // let init_fn = {
        //     let init_fn_str = v8::String::new(&mut scope, "init")
        //         .context("Failed to create v8 string")
        //         .unwrap();
        //     let init_fn = bootstrap_ns.get(&mut scope, init_fn_str.into()).unwrap();
        //     let init_fn = v8::Local::<v8::Function>::try_from(init_fn).unwrap();
        //     let init_fn = v8::Global::new(&mut scope, init_fn);
        //     init_fn
        // };
        // self.runtime_init_fn = Some(init_fn);

        // let undefined = v8::undefined(&mut scope);
        // let result = init_fn
        //     .call(&mut scope, undefined.into(), &[])
        //     .with_context(|| format!("Failed to call init, why?"));
        // println!("{:?}", result);

        // let mut js_scope = self.js_runtime.handle_scope();
        // let mut js_scope = deno_core::v8::TryCatch::new(&mut js_scope);
        // let module_namespace = deno_core::v8::Local::new(&mut js_scope, module_namespace);
        // let bootstrap_key = deno_core::v8::String::new(&mut js_scope, "bootstrap")
        //     .context("Failed to create v8 string")?;
        // let bootstrap_ns = module_namespace
        //     .get(&mut js_scope, bootstrap_key.into())
        //     .unwrap()
        //     .to_object(&mut js_scope)
        //     .unwrap();

        // let init_fn_str = deno_core::v8::String::new(&mut js_scope, "init")
        //     .context("Failed to create v8 string")?;
        // let init_fn = bootstrap_ns.get(&mut js_scope, init_fn_str.into()).unwrap();
        // let init_fn = deno_core::v8::Local::<v8::Function>::try_from(init_fn).unwrap();
        // let init_fn = deno_core::v8::Global::new(&mut js_scope, init_fn);
        // self.runtime_init_fn = Some(init_fn);

        // let shutdown_fn_str = deno_core::v8::String::new(&mut js_scope, "shutdown")
        //     .context("Failed to create v8 string")?;
        // let shutdown_fn = bootstrap_ns
        //     .get(&mut js_scope, shutdown_fn_str.into())
        //     .unwrap();
        // let shutdown_fn = deno_core::v8::Local::<v8::Function>::try_from(shutdown_fn).unwrap();
        // let shutdown_fn = deno_core::v8::Global::new(&mut js_scope, shutdown_fn);
        // self.runtime_shutdown_fn = Some(shutdown_fn);

        Ok(())
    }

    pub fn bootstrap_init(&mut self) -> RiftResult<()> {
        // if self.runtime_init_fn.is_none() {
        //     return Err(anyhow::anyhow!("No bootstrap init function found"));
        // }
        // let init_fn = self.runtime_init_fn.as_ref().unwrap();
        // let mut scope = self.js_runtime.handle_scope();
        // let undefined: v8::Local<Value> = v8::undefined(&mut scope).into();
        // // let on_load_fn = v8::Local::new(&mut scope, on_load_fn_ref);
        // let init_fn = v8::Local::new(&mut scope, init_fn);
        // let result = init_fn.call(&mut scope, undefined.into(), &[]).unwrap();
        // let result: bool = result.to_boolean(&mut scope).boolean_value(&mut scope);
        // if result {
        //     return Ok(());
        // } else {
        //     return Err(anyhow::anyhow!("bootstrap init failed"));
        // }
        Ok(())
    }
    pub fn bootstrap_shutdown(&mut self) -> RiftResult<()> {
        // if self.runtime_shutdown_fn.is_none() {
        //     return Err(anyhow::anyhow!("No bootstrap shutdown function found"));
        // }
        // let shutdown_fn = self.runtime_shutdown_fn.as_ref().unwrap();
        // let mut scope = self.js_runtime.handle_scope();
        // let undefined: v8::Local<Value> = v8::undefined(&mut scope).into();
        // // let on_load_fn = v8::Local::new(&mut scope, on_load_fn_ref);
        // let shutdown_fn = v8::Local::new(&mut scope, shutdown_fn);
        // let _ = shutdown_fn.call(&mut scope, undefined.into(), &[]);
        Ok(())
    }
}
