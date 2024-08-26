pub mod as_posix;

use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use as_posix::PathBufExt;

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
            // todo!()
        }
        Err(e) => anyhow::bail!(
            "Failed to canonicalize path: {:?}, error: {:?}",
            path.as_ref(),
            e
        ),
    }
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

pub fn write_file<T: AsRef<[u8]>>(filename: &Path, data: T, mode: u32) -> RiftResult<()> {
    fn write_file_impl<T: AsRef<[u8]>>(
        filename: &Path,
        data: T,
        update_mode: bool,
        mode: u32,
        is_create: bool,
        is_append: bool,
    ) -> RiftResult<()> {
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

        match file.write_all(data.as_ref()) {
            Ok(_) => Ok(()),
            Err(e) => anyhow::bail!(e),
        }
    }
    write_file_impl(filename, data, true, mode, true, false)
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
