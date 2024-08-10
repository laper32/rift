pub mod dir;
pub mod manifest;
mod runtime;
mod workspace;

pub fn init() -> bool {
    runtime::init();
    true
}

pub fn shutdown() {
    runtime::shutdown();
}
