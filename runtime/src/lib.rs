use engine::errors::RiftResult;
use runtime::ScriptRuntime;

mod forward;
mod loader;
mod runtime;
mod shared;
mod specifier;

pub async fn init() -> RiftResult<()> {
    let result = ScriptRuntime::instance().bootstrap().await;
    // if result.is_err() {
    //     return result;
    // }
    // let result = ScriptRuntime::instance().runtime_init();
    result
}

pub fn shutdown() -> RiftResult<()> {
    ScriptRuntime::instance().runtime_shutdown()
}
