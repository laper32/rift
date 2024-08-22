use std::path::PathBuf;

use dir::PathIdentity;
use workspace::Workspace;

mod blob;
pub mod dir;
pub mod events;
mod fs;
pub mod hash;
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

pub struct Engine {
    installation_path: PathBuf,
    user_path: PathBuf,
    project_path: PathBuf,
    // pub home: PathBuf,
}

impl Engine {
    fn new() -> Self {
        Self {
            installation_path: PathIdentity::get_rift_path(PathIdentity::Installation).into(),
            user_path: PathIdentity::get_rift_path(PathIdentity::UserProfile).into(),
            project_path: PathIdentity::get_rift_path(PathIdentity::Project).into(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<Engine> =
            once_cell::sync::Lazy::new(|| Engine::new());
        unsafe { &mut *INSTANCE }
    }
}
