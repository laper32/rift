mod ops;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use relative_path::PathExt;
use serde::{Deserialize, Serialize};

use crate::{
    schema::{self, TomlTaskFlag, TomlTaskInstance},
    task::TaskManager,
    util::{errors::RiftResult, fs::as_posix::PathExt as _},
    workspace::WorkspaceManager,
};

pub fn init_ops() -> deno_core::Extension {
    ops::manifest::init_ops()
}

pub const MANIFEST_IDENTIFIER: &str = "Rift.toml";

/// 为什么有EitherManifest？为什么schema那边还要定义一份TomlXXX？为什么我们有MaybePackage的同时还要有EitherManifest?
///
/// 首先，schema那边定义了所有我们所需要的配置信息，但是有一个问题：我们不能，也不可能知道这个配置信息有没有问题。<br/>
/// 最典型的就是我们不可能在加载配置文件的时候就知道说workspace和folder同时存在吧。 <br/>
/// 所以我们需要做第一层转换，将配置文件的数据格式转换成我们真正需要的manifset struct。 <br/>
/// 这也是为什么我们有TomlManifest。
///
/// 然后，加载了Manifest以后，我们肯定要针对这些Manifest分类，哪些是项目？哪些用于处理架构？哪些是给Rift的扩展？我们总得想办法知道吧。 <br/>
/// 所以下面这三个enum field就是这么来的。
#[derive(Debug)]
pub enum EitherManifest {
    Real(Manifest),
    Virtual(VirtualManifest),
    Rift(RiftManifest),
}
impl EitherManifest {
    pub fn name(&self) -> String {
        match self {
            EitherManifest::Real(m) => m.name(),
            EitherManifest::Virtual(m) => m.name(),
            EitherManifest::Rift(m) => m.name(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceManifest {
    pub name: String,
    /// 没啥说的
    pub members: Vec<String>,

    /// 没啥说的
    pub exclude: Option<Vec<String>>,

    /// 指向的是文件路径
    pub metadata: Option<String>,

    /// 指向的是文件路径
    pub plugins: Option<String>,

    /// 指向的是文件路径
    pub dependencies: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderManifest {
    pub name: String,
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub name: String,
    pub authors: Vec<String>,
    pub version: String,
    pub description: Option<String>,
    pub plugins: Option<String>,
    pub dependencies: Option<String>,
    pub metadata: Option<String>,
    // 如果project和target同时存在，那么members和exclude将无法使用，就算里面写东西也会被忽略
    // 除此之外无限制。
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,

    // 当且仅当只有一个target出现的时候才会有这个field
    pub target: Option<TargetManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetManifest {
    pub name: String,
    pub build_type: String,
    pub plugins: Option<String>,
    pub dependencies: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub metadata: Option<String>,
    pub dependencies: Option<String>,
    pub entry: String,
}

pub type AliasManifest = HashMap<String, AliasInstanceManifest>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasInstanceManifest {
    pub alias: String,
    pub referred_pkg_name: String,
}

pub type TaskManifest = HashMap<String, TaskInstanceManifest>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInstanceManifest {
    pub description: Option<String>,
    pub args: Option<Vec<TaskFlagManifest>>,
    pub is_command: bool,
    pub referred_pkg_name: String,
}

impl TaskInstanceManifest {
    pub fn has_flag(&self, flag_name: &str) -> bool {
        self.args
            .as_ref()
            .map_or(false, |args| args.iter().any(|arg| arg.name.eq(flag_name)))
    }
    fn from_schema(pkg_name: String, schema: &TomlTaskInstance) -> Self {
        TaskInstanceManifest {
            description: schema.about.clone(),
            args: schema
                .args
                .as_ref()
                .map(|args| args.iter().map(TaskFlagManifest::from_schema).collect()),
            referred_pkg_name: pkg_name,
            is_command: schema.is_command.unwrap_or(false),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskFlagManifest {
    pub name: String,
    pub short: Option<String>,
    pub description: Option<String>,
    pub default: Option<toml::Value>,
    pub conflict_with: Option<Vec<String>>,
    pub heading: Option<String>,
}

impl TaskFlagManifest {
    fn from_schema(schema: &TomlTaskFlag) -> Self {
        TaskFlagManifest {
            name: schema.name.clone(),
            short: schema.short.clone(),
            description: schema.description.clone(),
            default: schema.default.clone(),
            conflict_with: schema.conflict_with.clone(),
            heading: schema.heading.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifestDeclarator {
    pub name: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyManifestDeclarator {
    pub name: String,

    pub data: HashMap<String, serde_json::Value>,
}

/// 针对项目本身的。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Manifest {
    Project(ProjectManifest),
    Target(TargetManifest),
}

impl Manifest {
    pub fn name(&self) -> String {
        match self {
            Manifest::Project(p) => p.name.clone(),
            Manifest::Target(t) => t.name.clone(),
        }
    }

    pub fn dependencies(&self) -> Option<String> {
        match self {
            Manifest::Project(p) => p.dependencies.clone(),
            Manifest::Target(t) => t.dependencies.clone(),
        }
    }
    pub fn plugins(&self) -> Option<String> {
        match self {
            Manifest::Project(p) => p.plugins.clone(),
            Manifest::Target(t) => t.plugins.clone(),
        }
    }
    pub fn metadata(&self) -> Option<String> {
        match self {
            Manifest::Project(p) => p.metadata.clone(),
            Manifest::Target(t) => t.metadata.clone(),
        }
    }
}

///组织项目结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VirtualManifest {
    Workspace(WorkspaceManifest),
    Folder(FolderManifest),
}

impl VirtualManifest {
    pub fn name(&self) -> String {
        match self {
            VirtualManifest::Workspace(w) => w.name.clone(),
            VirtualManifest::Folder(f) => f.name.clone(),
        }
    }

    pub fn members(&self) -> Option<Vec<String>> {
        match self {
            VirtualManifest::Workspace(w) => Some(w.members.clone()),
            VirtualManifest::Folder(f) => f.members.clone(),
        }
    }

    pub fn excludes(&self) -> Option<Vec<String>> {
        match self {
            VirtualManifest::Workspace(w) => w.exclude.clone(),
            VirtualManifest::Folder(f) => f.exclude.clone(),
        }
    }

    pub fn dependencies(&self) -> Option<String> {
        match self {
            VirtualManifest::Workspace(w) => w.dependencies.clone(),
            VirtualManifest::Folder(_) => None,
        }
    }

    pub fn plugins(&self) -> Option<String> {
        match self {
            VirtualManifest::Workspace(w) => w.plugins.clone(),
            VirtualManifest::Folder(_) => None,
        }
    }

    pub fn metadata(&self) -> Option<String> {
        match self {
            VirtualManifest::Workspace(w) => w.metadata.clone(),
            VirtualManifest::Folder(_) => None,
        }
    }
}

// 给rift用的
// 如：插件，内核扩展（如果以后有需要的话）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiftManifest {
    Plugin(PluginManifest),
}

impl RiftManifest {
    pub fn name(&self) -> String {
        match self {
            RiftManifest::Plugin(p) => p.name.clone(),
        }
    }
    pub fn dependencies(&self) -> Option<String> {
        match self {
            RiftManifest::Plugin(p) => p.dependencies.clone(),
        }
    }
    pub fn metadata(&self) -> Option<String> {
        match self {
            RiftManifest::Plugin(p) => p.metadata.clone(),
        }
    }
}

impl Into<EitherManifest> for RiftManifest {
    fn into(self) -> EitherManifest {
        EitherManifest::Rift(self)
    }
}

pub fn find_root_manifest(current_path: &PathBuf) -> Option<PathBuf> {
    let parent_manifest_path = current_path.parent()?.join(MANIFEST_IDENTIFIER);
    if parent_manifest_path.exists() {
        Some(parent_manifest_path)
    } else {
        find_root_manifest(&parent_manifest_path.parent()?.to_path_buf())
    }
}

/// 解析Manifest，根据路劲内的信息为其分类至正确的enum。
pub fn read_manifest(path: &Path) -> RiftResult<EitherManifest> {
    if !WorkspaceManager::instance().is_init() {
        return Err(anyhow::anyhow!("WorkspaceManager is not initialized."));
    }

    let manifest = (|| {
        let deserialized_toml = schema::load_manifest(&path.to_path_buf());
        match deserialized_toml {
            Ok(manifest) => {
                if manifest.workspace.is_some() {
                    if manifest.folder.is_some()
                        || manifest.project.is_some()
                        || manifest.target.is_some()
                        || manifest.plugin.is_some()
                    {
                        anyhow::bail!(
                            "Workspace and Folder/Project/Target/Plugin can't be used together."
                        )
                    }
                    let workspace_manifest = manifest.workspace.unwrap();
                    // 如果workspace manifest里面没有指定名字，那么我们就认为这个workspace的文件夹名字就是其名字。
                    // 毕竟按理说这里就是root。。。所以这么写也没什么问题。
                    let mut workspace_name = "";
                    if workspace_manifest.name.is_none() {
                        let manifest_location = path.parent().unwrap();
                        let workspace_root = WorkspaceManager::instance().root();
                        if workspace_root.eq(manifest_location) {
                            workspace_name =
                                manifest_location.file_name().unwrap().to_str().unwrap();
                        }
                    } else {
                        workspace_name = workspace_manifest.name.as_ref().unwrap();
                    }

                    if manifest.task.is_some() {
                        let task = manifest.task.unwrap();
                        let mut tasks: TaskManifest = HashMap::new();
                        for (task_name, task_instance) in task {
                            tasks.insert(
                                task_name,
                                TaskInstanceManifest::from_schema(
                                    workspace_name.to_string(),
                                    &task_instance,
                                ),
                            );
                        }
                        TaskManager::instance().load_task_from_manifest(&tasks);
                    }
                    if manifest.alias.is_some() {
                        let alias = manifest.alias.unwrap();
                        let mut aliases: AliasManifest = HashMap::new();
                        for (alias_name, alias_instance) in alias {
                            aliases.insert(
                                alias_name,
                                AliasInstanceManifest {
                                    alias: alias_instance,
                                    referred_pkg_name: workspace_name.to_string(),
                                },
                            );
                        }
                        TaskManager::instance().load_alias_from_manifest(&aliases);
                    }

                    return Ok(EitherManifest::Virtual(VirtualManifest::Workspace(
                        WorkspaceManifest {
                            name: workspace_name.to_string(),
                            members: workspace_manifest.members,
                            exclude: Some(workspace_manifest.exclude),
                            metadata: workspace_manifest.metadata,
                            plugins: workspace_manifest.plugins,
                            dependencies: workspace_manifest.dependencies,
                        },
                    )));
                } else if manifest.folder.is_some() {
                    if manifest.workspace.is_some()
                        || manifest.project.is_some()
                        || manifest.target.is_some()
                        || manifest.plugin.is_some()
                    {
                        anyhow::bail!(
                            "Workspace and Folder/Project/Target/Plugin can't be used together."
                        )
                    }
                    let folder_manifest = manifest.folder.unwrap();
                    let folder_name = if folder_manifest.name.is_none() {
                        // 这里用了一个trick：
                        // 我们folder一定是在根目录底下的，换句话说folder不可能作为根目录存在。
                        // 所以我们先得到当前root，然后取parent，再取当前folder所在manifest的路径
                        // 然后我们取这两个路径之间的相对路径，归一化成"/"以后换成"."就是我们的结果。
                        // 很tricky但很有效。
                        let manifest_location = path.parent().unwrap();
                        let workspace_root = WorkspaceManager::instance().root();
                        if workspace_root.eq(manifest_location) {
                            anyhow::bail!("You can't use folder manifest in workspace root.")
                        }
                        let dummy = workspace_root.parent().unwrap().to_path_buf();
                        let calculated_result = manifest_location.relative_to(dummy);
                        if calculated_result.is_err() {
                            anyhow::bail!("Cannot calculate folder name.")
                        }

                        let calculated_result = calculated_result.unwrap().into_string().to_owned();
                        Path::new(&calculated_result)
                            .as_posix()
                            .unwrap()
                            .to_string()
                            .replace("/", ".")
                    } else {
                        folder_manifest.name.as_ref().unwrap().to_string()
                    };

                    return Ok(EitherManifest::Virtual(VirtualManifest::Folder(
                        FolderManifest {
                            name: folder_name,
                            members: Some(folder_manifest.members),
                            exclude: Some(folder_manifest.exclude),
                        },
                    )));
                } else if manifest.project.is_some() {
                    let project_manifest = manifest.project.unwrap();
                    if manifest.workspace.is_some()
                        || manifest.folder.is_some()
                        || manifest.plugin.is_some()
                    {
                        anyhow::bail!("Workspace and Folder/Project/Plugin can't be used together.")
                    }

                    let mut result_manifest = ProjectManifest {
                        name: project_manifest.name,
                        authors: project_manifest.authors,
                        version: project_manifest.version,
                        description: project_manifest.description,
                        plugins: project_manifest.plugins,
                        dependencies: project_manifest.dependencies,
                        metadata: project_manifest.metadata,
                        members: project_manifest.members,
                        exclude: project_manifest.exclude,
                        target: None,
                    };

                    if manifest.target.is_some() {
                        if result_manifest.members.is_some() || result_manifest.exclude.is_some() {
                            anyhow::bail!("Members/Exclude cannot both occur with Target.")
                        } else {
                            let target_manifest = manifest.target.unwrap();
                            result_manifest.target = Some(TargetManifest {
                                name: target_manifest.name,
                                build_type: target_manifest.build_type,
                                // 单Project，单Target架构的情况下，只认Project。
                                plugins: None,
                                dependencies: None,
                                metadata: None,
                            });
                        }
                    }
                    if manifest.task.is_some() {
                        let task = manifest.task.unwrap();
                        let mut tasks: TaskManifest = HashMap::new();
                        for (task_name, task_instance) in task {
                            tasks.insert(
                                task_name,
                                TaskInstanceManifest::from_schema(
                                    result_manifest.name.clone(),
                                    &task_instance,
                                ),
                            );
                        }
                        TaskManager::instance().load_task_from_manifest(&tasks);
                    }
                    if manifest.alias.is_some() {
                        let alias = manifest.alias.unwrap();
                        let mut aliases: AliasManifest = HashMap::new();
                        for (alias_name, alias_instance) in alias {
                            aliases.insert(
                                alias_name,
                                AliasInstanceManifest {
                                    alias: alias_instance,
                                    referred_pkg_name: result_manifest.name.clone(),
                                },
                            );
                        }
                        TaskManager::instance().load_alias_from_manifest(&aliases);
                    }

                    return Ok(EitherManifest::Real(Manifest::Project(result_manifest)));
                } else if manifest.target.is_some() {
                    let target_manifest = manifest.target.unwrap();
                    if manifest.workspace.is_some()
                        || manifest.folder.is_some()
                        || manifest.plugin.is_some()
                    {
                        anyhow::bail!("Workspace and Folder/Target/Plugin can't be used together.")
                    }

                    if manifest.task.is_some() {
                        let task = manifest.task.unwrap();
                        let mut tasks: TaskManifest = HashMap::new();
                        for (task_name, task_instance) in task {
                            tasks.insert(
                                task_name,
                                TaskInstanceManifest::from_schema(
                                    target_manifest.name.clone(),
                                    &task_instance,
                                ),
                            );
                        }
                        TaskManager::instance().load_task_from_manifest(&tasks);
                    }
                    if manifest.alias.is_some() {
                        let alias = manifest.alias.unwrap();
                        let mut aliases: AliasManifest = HashMap::new();
                        for (alias_name, alias_instance) in alias {
                            aliases.insert(
                                alias_name,
                                AliasInstanceManifest {
                                    alias: alias_instance,
                                    referred_pkg_name: target_manifest.name.clone(),
                                },
                            );
                        }
                        TaskManager::instance().load_alias_from_manifest(&aliases);
                    }

                    return Ok(EitherManifest::Real(Manifest::Target(TargetManifest {
                        name: target_manifest.name,
                        build_type: target_manifest.build_type,
                        plugins: target_manifest.plugins,
                        dependencies: target_manifest.dependencies,
                        metadata: target_manifest.metadata,
                    })));
                } else if manifest.plugin.is_some() {
                    let plugin_manifest = manifest.plugin.unwrap();
                    // plugin和其他都互斥
                    // 插件就老老实实平铺，别TM给老子干有的没的！
                    if manifest.workspace.is_some()
                        || manifest.folder.is_some()
                        || manifest.project.is_some()
                        || manifest.target.is_some()
                    {
                        anyhow::bail!(
                            "Workspace and Folder/Project/Target/Plugin can't be used together."
                        )
                    }

                    if manifest.task.is_some() {
                        let task = manifest.task.unwrap();
                        let mut tasks: TaskManifest = HashMap::new();
                        for (task_name, task_instance) in task {
                            tasks.insert(
                                task_name,
                                TaskInstanceManifest::from_schema(
                                    plugin_manifest.name.clone(),
                                    &task_instance,
                                ),
                            );
                        }
                        TaskManager::instance().load_task_from_manifest(&tasks);
                    }
                    if manifest.alias.is_some() {
                        let alias = manifest.alias.unwrap();
                        let mut aliases: AliasManifest = HashMap::new();
                        for (alias_name, alias_instance) in alias {
                            aliases.insert(
                                alias_name,
                                AliasInstanceManifest {
                                    alias: alias_instance,
                                    referred_pkg_name: plugin_manifest.name.clone(),
                                },
                            );
                        }
                        TaskManager::instance().load_alias_from_manifest(&aliases);
                    }

                    return Ok(EitherManifest::Rift(RiftManifest::Plugin(PluginManifest {
                        name: plugin_manifest.name,
                        version: plugin_manifest.version,
                        authors: plugin_manifest.authors,
                        description: plugin_manifest.description,
                        metadata: plugin_manifest.metadata,
                        dependencies: plugin_manifest.dependencies,
                        entry: plugin_manifest.entry,
                    })));
                } else {
                    anyhow::bail!("No any schema field found.")
                }
            }
            Err(e) => anyhow::bail!("error: {:?}", e),
        }
    })();

    Ok(manifest?)
}

/// 暂时用不着，之后再说
#[allow(dead_code)]
pub fn print_manifest_structure(path: &Path, prefix: String, is_last: bool) {
    // Current folder
    let indent = if is_last { "└─" } else { "├─" };
    println!(
        "{}{}{}",
        prefix,
        indent,
        path.file_name().unwrap().to_str().unwrap()
    );

    // If has sub folder
    if let Ok(entries) = fs::read_dir(path) {
        let mut entries = entries.flatten().collect::<Vec<_>>();
        entries.sort_by_key(|e| e.path()); // sort by char

        let mut iter = entries.iter().peekable();
        while let Some(entry) = iter.next() {
            let entry_path = entry.path();

            if entry_path.is_dir() {
                // recursivlely print
                let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
                let is_last_entry = iter.peek().is_none();
                print_manifest_structure(&entry_path, new_prefix, is_last_entry);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::print_manifest_structure;
    use crate::{
        manifest::{find_root_manifest, read_manifest},
        util::get_cargo_project_root,
    };

    #[test]
    fn test_find_root_manifest() {
        let sample_manifest_path = get_cargo_project_root()
            .unwrap()
            .join("sample")
            .join("06_workspace_folder_project_target")
            .join("folder2");
        let manifest_root = find_root_manifest(&sample_manifest_path);
        println!("{:?}", manifest_root);
        let parsed_result = read_manifest(&manifest_root.unwrap().to_path_buf()).unwrap();
        println!("{:?}", parsed_result);
    }

    #[test]
    fn test_print_manifest_structure() {
        let project_root = get_cargo_project_root()
            .unwrap()
            .join("sample")
            .join("04_workspace_and_multiple_projects");

        println!("Manifest Structure:");
        print_manifest_structure(&project_root, String::new(), true);
    }
}
