/// 这里应当分为如下几个部分：
/// 1. Rift内部捆绑
/// 2. 第一方插件
/// 3. User目录的插件
/// 4. 项目内部的插件
/// 5. 项目内部作为Util的脚本
use std::path::{Path, PathBuf};

use relative_path::RelativePathBuf;

/// An `import` specifier referring to a file within the current project.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RiftModuleLocalImportSpecifier {
    // 相对路径。。。
    Relative(String),
    // 项目的根目录
    // 这个还得看看怎么归类，因为这里有歧义。
    ProjectRoot(String),
}

impl std::fmt::Display for RiftModuleLocalImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiftModuleLocalImportSpecifier::Relative(path) => write!(f, "{path}"),
            RiftModuleLocalImportSpecifier::ProjectRoot(path) => write!(f, "/{path}"),
        }
    }
}

impl std::str::FromStr for RiftModuleLocalImportSpecifier {
    type Err = anyhow::Error;

    fn from_str(specifier: &str) -> Result<Self, Self::Err> {
        if specifier == "."
            || specifier == ".."
            || specifier.starts_with("./")
            || specifier.starts_with("../")
        {
            return Ok(Self::Relative(specifier.to_string()));
        } else if let Some(project_root_subpath) = specifier.strip_prefix('/') {
            return Ok(Self::ProjectRoot(project_root_subpath.to_string()));
        } else {
            anyhow::bail!("Invalid module specifier: {}", specifier)
        }
    }
}

// =================================================================================

/// A specifier from an `import` statement in a JavaScript module. Can
/// be resolved to a module specifier using the `resolve` function.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RiftModuleImportSpecifier {
    /// 项目内的，没啥好说的吧。
    Local(RiftModuleLocalImportSpecifier),
    /// 非项目内的包，比如说：安装目录的插件包，UserProfile的插件包
    External(String),
}

impl std::fmt::Display for RiftModuleImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiftModuleImportSpecifier::Local(specifier) => write!(f, "{}", specifier),
            RiftModuleImportSpecifier::External(specifier) => write!(f, "{}", specifier),
        }
    }
}

// =============================================================================

/// A specifier for a Rift module, either from the filesystem or
/// from the internal runtime package. A module specifier can be converted
/// from/to a URL.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, serde_with::DeserializeFromStr, serde_with::SerializeDisplay,
)]
pub enum RiftModuleSpecifier {
    /// 捆绑进rift.exe的内置包，比如说RiftProjectManifest这种不太适合作为插件的情况。。
    Runtime { subpath: RelativePathBuf },
    /// 外部路径，咯
    File { path: PathBuf },
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
                    .map_err(|_| anyhow::anyhow!("Invalid file URL: {}", value))?;
                Ok(Self::File { path })
            }
            "rift" => {
                anyhow::ensure!(!value.has_host(), "invalid specifier: {}", value);
                let subpath = RelativePathBuf::from(value.path().trim_start_matches('/'));
                Ok(Self::Runtime { subpath })
            }
            _ => anyhow::bail!("Unsupported URL scheme: {}", value.scheme()),
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
            RiftModuleSpecifier::Runtime { subpath } => {
                let mut url: url::Url = "rift:///".parse().expect("failed to build URL");
                url.set_path(subpath.as_str());
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
