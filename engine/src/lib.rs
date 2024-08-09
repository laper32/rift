use module::ModuleManager;
pub mod dir;
pub mod ffi;
pub mod manifest;
mod module;
mod runtime;

fn init() -> bool {
    module::init();
    // runtime::init();
    true
}

fn shutdown() {}
