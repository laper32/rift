use crate::package::Package;

pub enum MaybePackage {
    Package(Package),
    Virtual
}

//
// pub struct Package {}
//
// pub struct Packages {
//     packages: HashMap<PathBuf, MaybePackage>,
// }
//
// pub enum MaybePackage {
//     // Project, Target, Plugin
//     Package(Package),
//     // Folder, Workspace
//     Virtual,
// }
//
// pub struct Workspace {
//     // 我觉得没啥好说的？
//     current_manifest: PathBuf,
//
//     // 用于处理我们有workspace的情况
//     root_manifest: Option<PathBuf>,
//     packages: Packages,
// }
//
// impl Workspace {
//     pub fn new(manifest_path: &Path) -> RiftResult<Workspace> {
//         let mut workspace = Workspace::new_default(manifest_path.to_path_buf());
//
//         Ok(workspace)
//     }
//
//     fn new_default(current_manifest: PathBuf) -> Workspace {
//         Workspace {
//             current_manifest,
//             root_manifest: None,
//             packages: Packages {
//                 packages: HashMap::new(),
//             },
//         }
//     }
//
//     fn find_root(&mut self, manifest_path: &Path) -> RiftResult<Option<PathBuf>> {
//         Ok(None)
//     }
//
//     /// Returns the root path of this workspace.
//     ///
//     /// That is, this returns the path of the directory containing the
//     /// `Rift.toml` which is the root of this workspace.
//     pub fn root(&self) -> &Path {
//         self.root_manifest().parent().unwrap()
//     }
//
//     /// Returns the path of the `Rift.toml` which is the root of this
//     /// workspace.
//     pub fn root_manifest(&self) -> &Path {
//         self.root_manifest
//             .as_ref()
//             .unwrap_or(&self.current_manifest)
//     }
// }
//
// impl Packages {
// }
