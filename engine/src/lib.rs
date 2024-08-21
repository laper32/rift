use workspace::Workspace;

pub mod dir;
pub mod events;
mod manifest;
mod package;
pub mod paths;
mod runtime;
mod schema;
pub mod task;
pub mod util;
mod workspace;

pub fn init() -> bool {
    let cwd = std::env::current_dir().unwrap();
    let ws = Workspace::new(&cwd);
    println!("Workspace root: {:?}", ws.root());
    println!("Workspace root manifest: {:?}", ws.root_manifest());
    true
}

pub fn shutdown() {
    runtime::shutdown();
}
