pub mod dir;
mod errors;
pub mod manifest;
mod runtime;
mod workspace;
mod package;

pub fn init() -> bool {
    runtime::init();
    true
}

pub fn shutdown() {
    runtime::shutdown();
}
