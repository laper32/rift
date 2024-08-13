use crate::errors::{RiftResult, SimpleError};
use crate::manifest::{Connector, Manifest, WorkspaceManifest, MANIFEST_IDENTIFIER};
use crate::package::Package;
use std::collections::HashMap;
use std::path::PathBuf;



// struct Packages {
//     packages: HashMap<PathBuf, Package>,
// }

// 
// pub struct Workspace {
//     current_manifest: PathBuf,
// 
//     // 整个工作区的包。
//     packages: Packages,
// }
// 
// trait Node {
//     // TODO: rename
//     fn root_manifest(&self) -> Option<PathBuf>;
// }
// 
// impl Node for Workspace {
//     fn root_manifest(&self) -> Option<PathBuf> {
//         None
//     }
// }
// 
// impl Workspace {
//     pub fn new(manifest_path: &WorkspaceManifest) -> Workspace {
//         todo!()
//     }
// }
// 
// pub struct WorkspaceBuilder {
//     possible_manifests: HashMap<PathBuf, Manifest>,
//     init_path: PathBuf,
// }
// 
// impl WorkspaceBuilder {
//     pub fn new(current_path: &PathBuf) -> Self {
//         Self {
//             possible_manifests: HashMap::new(),
//             init_path: current_path.clone(),
//         }
//     }
// 
//     fn find_root(&self, current_path: &PathBuf) -> Option<PathBuf> {
//         let parent_manifest_path = current_path.parent()?.join(MANIFEST_IDENTIFIER);
//         if parent_manifest_path.exists() {
//             Some(parent_manifest_path)
//         } else {
//             self.find_root(&current_path.parent()?.to_path_buf())
//         }
//     }
// 
//     pub fn build(self) -> RiftResult<Workspace> {
//         let root = match self.find_root(&self.init_path) {
//             None => return Err(Box::new(SimpleError::new("Unable to parse manifest toml"))),
//             Some(path) => path,
//         };
//         // let root_workspace = load_manifest::<TomlWorkspace>(&root)?;
//         todo!()
//     }
// }
