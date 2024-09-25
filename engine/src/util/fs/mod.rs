pub mod as_posix;

use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Context;
use as_posix::PathBufExt;

use crate::util::path::make_rand_temp_file_path;

use super::errors::RiftResult;

pub const NON_INSTALLATION_PATH_NAME: &str = ".rift";

/// Similar to `std::fs::canonicalize()` but do some extra works on Windows:
/// 1. Remove UNC prefix.
/// 2. Convert '\' to '/'.
#[inline]
pub fn canonicalize_path<P: AsRef<Path>>(path: P) -> RiftResult<PathBuf> {
    let canonicalized_path = deno_core::strip_unc_prefix(path.as_ref().canonicalize()?);
    let result = canonicalized_path
        .as_posix()
        .ok_or_else(|| anyhow::format_err!("Failed to canonicalize path: {:?}", path.as_ref()));
    match result {
        Ok(result) => {
            let path = result.into_owned();
            Ok(PathBuf::from(path))
        }
        Err(e) => Err(e),
    }
}

/// Writes the file to the file system at a temporary path, then
/// renames it to the destination in a single sys call in order
/// to never leave the file system in a corrupted state.
///
/// This also handles creating the directory if a NotFound error
/// occurs.
pub fn atomic_write_file_with_retries<T: AsRef<[u8]>>(
    file_path: &Path,
    data: T,
    mode: u32,
) -> std::io::Result<()> {
    let mut count = 0;
    loop {
        match atomic_write_file(file_path, data.as_ref(), mode) {
            Ok(()) => return Ok(()),
            Err(err) => {
                if count >= 5 {
                    // too many retries, return the error
                    return Err(err);
                }
                count += 1;
                let sleep_ms = std::cmp::min(50, 10 * count);
                std::thread::sleep(std::time::Duration::from_millis(sleep_ms));
            }
        }
    }
}

/// Writes the file to the file system at a temporary path, then
/// renames it to the destination in a single sys call in order
/// to never leave the file system in a corrupted state.
///
/// This also handles creating the directory if a NotFound error
/// occurs.
fn atomic_write_file<T: AsRef<[u8]>>(file_path: &Path, data: T, mode: u32) -> std::io::Result<()> {
    fn atomic_write_file_raw(
        temp_file_path: &Path,
        file_path: &Path,
        data: &[u8],
        mode: u32,
    ) -> std::io::Result<()> {
        write_file(temp_file_path, data, mode)?;
        std::fs::rename(temp_file_path, file_path).map_err(|err| {
            // clean up the created temp file on error
            let _ = std::fs::remove_file(temp_file_path);
            err
        })
    }

    fn inner(file_path: &Path, data: &[u8], mode: u32) -> std::io::Result<()> {
        let temp_file_path = make_rand_temp_file_path(file_path);

        if let Err(write_err) = atomic_write_file_raw(&temp_file_path, file_path, data, mode) {
            if write_err.kind() == ErrorKind::NotFound {
                let parent_dir_path = file_path.parent().unwrap();
                match std::fs::create_dir_all(parent_dir_path) {
                    Ok(()) => {
                        return atomic_write_file_raw(&temp_file_path, file_path, data, mode)
                            .map_err(|err| add_file_context_to_err(file_path, err));
                    }
                    Err(create_err) => {
                        if !parent_dir_path.exists() {
                            return Err(std::io::Error::new(
                                create_err.kind(),
                                format!(
                                    "{:#} (for '{}')\nCheck the permission of the directory.",
                                    create_err,
                                    parent_dir_path.display()
                                ),
                            ));
                        }
                    }
                }
            }
            return Err(add_file_context_to_err(file_path, write_err));
        }
        Ok(())
    }

    inner(file_path, data.as_ref(), mode)
}

/// Creates a std::fs::File handling if the parent does not exist.
pub fn create_file(file_path: &Path) -> std::io::Result<std::fs::File> {
    match std::fs::File::create(file_path) {
        Ok(file) => Ok(file),
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                let parent_dir_path = file_path.parent().unwrap();
                match std::fs::create_dir_all(parent_dir_path) {
                    Ok(()) => {
                        return std::fs::File::create(file_path)
                            .map_err(|err| add_file_context_to_err(file_path, err));
                    }
                    Err(create_err) => {
                        if !parent_dir_path.exists() {
                            return Err(std::io::Error::new(
                                create_err.kind(),
                                format!(
                                    "{:#} (for '{}')\nCheck the permission of the directory.",
                                    create_err,
                                    parent_dir_path.display()
                                ),
                            ));
                        }
                    }
                }
            }
            Err(add_file_context_to_err(file_path, err))
        }
    }
}

