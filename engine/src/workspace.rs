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
并不是什么时候都会[workspace]，有的时候是project，有的时候就只是一个单独的target，不管怎么说，总不能给每个都设置一个struct来处理吧？

2. 哪些包是可编译的，哪些包是用于组织项目结构不参与实际运行的(Workspace, Folder)，哪些包是Rift用的？
可编译单元肯定是target，但我们肯定要转译一层，而非直接用manifest解析出来的结果。

3. 依赖关系。
插件有依赖关系，项目/workspace有依赖关系，项目的引用也有依赖关系。
本质问题和1一样

4. Package的定义？
虽然说我们把Workspace和Folder也算成Package，但如果按照严格定义的话，Package是可以发布在包管理器上的，而非常明显的，Workspace和Folder不行。
N.B. 所以对于Workspace和Folder的定义应该是Tunnel?

-------

因此，我们需要如下Field:
current_manifest: PathBuf => 我们需要知道当前执行rift的时候，我们在哪个文件夹
因为我们每次启动rift是不可能都有机会读缓存的，解析workspace这种操作是不可避免的，倒不如每次都记录一下。

root_manifest: PathBuf => 我们需要知道当前项目的根目录在哪里
考虑如下情况: Workspace, Project，这两个都有组织项目结构的功能。
有些项目只有一个Project+多个Target，那么这个时候root_manifest就是这个Project
有的项目是是多个Project，那自然上面就会有个Workspace，所以root_manifest就是Workspace
那么问题就和current_manifest一样了，rift build的时候，肯定是全量搜+编译，不可能每次都读缓存。

packages: Packages => 没啥好说的？我们需要知道整个工作区有多少个包
N.B. 有关于包的逻辑：已知我们的Package有5种类型，且他们有一定的上下级关系。
但在第一遍扫描的时候，重点应当是我们有多少个包
后续扫描才重点关注它们的逻辑关系，如根据包的情况染色，建立节点，etc。

-------
执行流程？
1. 先找到根目录(Root的Rift.toml)，如果没有，就认定其为根目录。
2. 找到根目录后，读Manifest，自上而下解析项目。
    1. 先解析Manifest里面有的
    2. 再解析Dependencies
    3. 再解析Plugins
3. 最后，根据Exclude的情况，排除指定忽略的路径（我们这时候无需知道其有没有Rift.toml，有就处理，没有pass）
4. 完成。

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
