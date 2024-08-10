use module::ModuleManager;
pub mod dir;
pub mod manifest;
mod runtime;
mod workspace;

fn init() -> bool {
    runtime::init();
    true
}

fn shutdown() {
    runtime::shutdown();
}
