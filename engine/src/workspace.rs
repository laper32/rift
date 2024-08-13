use crate::{
    errors::RiftResult,
    manifest::{find_root_manifest, Manifest, VirtualManifest},
};
use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
};

pub struct Packages {
    packages: HashMap<PathBuf, Manifest>,
}

pub enum MaybePackage {
    Package(Manifest),
    Virtual(VirtualManifest),
}

pub struct Workspace {
    // 我们需要知道是在哪里调用的rift
    current_manifest: PathBuf,

    // 我们需要记录这个workspace的根目录在哪里
    root_manifest: Option<PathBuf>,

    // 有多少包
    packages: Packages,
}

impl Workspace {
    pub fn new(manifest_path: &PathBuf) -> Workspace {
        if !manifest_path.ends_with("Rift.toml") {
            panic!("Workspace must be initialized with a Rift.toml file");
        }
        let mut ws = Workspace::new_default(manifest_path);
        ws.root_manifest = find_root_manifest(manifest_path);
        ws
    }
    fn new_default(manifest_path: &PathBuf) -> Workspace {
        Workspace {
            current_manifest: manifest_path.clone(),
            root_manifest: None,
            packages: Packages {
                packages: HashMap::new(),
            },
        }
    }

    pub fn root(&self) -> &Path {
        self.root_manifest().parent().unwrap()
    }

    pub fn root_manifest(&self) -> &Path {
        self.root_manifest
            .as_ref()
            .unwrap_or(&self.current_manifest)
    }
}

impl Packages {
    pub fn load(&mut self, manifest_path: &Path) -> RiftResult<&MaybePackage> {
        // let manifest = Manifest::load(manifest_path)?;
        // Ok(MaybePackage::Package(manifest))
        let key = manifest_path.parent().unwrap();
        match self.packages.entry(key.to_path_buf()) {
            Entry::Occupied(e) => {
                todo!()
            }
            Entry::Vacant(v) => {
                // let source_id = SourceId::for_path(key)?;
                // let manifest = read_manifest(manifest_path, source_id, self.gctx)?;
                // Ok(v.insert(match manifest {
                //     EitherManifest::Real(manifest) => {
                //         MaybePackage::Package(Package::new(manifest, manifest_path))
                //     }
                //     EitherManifest::Virtual(vm) => MaybePackage::Virtual(vm),
                // }))
                todo!();
            }
        };
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::paths::PathExt;
    use crate::util;

    use super::Workspace;

    #[test]
    fn test_load_workspace_happypath() {
        // happypath是什么？
        // 一定在根目录底下，且有Rift.toml
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("04_workspace_and_multiple_projects")
            .join("Rift.toml"); // 这里只有一个Workspace和Project，没有别的东西
        let ws = Workspace::new(&simple_workspace);
        let binding = our_project_root
            .join("sample")
            .join("04_workspace_and_multiple_projects");
        let expected_result = binding.as_posix();

        let binding = our_project_root
            .join("sample")
            .join("04_workspace_and_multiple_projects")
            .join("Rift.toml");
        let expect_result_manifest = binding.as_posix();
        assert_eq!(ws.root().as_posix(), expected_result);
        assert_eq!(ws.root_manifest().as_posix(), expect_result_manifest);
    }
}

// struct Packages {
//     packages: HashMap<PathBuf, Package>,
// }

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
