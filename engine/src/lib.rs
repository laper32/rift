use std::path::{Path, PathBuf};

use anyhow::Context;
use lazycell::LazyCell;
use util::{
    errors::RiftResult,
    fs::{canonicalize_path, NON_INSTALLATION_PATH_NAME},
};
use workspace::Workspace;

mod manifest;
mod package;
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
    // runtime::shutdown();
}

pub struct Rift {
    /// rift.exe的路径
    rift_exe: LazyCell<PathBuf>,
}

/// Returns the absolute path of where the given executable is located based
/// on searching the `PATH` environment variable.
///
/// Returns an error if it cannot be found.
pub fn resolve_executable(exec: &Path) -> RiftResult<PathBuf> {
    if exec.components().count() == 1 {
        let paths = std::env::var_os("PATH").ok_or_else(|| anyhow::format_err!("no PATH"))?;
        let candidates = std::env::split_paths(&paths).flat_map(|path| {
            let candidate = path.join(&exec);
            let with_exe = if std::env::consts::EXE_EXTENSION.is_empty() {
                None
            } else {
                Some(candidate.with_extension(std::env::consts::EXE_EXTENSION))
            };
            core::iter::once(candidate).chain(with_exe)
        });
        for candidate in candidates {
            if candidate.is_file() {
                return Ok(candidate);
            }
        }

        anyhow::bail!("no executable for `{}` found in PATH", exec.display())
    } else {
        Ok(exec.into())
    }
}

impl Rift {
    fn new() -> Self {
        Self {
            rift_exe: LazyCell::new(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<Rift> =
            once_cell::sync::Lazy::new(|| Rift::new());
        unsafe { &mut *INSTANCE }
    }

    // Windows上是按照一个完整的包来处理的，换句话说rift.exe一定会在/bin里面。。。
    // Linux的话，如果你是直接把它放在/usr/bin里面的话，这个就没用了
    pub fn installation_path(&self) -> RiftResult<&Path> {
        self.rift_exe().map(|exe| {
            exe // ${InstallationPath}/bin/rift.exe
                .parent() // ${InstallationPath}/bin
                .unwrap()
                .parent() // ${InstallationPath}
                .unwrap()
        })
    }

    // 用户目录, aka: ~/.rift
    pub fn home_path(&self) -> RiftResult<PathBuf> {
        Ok(homedir::my_home()
            .unwrap()
            .unwrap()
            .join(NON_INSTALLATION_PATH_NAME))
    }

    pub fn rift_exe(&self) -> RiftResult<&Path> {
        self.rift_exe
            .try_borrow_with(|| {
                let from_env = || -> RiftResult<PathBuf> {
                    // Try re-using the `rift` set in the environment already. This allows
                    // commands that use Cargo as a library to inherit (via `rift <subcommand>`)
                    // or set (by setting `$RIFT`) a correct path to `rift` when the current exe
                    // is not actually cargo (e.g., `rift-*` binaries, Valgrind, `ld.so`, etc.).
                    let exe = canonicalize_path(
                        std::env::var_os("RIFT")
                            .map(PathBuf::from)
                            .ok_or_else(|| anyhow::anyhow!("$RIFT not set"))?,
                    )?;
                    Ok(exe)
                };
                fn from_current_exe() -> RiftResult<PathBuf> {
                    // Try fetching the path to `rift` using `env::current_exe()`.
                    // The method varies per operating system and might fail; in particular,
                    // it depends on `/proc` being mounted on Linux, and some environments
                    // (like containers or chroots) may not have that available.
                    let exe = canonicalize_path(std::env::current_exe()?)?;
                    Ok(exe)
                }

                fn from_argv() -> RiftResult<PathBuf> {
                    // Grab `argv[0]` and attempt to resolve it to an absolute path.
                    // If `argv[0]` has one component, it must have come from a `PATH` lookup,
                    // so probe `PATH` in that case.
                    // Otherwise, it has multiple components and is either:
                    // - a relative path (e.g., `./rift`, `target/debug/rift`), or
                    // - an absolute path (e.g., `/usr/local/bin/rift`).
                    // In either case, `Path::canonicalize` will return the full absolute path
                    // to the target if it exists.
                    let argv0 = std::env::args_os()
                        .map(PathBuf::from)
                        .next()
                        .ok_or_else(|| anyhow::anyhow!("no argv[0]"))?;
                    resolve_executable(&argv0)
                }
                let exe = from_env()
                    .or_else(|_| from_current_exe())
                    .or_else(|_| from_argv())
                    .context("couldn't get the path to cargo executable")?;
                Ok(exe)
            })
            .map(AsRef::as_ref)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_rift_exe() {
        println!("{:?}", super::Rift::instance().rift_exe().unwrap());
        println!(
            "{:?}",
            url::Url::from_file_path(super::Rift::instance().rift_exe().unwrap())
                .unwrap()
                .path()
        )
        // println!(
        //     "{:?}",
        //     super::Engine::instance().installation_path().unwrap()
        // );
    }
}