fn add_file_context_to_err(file_path: &Path, err: std::io::Error) -> std::io::Error {
    std::io::Error::new(
        err.kind(),
        format!("{:#} (for '{}')", err, file_path.display()),
    )
}

pub fn write_file<T: AsRef<[u8]>>(filename: &Path, data: T, mode: u32) -> std::io::Result<()> {
    write_file_2(filename, data, true, mode, true, false)
}

pub fn write_file_2<T: AsRef<[u8]>>(
    filename: &Path,
    data: T,
    update_mode: bool,
    mode: u32,
    is_create: bool,
    is_append: bool,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .read(false)
        .write(true)
        .append(is_append)
        .truncate(!is_append)
        .create(is_create)
        .open(filename)?;

    if update_mode {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = mode & 0o777;
            let permissions = PermissionsExt::from_mode(mode);
            file.set_permissions(permissions)?;
        }
        #[cfg(not(unix))]
        let _ = mode;
    }

    file.write_all(data.as_ref())
}

/// Asynchronously removes a directory and all its descendants, but does not error
/// when the directory does not exist.
pub async fn remove_dir_all_if_exists(path: &Path) -> std::io::Result<()> {
    let result = tokio::fs::remove_dir_all(path).await;
    match result {
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
        _ => result,
    }
}

mod clone_dir_imp {
    use crate::util::errors::RiftResult;

    #[cfg(target_vendor = "apple")]
    mod apple {
        use std::path::Path;

        use crate::util::{errors::RiftResult, fs::copy_dir_recursive};

        fn clonefile(from: &Path, to: &Path) -> std::io::Result<()> {
            let from = std::ffi::CString::new(from.as_os_str().as_bytes())?;
            let to = std::ffi::CString::new(to.as_os_str().as_bytes())?;
            // SAFETY: `from` and `to` are valid C strings.
            let ret = unsafe { libc::clonefile(from.as_ptr(), to.as_ptr(), 0) };
            if ret != 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        }
        pub fn clone_dir_recursive(from: &Path, to: &Path) -> RiftResult<()> {
            if let Some(parent) = to.parent() {
                std::fs::create_dir_all(parent)?;
            }
            // Try to clone the whole directory
            if let Err(err) = clonefile(from, to) {
                if err.kind() != std::io::ErrorKind::AlreadyExists {
                    tracing::warn!(
                        "Failed to clone dir {:?} to {:?} via clonefile: {}",
                        from,
                        to,
                        err
                    );
                }
                // clonefile won't overwrite existing files, so if the dir exists
                // we need to handle it recursively.
                copy_dir_recursive(from, to)?;
            }

            Ok(())
        }
    }

    #[cfg(target_vendor = "apple")]
    pub(super) use apple::clone_dir_recursive;

    pub(super) fn clone_dir_recursive(
        from: &std::path::Path,
        to: &std::path::Path,
    ) -> RiftResult<()> {
        if let Err(e) = super::hard_link_dir_recursive(from, to) {
            tracing::debug!("Failed to hard link dir {:?} to {:?}: {}", from, to, e);
            super::copy_dir_recursive(from, to)?;
        }
        Ok(())
    }
}

pub fn clone_dir_recursive(from: &Path, to: &Path) -> RiftResult<()> {
    clone_dir_imp::clone_dir_recursive(from, to)
}

/// Copies a directory to another directory.
///
/// Note: Does not handle symlinks.
pub fn copy_dir_recursive(from: &Path, to: &Path) -> RiftResult<()> {
    std::fs::create_dir_all(to).with_context(|| format!("Creating {}", to.display()))?;
    let read_dir =
        std::fs::read_dir(from).with_context(|| format!("Reading {}", from.display()))?;

    for entry in read_dir {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let new_from = from.join(entry.file_name());
        let new_to = to.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_recursive(&new_from, &new_to)
                .with_context(|| format!("Dir {} to {}", new_from.display(), new_to.display()))?;
        } else if file_type.is_file() {
            std::fs::copy(&new_from, &new_to).with_context(|| {
                format!("Copying {} to {}", new_from.display(), new_to.display())
            })?;
        }
    }

    Ok(())
}

