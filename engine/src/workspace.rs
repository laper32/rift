use crate::errors::{RiftResult, SimpleError};
use crate::manifest::{Manifest, WorkspaceManifest, MANIFEST_IDENTIFIER};
use crate::package::Package;
use std::collections::HashMap;
use std::path::PathBuf;

struct Packages {
    packages: HashMap<PathBuf, Package>,
}

/*
需要明确如下问题：
1. 这个项目里有多少个包？
并不是什么时候都会[workspace]，有的时候是project，有的时候是target，不管怎么说，总不能给每个都设置一个struct来处理吧？


2. 哪些包是可编译的，哪些包是用于组织项目结构不参与实际运行的(Workspace, Folder)，哪些包是Rift用的？
可编译单元肯定是target，但我们肯定要转译一层，而非直接用manifest解析出来的结果。

3. 依赖关系。
插件有依赖关系，项目/workspace有依赖关系，项目的引用也有依赖关系。
本质问题和1一样

*/

pub struct Workspace {
    current_manifest: PathBuf,

    // 整个工作区的包。
    packages: Packages,
}

trait Node {
    // TODO: rename
    fn root_manifest(&self) -> Option<PathBuf>;
}

impl Node for Workspace {
    fn root_manifest(&self) -> Option<PathBuf> {
        None
    }
}

impl Workspace {
    pub fn new(manifest_path: &WorkspaceManifest) -> Workspace {
        todo!()
    }
}

pub struct WorkspaceBuilder {
    possible_manifests: HashMap<PathBuf, Manifest>,
    init_path: PathBuf,
}

impl WorkspaceBuilder {
    pub fn new(current_path: &PathBuf) -> Self {
        Self {
            possible_manifests: HashMap::new(),
            init_path: current_path.clone(),
        }
    }

    fn find_root(&self, current_path: &PathBuf) -> Option<PathBuf> {
        let parent_manifest_path = current_path.parent()?.join(MANIFEST_IDENTIFIER);
        if parent_manifest_path.exists() {
            Some(parent_manifest_path)
        } else {
            self.find_root(&current_path.parent()?.to_path_buf())
        }
    }

    pub fn build(self) -> RiftResult<Workspace> {
        let root = match self.find_root(&self.init_path) {
            None => return Err(Box::new(SimpleError::new("Unable to parse manifest toml"))),
            Some(path) => path,
        };
        // let root_workspace = load_manifest::<TomlWorkspace>(&root)?;
        todo!()
    }
}
