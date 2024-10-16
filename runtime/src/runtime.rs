use std::{path::Path, rc::Rc, result, sync::Arc};

use anyhow::Context;
use deno_core::v8;
use deno_runtime::{
    deno_core::ModuleSpecifier,
    deno_fs::RealFs,
    deno_permissions::PermissionsContainer,
    permissions::RuntimePermissionDescriptorParser,
    worker::{WorkerOptions, WorkerServiceOptions},
};
use engine::errors::RiftResult;

use crate::loader::RuntimeModuleLoader;

pub struct ScriptRuntime {
    worker: deno_runtime::worker::MainWorker,
    tokio: tokio::runtime::Runtime,
    runtime_init_fn: Option<v8::Global<v8::Function>>,
    runtime_shutdown_fn: Option<v8::Global<v8::Function>>,
}

impl ScriptRuntime {
    fn new() -> Self {
        let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/index.js");
        let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();
        let fs = Arc::new(RealFs);
        let permission_desc_parser = Arc::new(RuntimePermissionDescriptorParser::new(fs.clone()));

        Self {
            tokio: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            worker: deno_runtime::worker::MainWorker::bootstrap_from_options(
                main_module.clone(),
                WorkerServiceOptions {
                    blob_store: Default::default(),
                    broadcast_channel: Default::default(),
                    feature_checker: Default::default(),
                    fs,
                    module_loader: Rc::new(RuntimeModuleLoader),
                    node_services: None,
                    npm_process_state_provider: None,
                    permissions: PermissionsContainer::allow_all(permission_desc_parser),
                    root_cert_store_provider: Default::default(),
                    shared_array_buffer_store: Default::default(),
                    compiled_wasm_module_store: None,
                    v8_code_cache: Default::default(),
                },
                WorkerOptions {
                    extensions: engine::init_ops(),
                    ..Default::default()
                },
            ),
            runtime_init_fn: None,
            runtime_shutdown_fn: None,
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<ScriptRuntime> =
            once_cell::sync::Lazy::new(|| ScriptRuntime::new());
        unsafe { &mut *INSTANCE }
    }

    pub(crate) fn runtime_init(&mut self) -> RiftResult<()> {
        if self.runtime_init_fn.is_none() {
            return Err(anyhow::anyhow!("No bootstrap init function found"));
        }
        let init_fn = self.runtime_init_fn.as_ref().unwrap();
        let mut scope = self.worker.js_runtime.handle_scope();
        let undefined: v8::Local<v8::Value> = v8::undefined(&mut scope).into();
        let init_fn = v8::Local::new(&mut scope, init_fn);
        let result = init_fn.call(&mut scope, undefined.into(), &[]).unwrap();
        let result: bool = result.to_boolean(&mut scope).boolean_value(&mut scope);
        if result {
            return Ok(());
        } else {
            return Err(anyhow::anyhow!("bootstrap init failed"));
        }
    }

    pub(crate) fn runtime_shutdown(&mut self) -> RiftResult<()> {
        if self.runtime_shutdown_fn.is_none() {
            return Err(anyhow::anyhow!("No bootstrap shutdown function found"));
        }
        let shutdown_fn = self.runtime_shutdown_fn.as_ref().unwrap();
        let mut scope = self.worker.js_runtime.handle_scope();
        let undefined: v8::Local<v8::Value> = v8::undefined(&mut scope).into();
        // let on_load_fn = v8::Local::new(&mut scope, on_load_fn_ref);
        let shutdown_fn = v8::Local::new(&mut scope, shutdown_fn);
        let _ = shutdown_fn.call(&mut scope, undefined.into(), &[]);
        Ok(())
    }

    pub(crate) async fn bootstrap(&mut self) -> RiftResult<()> {
        let result = self.bootstrap_impl().await;
        result
    }

    async fn bootstrap_impl(&mut self) -> RiftResult<()> {
        let entry_path: deno_core::ModuleSpecifier = "rift:dist/index.js"
            .parse()
            .expect("failed to parse specifier");
        let module_id = self.worker.preload_main_module(&entry_path).await?;
        self.worker.evaluate_module(module_id).await?;
        self.worker.run_event_loop(false).await?;
        let module_namespace = self.worker.js_runtime.get_module_namespace(module_id)?;
        let mut js_scope = self.worker.js_runtime.handle_scope();
        let mut js_scope = v8::TryCatch::new(&mut js_scope);
        let module_namespace = v8::Local::new(&mut js_scope, module_namespace);
        let bootstrap_key =
            v8::String::new(&mut js_scope, "bootstrap").context("Failed to create v8 string")?;
        let bootstrap_ns = module_namespace
            .get(&mut js_scope, bootstrap_key.into())
            .unwrap()
            .to_object(&mut js_scope)
            .unwrap();
        let init_fn_str =
            v8::String::new(&mut js_scope, "init").context("Failed to create v8 string")?;
        let init_fn = bootstrap_ns.get(&mut js_scope, init_fn_str.into()).unwrap();
        let init_fn = v8::Local::<v8::Function>::try_from(init_fn).unwrap();
        let init_fn = v8::Global::new(&mut js_scope, init_fn);
        self.runtime_init_fn = Some(init_fn);

        let shutdown_fn_str =
            v8::String::new(&mut js_scope, "shutdown").context("Failed to create v8 string")?;
        let shutdown_fn = bootstrap_ns
            .get(&mut js_scope, shutdown_fn_str.into())
            .unwrap();
        let shutdown_fn = v8::Local::<v8::Function>::try_from(shutdown_fn).unwrap();
        let shutdown_fn = v8::Global::new(&mut js_scope, shutdown_fn);
        self.runtime_shutdown_fn = Some(shutdown_fn);

        Ok(())
    }
}
