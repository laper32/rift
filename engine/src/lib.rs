pub mod dir;
mod errors;
mod runtime;
mod workspace;
mod package;
mod schema;
mod manifest;

pub fn init() -> bool {
    runtime::init();
    true
}

pub fn shutdown() {
    runtime::shutdown();
}
