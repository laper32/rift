pub mod dir;
pub mod manifest;
mod runtime;
mod workspace;
mod errors;

pub fn init() -> bool {
    runtime::init();
    true
}

pub fn shutdown() {
    runtime::shutdown();
}
