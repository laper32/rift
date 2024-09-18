use std::path::{Path, PathBuf};

use anyhow::Context;
use deno_core::extension;
use lazycell::LazyCell;
use manifest::EitherManifest;

use util::{
    errors::RiftResult,
    fs::{canonicalize_path, NON_INSTALLATION_PATH_NAME},
};

mod manifest;
pub mod plsys;
mod runtime;
mod schema;
pub mod task;
pub mod util;
mod workspace;

#[derive(Debug)]
pub struct CurrentEvaluatingPackage {
    manifest: EitherManifest,
    manifest_path: PathBuf,
}

impl CurrentEvaluatingPackage {
    pub fn new(manifest: EitherManifest, manifest_path: PathBuf) -> Self {
        Self {
            manifest,
            manifest_path,
        }
    }
    pub fn manifest(&self) -> &EitherManifest {
        &self.manifest
    }
    pub fn manifest_path(&self) -> &PathBuf {
        &self.manifest_path
    }
}
fn setup_panic_hook() {
    // This function does two things inside of the panic hook:
    // - Tokio does not exit the process when a task panics, so we define a custom
    //   panic hook to implement this behaviour.
    // - We print a message to stderr to indicate that this is a bug in Deno, and
    //   should be reported to us.
    let orig_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        eprintln!("\n============================================================");
        eprintln!("Detected panic in Rift.");
        eprintln!("Report this at https://github.com/laper32/rift/issues/new.");
        eprintln!("If you can reliably reproduce this panic, include the");
        eprintln!("reproduction steps and re-run with the RUST_BACKTRACE=1 env");
        eprintln!("var set and include the backtrace in your report.");
        eprintln!();
        eprintln!(
            "Platform: {} {}",
            std::env::consts::OS,
            std::env::consts::ARCH
        );
        // todo: print rift version
        eprintln!("Args: {:?}", std::env::args().collect::<Vec<_>>());
        orig_hook(panic_info);
        std::process::exit(1);
    }));
}

pub fn main() {
    setup_panic_hook();

    init();
    shutdown();
}

pub fn init() -> bool {
    runtime::init();
    true
}

pub fn shutdown() {
    runtime::shutdown();
}

/// Returns the canonicalized absolute path of where the given executable is located based
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
                return canonicalize_path(candidate);
            }
        }

        anyhow::bail!("no executable for `{}` found in PATH", exec.display())
    } else {
        canonicalize_path(exec)
    }
}

pub struct Rift {
    /// rift.exe的路径
    rift_exe: LazyCell<PathBuf>,
    current_evaluating_package: Option<CurrentEvaluatingPackage>,
}

impl Rift {
    fn new() -> Self {
        Self {
            rift_exe: LazyCell::new(),
            current_evaluating_package: None,
            // current_evaluating_package: Mutex::new(None),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<Rift> =
            once_cell::sync::Lazy::new(|| Rift::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn set_current_evaluating_package(&mut self, package: CurrentEvaluatingPackage) {
        self.current_evaluating_package = Some(package);
    }

    pub fn get_current_evaluating_package(&self) -> &Option<CurrentEvaluatingPackage> {
        &self.current_evaluating_package
    }

    // Windows上是按照一个完整的包来处理的，换句话说rift.exe一定会在/bin里面。。。
    // Linux的话，如果你是直接把它放在/usr/bin里面的话，这个就没用了
    pub fn installation_path(&self) -> RiftResult<PathBuf> {
        let installation_path = self.rift_exe()?.parent().unwrap().parent().unwrap();
        canonicalize_path(installation_path)
    }

    // 用户目录, aka: ~/.rift
    pub fn home_path(&self) -> RiftResult<PathBuf> {
        match homedir::my_home() {
            Ok(path) => match path {
                Some(path) => {
                    let path = path.join(NON_INSTALLATION_PATH_NAME);
                    match canonicalize_path(path) {
                        Ok(path) => Ok(path),
                        Err(e) => Err(e).with_context(|| {
                            format!("Failed to get home path. Rift installation may be corrupted.")
                        }),
                    }
                }
                None => anyhow::bail!("Homedir is None."),
            },
            Err(_) => anyhow::bail!("Trying to get home directory"),
        }
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

mod ops {
    use deno_core::{error::AnyError, op2};

    use crate::Rift;

    #[op2]
    #[string]
    pub(super) fn get_rift_exe() -> Result<String, AnyError> {
        match Rift::instance().rift_exe() {
            Ok(path) => match path.to_str() {
                Some(path) => Ok(path.to_string()),
                None => anyhow::bail!("Failed to convert path to string."),
            },
            Err(e) => Err(e),
        }
    }

    #[op2]
    #[string]
    pub(super) fn get_home_path() -> Result<String, AnyError> {
        match Rift::instance().home_path() {
            Ok(path) => match path.to_str() {
                Some(path) => Ok(path.to_string()),
                None => anyhow::bail!("Failed to convert path to string."),
            },
            Err(e) => Err(e),
        }
    }

    #[op2]
    #[string]
    pub(super) fn get_installation_path() -> Result<String, AnyError> {
        match Rift::instance().installation_path() {
            Ok(path) => match path.to_str() {
                Some(path) => Ok(path.to_string()),
                None => anyhow::bail!("Failed to convert path to string."),
            },
            Err(e) => Err(e),
        }
    }
}

extension! {
    rift,
    ops = [
        ops::get_rift_exe,
        ops::get_home_path,
        ops::get_installation_path
    ]
}

#[cfg(test)]
mod test {

    #[test]
    fn test_necessary_paths() {
        println!("{:?}", super::Rift::instance().rift_exe().unwrap());
        println!("{:?}", super::Rift::instance().installation_path().unwrap());
        println!("{:?}", super::Rift::instance().home_path().unwrap());
        // println!("{:?}", super::Rift::instance().home_path());
    }
}
