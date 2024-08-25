use std::borrow::Cow;
/// 这里应当分为如下几个部分：
/// 1. Rift内部捆绑
/// 2. 第一方插件
/// 3. User目录的插件
/// 4. 项目内部的插件
/// 5. 项目内部作为Util的脚本
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::manifest::MANIFEST_IDENTIFIER;
use crate::runtime::RuntimeFiles;
use crate::util::errors::RiftResult;
use crate::workspace::Workspace;
use anyhow::Context;
use relative_path::{PathExt, RelativePathBuf};

use super::script::ScriptManager;

/// An `import` specifier referring to a file within the current project.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RiftLocalImportSpecifier {
    // 相对路径。。。
    Relative(String),
    // // TODO: 最好是能做成这样: import * from "rift:projectRoot/subpath"
    // // 这样的话我们就可以直接解决掉import的时候不知道怎么办的问题
    // ProjectRoot(String),
}

impl std::fmt::Display for RiftLocalImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiftLocalImportSpecifier::Relative(path) => write!(f, "{path}"),
            // RiftModuleLocalImportSpecifier::ProjectRoot(path) => write!(f, "/{path}"),
        }
    }
}

impl std::str::FromStr for RiftLocalImportSpecifier {
    type Err = anyhow::Error;

    fn from_str(specifier: &str) -> Result<Self, Self::Err> {
        if specifier == "."
            || specifier == ".."
            || specifier.starts_with("./")
            || specifier.starts_with("../")
        {
            return Ok(Self::Relative(specifier.to_string()));
        }
        /* else if let Some(project_root_subpath) = specifier.strip_prefix('/') {
            return Ok(Self::ProjectRoot(project_root_subpath.to_string()));
        }  */
        else {
            anyhow::bail!("Invalid module specifier: {}", specifier)
        }
    }
}

// =================================================================================

/// A specifier from an `import` statement in a JavaScript module. Can
/// be resolved to a module specifier using the `resolve` function.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RiftImportSpecifier {
    /// 项目内的，没啥好说的吧。
    Local(RiftLocalImportSpecifier),
    /// 非项目内的包，比如说：安装目录的插件包，UserProfile的插件包
    External(String),
}

impl std::fmt::Display for RiftImportSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiftImportSpecifier::Local(specifier) => write!(f, "{}", specifier),
            RiftImportSpecifier::External(specifier) => write!(f, "{}", specifier),
        }
    }
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
            let local_specifier = specifier.parse()?;
            Ok(Self::Local(local_specifier))
        } else {
            Ok(Self::External(specifier.to_string()))
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

// =================================================================================

pub fn runtime_specifiers_with_contents(
) -> impl Iterator<Item = (RiftModuleSpecifier, Cow<'static, [u8]>)> {
    RuntimeFiles::iter().flat_map(|path| {
        let file = RuntimeFiles::get(&path)?;
        let specifier = RiftModuleSpecifier::Runtime {
            subpath: RelativePathBuf::from(&*path),
        };
        Some((specifier, file.data))
    })
}

pub fn read_specifier_contents(specifier: &RiftModuleSpecifier) -> RiftResult<Arc<Vec<u8>>> {
    match specifier {
        RiftModuleSpecifier::Runtime { subpath } => {
            let file = RuntimeFiles::get(subpath.as_str())
                .with_context(|| format!("internal module '{specifier}' not found"))?;
            Ok(Arc::new(file.data.to_vec()))
        }
        RiftModuleSpecifier::File { path } => {
            let (_, contents) = ScriptManager::instance()
                .load_cached(path)?
                .with_context(|| format!("module '{specifier}' not loaded"))?;
            Ok(contents)
        }
    }
}

pub async fn load_specifier_contents(specifier: &RiftModuleSpecifier) -> RiftResult<Arc<Vec<u8>>> {
    match specifier {
        RiftModuleSpecifier::Runtime { .. } => {}
        RiftModuleSpecifier::File { path } => {
            ScriptManager::instance()
                .load(path)
                .await
                .with_context(|| format!("module '{specifier}' not loaded"))?;
        }
    }
    read_specifier_contents(specifier)
}

pub fn resolve(
    specifier: &RiftImportSpecifier,
    referrer: &RiftModuleSpecifier,
) -> RiftResult<RiftModuleSpecifier> {
    match referrer {
        RiftModuleSpecifier::Runtime { subpath } => {
            let specifier_path = match specifier {
                RiftImportSpecifier::Local(RiftLocalImportSpecifier::Relative(specifier_path)) => {
                    specifier_path
                }
                _ => {
                    anyhow::bail!("Invalid specifier")
                }
            };
            let new_subpath = subpath
                .parent()
                .map(|parent| parent.to_owned())
                .unwrap_or(RelativePathBuf::from(""))
                .join(specifier_path);

            // 既然惯例是index.ts/js 那我们也这么用算了。。
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
                    return Ok(RiftModuleSpecifier::Runtime { subpath: candidate });
                }
            }
            anyhow::bail!("internal module '{specifier}' not found (imported from {referrer})");
        }
        RiftModuleSpecifier::File { path } => {
            // 这里要做的事就有点多了。首先我们得明确如下问题：
            // 这个文件是来自于哪里的？是插件，还是项目，还是只是单纯的为了简写而做的包装性的Util，还是Rift.toml里面指定的脚本？
            let project_manifest = path.join(MANIFEST_IDENTIFIER);
            let workspace = Workspace::new(&project_manifest);
            let subpath = path.relative_to(path)?;
            match specifier {
                RiftImportSpecifier::Local(RiftLocalImportSpecifier::Relative(specifier_path)) => {
                    let new_subpath = subpath
                        .parent()
                        .map(|parent| parent.to_owned())
                        .unwrap_or(RelativePathBuf::from(""))
                        .join_normalized(specifier_path);
                }
                RiftImportSpecifier::External(_) => todo!(),
            }
            // match specifier {
            //     // 以项目根目录为主的import
            //     RiftModuleImportSpecifier::Local(RiftModuleLocalImportSpecifier::ProjectRoot(
            //         specifier_path,
            //     )) => {}
            //     // 直接就是相对路径import
            //     RiftModuleImportSpecifier::Local(RiftModuleLocalImportSpecifier::Relative(
            //         specifier_path,
            //     )) => {}
            //     // External可以来自于：
            //     // - 项目的.rift/plugins
            //     // - ~/.rift/plugins
            //     // - ${InstallationPath}/plugins
            //     RiftModuleImportSpecifier::External(dep) => {}
            // }
            todo!()
        }
    }
    todo!()
}

#[cfg(test)]
mod test {
    use crate::runtime::specifier::RiftModuleSpecifier;

    use super::read_specifier_contents;

    #[test]
    fn test_module_specifier() {
        let specifier: RiftModuleSpecifier = "rift:///dist/index.js".parse().expect("Failed");
        println!("{:?}", specifier);
        let contents = read_specifier_contents(&specifier);
        println!("{:?}", contents.unwrap().len())
    }
}
