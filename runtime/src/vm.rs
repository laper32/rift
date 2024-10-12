use std::{path::PathBuf, rc::Rc};

use deno_core::{Extension, RuntimeOptions};
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
    js_runtime: deno_core::JsRuntime,
    tokio: tokio::runtime::Runtime,
    current_evaluating_script_path: Option<PathBuf>,
}

impl ScriptRuntime {
    fn new() -> Self {
        Self {
            js_runtime: deno_core::JsRuntime::new(RuntimeOptions {
                module_loader: Some(Rc::new(loader::TsModuleLoader)),
                extensions: init_ops(),
                startup_snapshot: Some(ENGINE_SNAPSHOT),
                ..Default::default()
            }),
            tokio: tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap(),
            current_evaluating_script_path: None,
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<ScriptRuntime> =
            once_cell::sync::Lazy::new(|| ScriptRuntime::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn js_runtime(&mut self) -> &'static mut deno_core::JsRuntime {
        &mut Self::instance().js_runtime
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
