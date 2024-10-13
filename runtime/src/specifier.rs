use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;
use engine::shared::errors::RiftResult;
use relative_path::RelativePathBuf;

use crate::shared::RuntimeFiles;

/// 处理 `import` 用的(JS的那个`import * from "xxx";`)
/// 我们在下面的 `resolve`函数里处理相关逻辑

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum RiftImportSpecifier {
    /// 只处理如 '.', '..', '../', '/' 这种情况
    Local(String),

    /// Local之外的全是External。
    ///
    /// 例: import * as build from "pkg:rift.build";
    External(String),
}

impl std::str::FromStr for RiftImportSpecifier {
    type Err = anyhow::Error;

    fn from_str(specifier: &str) -> Result<Self, Self::Err> {
        if specifier == "."
            || specifier == ".."
            || specifier.starts_with("./")
            || specifier.starts_with("../")
            || specifier.starts_with('/')
        {
            Ok(Self::Local(specifier.to_string()))
        } else {
            Ok(Self::External(specifier.to_string()))
        }
    }
}

impl std::fmt::Display for RiftImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiftImportSpecifier::Local(specifier) => write!(f, "{}", specifier),
            RiftImportSpecifier::External(specifier) => write!(f, "{}", specifier),
        }
    }
}

/// A specifier for a Rift module, either from the filesystem or
/// from the internal runtime package. A module specifier can be converted
/// from/to a URL.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum RiftModuleSpecifier {
    Internal { subpath: RelativePathBuf }, // rift:xxx
    Package { pkg_name: String },          // pkg:xxx
    File { path: PathBuf },                // ./xxx.ts || ../xxx.ts || /xxx.ts
}

impl RiftModuleSpecifier {
    pub fn from_path(path: &Path) -> Self {
        Self::File {
            path: path.to_owned(),
        }
    }
}

impl TryFrom<&'_ url::Url> for RiftModuleSpecifier {
    type Error = anyhow::Error;

    fn try_from(value: &'_ url::Url) -> Result<Self, Self::Error> {
        match value.scheme() {
            "file" => {
                let path = value
                    .to_file_path()
                    .map_err(|_| anyhow::anyhow!("failed to convert specifier {value} to path"))?;
                Ok(Self::File { path })
            }
            "rift" => {
                anyhow::ensure!(!value.has_host(), "invalid specifier: {}", value);
                let subpath = RelativePathBuf::from(value.path().trim_start_matches('/'));
                Ok(Self::Internal { subpath })
            }
            "pkg" => {
                anyhow::ensure!(!value.has_host(), "invalid specifier: {}", value);
                let pkg_name = value.path().trim_start_matches('/').to_string();
                Ok(Self::Package { pkg_name })
            }
            _ => {
                anyhow::bail!("invalid scheme in specifier: {value}");
            }
        }
    }
}

impl TryFrom<url::Url> for RiftModuleSpecifier {
    type Error = anyhow::Error;

    fn try_from(value: url::Url) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl From<&'_ RiftModuleSpecifier> for url::Url {
    fn from(value: &'_ RiftModuleSpecifier) -> Self {
        match value {
            RiftModuleSpecifier::Internal { subpath } => {
                let mut url: url::Url = "rift:///".parse().expect("failed to build URL");
                url.set_path(subpath.as_str());
                url
            }
            RiftModuleSpecifier::Package { pkg_name } => {
                let mut url: url::Url = "pkg:///".parse().expect("failed to build URL");
                url.set_path(pkg_name.as_str());
                url
            }
            RiftModuleSpecifier::File { path } => {
                url::Url::from_file_path(path).expect("failed to build URL")
            }
        }
    }
}

impl From<RiftModuleSpecifier> for url::Url {
    fn from(value: RiftModuleSpecifier) -> Self {
        Self::from(&value)
    }
}

impl std::str::FromStr for RiftModuleSpecifier {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url: url::Url = s.parse()?;
        let specifier = url.try_into()?;
        Ok(specifier)
    }
}

impl std::fmt::Display for RiftModuleSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", url::Url::from(self))
    }
}

pub(crate) fn read_specifier_contents(specifier: &RiftModuleSpecifier) -> RiftResult<Arc<Vec<u8>>> {
    match specifier {
        RiftModuleSpecifier::Internal { subpath } => {
            let file = RuntimeFiles::get(subpath.as_str())
                .with_context(|| format!("Internal module \"{specifier}\" not found."))?;
            Ok(Arc::new(file.data.to_vec()))
        }
        RiftModuleSpecifier::Package { pkg_name } => todo!(),
        RiftModuleSpecifier::File { path } => todo!(),
    }
}

pub(crate) fn resolve(
    specifier: &RiftImportSpecifier,
    referrer: &RiftModuleSpecifier,
) -> RiftResult<RiftModuleSpecifier> {
    match referrer {
        RiftModuleSpecifier::Internal { subpath } => {
            let specifier_path = match specifier {
                RiftImportSpecifier::Local(specifier_path) => specifier_path,
                _ => anyhow::bail!("invalid specifier '{specifier}' imported from {referrer}"),
            };
            let new_subpath = subpath
                .parent()
                .map(|parent| parent.to_owned())
                .unwrap_or(RelativePathBuf::from(""))
                .join(specifier_path);

            let candidates = [
                new_subpath.join("index.js"),
                new_subpath.join("index.ts"),
                new_subpath.with_extension("js"),
                new_subpath.with_extension("ts"),
                new_subpath,
            ];

            for candidate in candidates {
                let file = RuntimeFiles::get(candidate.as_str());
                if file.is_some() {
                    return Ok(RiftModuleSpecifier::Internal { subpath: candidate });
                }
            }

            anyhow::bail!("internal module '{specifier}' not found (imported from {referrer})");
        }
        RiftModuleSpecifier::Package { pkg_name } => {
            todo!("RiftModuleSpecifier::Package => TODO Later")
        }
        RiftModuleSpecifier::File { path } => todo!("RiftModuleSpecifier::File => TODO Later"),
    }
}

#[cfg(test)]
mod test {
    use super::RiftModuleSpecifier;

    #[test]
    fn test_specifiers() {
        let rift_specifier: RiftModuleSpecifier = "rift:path/to/file.ts".parse().unwrap();
        println!("{}", rift_specifier);
        let pkg_specifier: RiftModuleSpecifier = "pkg:rift.generate/to/file.ts".parse().unwrap();
        println!("{}", pkg_specifier);
        let file_specifier: RiftModuleSpecifier = "file:///D:/workshop/projects/rift/sample/02_single_target_with_project/rift/dependencies.ts".parse().unwrap();
        println!("{}", file_specifier);
        let deno_specifier: deno_core::ModuleSpecifier = rift_specifier.into();
        // assert_eq!(specifier.to_string(), "rift:///path/to/file.ts");
        // let specifier: RiftModuleSpecifier = "pkg:///path/to/file.ts".parse().unwrap();
        // assert_eq!(specifier.to_string(), "pkg:///path/to/file.ts");
        // let specifier: RiftModuleSpecifier = "file:///path/to/file.ts".parse().unwrap();
        // assert_eq!(specifier.to_string(), "file:///path/to/file.ts");
        // let file_specifier = url::Url::parse("file:///path/to/file.ts").unwrap();
    }
}