/// Hardlinks the files in one directory to another directory.
///
/// Note: Does not handle symlinks.
pub fn hard_link_dir_recursive(from: &Path, to: &Path) -> RiftResult<()> {
    std::fs::create_dir_all(to).with_context(|| format!("Creating {}", to.display()))?;
    let read_dir =
        std::fs::read_dir(from).with_context(|| format!("Reading {}", from.display()))?;

    for entry in read_dir {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let new_from = from.join(entry.file_name());
        let new_to = to.join(entry.file_name());

        if file_type.is_dir() {
            hard_link_dir_recursive(&new_from, &new_to)
                .with_context(|| format!("Dir {} to {}", new_from.display(), new_to.display()))?;
        } else if file_type.is_file() {
            // note: chance for race conditions here between attempting to create,
            // then removing, then attempting to create. There doesn't seem to be
            // a way to hard link with overwriting in Rust, but maybe there is some
            // way with platform specific code. The workaround here is to handle
            // scenarios where something else might create or remove files.
            if let Err(err) = std::fs::hard_link(&new_from, &new_to) {
                if err.kind() == ErrorKind::AlreadyExists {
                    if let Err(err) = std::fs::remove_file(&new_to) {
                        if err.kind() == ErrorKind::NotFound {
                            // Assume another process/thread created this hard link to the file we are wanting
                            // to remove then sleep a little bit to let the other process/thread move ahead
                            // faster to reduce contention.
                            std::thread::sleep(Duration::from_millis(10));
                        } else {
                            return Err(err).with_context(|| {
                                format!(
                                    "Removing file to hard link {} to {}",
                                    new_from.display(),
                                    new_to.display()
                                )
                            });
                        }
                    }

                    // Always attempt to recreate the hardlink. In contention scenarios, the other process
                    // might have been killed or exited after removing the file, but before creating the hardlink
                    if let Err(err) = std::fs::hard_link(&new_from, &new_to) {
                        // Assume another process/thread created this hard link to the file we are wanting
                        // to now create then sleep a little bit to let the other process/thread move ahead
                        // faster to reduce contention.
                        if err.kind() == ErrorKind::AlreadyExists {
                            std::thread::sleep(Duration::from_millis(10));
                        } else {
                            return Err(err).with_context(|| {
                                format!(
                                    "Hard linking {} to {}",
                                    new_from.display(),
                                    new_to.display()
                                )
                            });
                        }
                    }
                } else {
                    return Err(err).with_context(|| {
                        format!(
                            "Hard linking {} to {}",
                            new_from.display(),
                            new_to.display()
                        )
                    });
                }
            }
        }
    }

    Ok(())
}

/// Creates a new symlink to a directory on the filesystem.
///
/// It wraps `std::os::unix::fs::symlink` and `std::os::windows::fs::symlink_dir`.
///
/// # Limitations
///
/// For Windows users, please check the following message from the Rust documentation:
///
/// Windows treats symlink creation as a [privileged action][symlink-security],
/// therefore this function is likely to fail unless the user makes changes to
/// their system to permit symlink creation. Users can try enabling Developer
/// Mode, granting the `SeCreateSymbolicLinkPrivilege` privilege, or running
/// the process as an administrator.
///
/// [symlink-security]: https://docs.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/create-symbolic-links
pub fn symlink_dir(oldpath: &Path, newpath: &Path) -> Result<(), std::io::Error> {
    let err_mapper = |err: std::io::Error| {
        std::io::Error::new(
            err.kind(),
            format!(
                "{}, symlink '{}' -> '{}'",
                err,
                oldpath.display(),
                newpath.display()
            ),
        )
    };
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink(oldpath, newpath).map_err(err_mapper)?;
    }
    #[cfg(not(unix))]
    {
        use std::os::windows::fs::symlink_dir;
        symlink_dir(oldpath, newpath).map_err(err_mapper)?;
    }
    Ok(())
}

/// Gets the total size (in bytes) of a directory.
pub fn dir_size(path: &Path) -> std::io::Result<u64> {
    let entries = std::fs::read_dir(path)?;
    let mut result = 0;
    for entry in entries {
        let entry = entry?;
        result += match entry.metadata()? {
            data if data.is_dir() => dir_size(&entry.path())?,
            data => data.len(),
        };
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    #[test]
    fn test_canonical_path() {
        let path = r#"D:\workshop\projects\rift\sample\01_simple_target/Rift.toml"#;
        let path = super::canonicalize_path(path).unwrap();
        assert_eq!(
            path.to_str().unwrap(),
            "D:/workshop/projects/rift/sample/01_simple_target/Rift.toml"
        );
    }
}
