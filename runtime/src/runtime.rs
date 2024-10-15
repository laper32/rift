use std::{path::Path, rc::Rc, sync::Arc};

use deno_runtime::{
    deno_core::{FsModuleLoader, ModuleSpecifier},
    deno_fs::RealFs,
    deno_permissions::PermissionsContainer,
    permissions::RuntimePermissionDescriptorParser,
    worker::{WorkerOptions, WorkerServiceOptions},
};

struct ScriptRuntime {
    worker: deno_runtime::worker::MainWorker,
    tokio: tokio::runtime::Runtime,
}

impl ScriptRuntime {
    fn new() -> Self {
        let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/bootstrap.js");
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
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<ScriptRuntime> =
            once_cell::sync::Lazy::new(|| ScriptRuntime::new());
        unsafe { &mut *INSTANCE }
    }
}
