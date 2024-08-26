pub mod errors;
pub mod fs;
pub mod os;

use std::{
    env,
    ffi::OsString,
    fs::read_dir,
    io::{self, ErrorKind},
    path::PathBuf,
};

/// 绝大多数的情况下，Cargo项目的根目录一定会有 **Cargo.lock**
pub fn get_cargo_project_root() -> io::Result<PathBuf> {
    let path = env::current_dir()?;
    let mut path_ancestors = path.as_path().ancestors();

    while let Some(p) = path_ancestors.next() {
        let has_cargo = read_dir(p)?
            .into_iter()
            .any(|p| p.unwrap().file_name() == OsString::from("Cargo.lock"));
        if has_cargo {
            let from = PathBuf::from(p);
            return Ok(from);
        }
    }
    Err(io::Error::new(
        ErrorKind::NotFound,
        "Ran out of places to find Cargo.toml",
    ))
}
