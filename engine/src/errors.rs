use std::fmt;
use std::path::{Path, PathBuf};
use anyhow::Error;

pub type RiftResult<T> = anyhow::Result<T>;


pub struct ManifestError {
    cause: Error,
    manifest_path: PathBuf
}

impl ManifestError {
    pub fn new<E: Into<Error>>(cause: E, manifest_path: PathBuf) -> ManifestError {
        ManifestError {
            cause: cause.into(),
            manifest_path
        }
    }

    pub fn manifest_path(&self) -> &PathBuf {
        &self.manifest_path
    }
    /// Returns an iterator over the `ManifestError` chain of causes.
    ///
    /// So if this error was not caused by another `ManifestError` this will be empty.
    pub fn cause(&self) -> ManifestCauses<'_> {
        ManifestCauses { current: self }
    }
}

impl std::error::Error for ManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.source()
    }
}

impl fmt::Debug for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cause.fmt(f)
    }
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.cause.fmt(f)
    }
}

/// An iterator over the `ManifestError` chain of causes.
pub struct ManifestCauses<'a> {
    current: &'a ManifestError,
}

impl<'a> Iterator for ManifestCauses<'a> {
    type Item = &'a ManifestError;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = self.current.cause.downcast_ref()?;
        Some(self.current)
    }
}

impl<'a> ::std::iter::FusedIterator for ManifestCauses<'a> {}