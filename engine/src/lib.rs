pub mod dir;
mod errors;
mod manifest;
mod package;
mod runtime;
mod schema;
mod workspace;

pub fn init() -> bool {
    runtime::init();
    true
}

pub fn shutdown() {
    runtime::shutdown();
}
