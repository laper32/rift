pub const MANIFEST_IDENTIFIER: &str = "Rift.toml";

pub struct WorkspaceManifest {
    /// 没啥说的
    pub members: Option<Vec<String>>,

    /// 没啥说的
    pub exclude: Option<Vec<String>>,

    /// 指向的是文件路径
    pub metadata: Option<String>,

    /// 指向的是文件路径
    pub plugins: Option<String>,

    /// 指向的是文件路径
    pub dependencies: Option<String>,
}

pub struct FolderManifest {
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

pub struct ProjectManifest {
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    pub description: Option<String>,
    pub plugins: Option<String>,
    pub dependencies: Option<String>,
    pub metadata: Option<String>,
    // 多个target才会用，如果只有一个的话建议不用，虽然我们不反对。
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}
pub struct TargetManifest {
    pub name: String,
    pub build_type: String,
    pub plugins: Option<String>,
    pub dependencies: Option<String>,
    pub metadata: Option<String>,
}

pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub metadata: Option<String>,
    pub dependencies: Option<String>,
}

pub enum Manifest {
    Workspace(WorkspaceManifest),
    Folder(FolderManifest),
    Project(ProjectManifest),
    Target(TargetManifest),
    Plugin(PluginManifest),
}

// impl ProjectManifest {
//     pub fn new() -> ProjectManifest {
//         ProjectManifest {}
//     }
// }
// 
// impl TargetManifest {
//     pub fn new() -> TargetManifest {
//         TargetManifest {
//             
//         }
//     }
// }
// 
// impl PluginManifest {
//     pub fn new() -> PluginManifest {
//         PluginManifest {
//             
//         }
//     }
// }