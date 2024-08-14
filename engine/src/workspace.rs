use crate::{
    errors::RiftResult,
    manifest::{find_root_manifest, read_manifest, EitherManifest, RiftManifest, VirtualManifest},
    package::Package,
};
use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
};

pub struct Packages {
    packages: HashMap<PathBuf, MaybePackage>,
}

#[derive(Debug)]
pub enum MaybePackage {
    Package(Package),
    Virtual(VirtualManifest),
    Rift(RiftManifest),
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
    pub fn load_packages(&mut self, manifest_path: &PathBuf) {
        let pkg = self.packages.load(&manifest_path);
        match pkg {
            Ok(MaybePackage::Package(pkg)) => match pkg.manifest() {
                // 因为project可能会有多个target，所以我们还得特殊处理
                crate::manifest::Manifest::Project(proj) => {
                    if proj.members.is_some() {
                        for ele in proj.members.clone().unwrap() {
                            let member_path = manifest_path.parent().unwrap().join(ele);
                            self.load_packages(&member_path);
                        }
                    }
                }
                crate::manifest::Manifest::Target(m) => {}
            },
            Ok(MaybePackage::Virtual(vm)) => match vm {
                VirtualManifest::Workspace(vm) => {
                    for ele in vm.members.clone() {
                        let member_path = manifest_path.parent().unwrap().join(ele);
                        self.load_packages(&member_path);
                    }
                }
                VirtualManifest::Folder(vm) => {
                    for ele in vm.members.clone().unwrap() {
                        let member_path = manifest_path.parent().unwrap().join(ele);
                        self.load_packages(&member_path);
                    }
                }
            },
            Ok(MaybePackage::Rift(rm)) => {
                // ...
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
    pub fn packages(&self) -> &Packages {
        &self.packages
    }
}

impl Packages {
    pub fn load(&mut self, manifest_path: &Path) -> RiftResult<&MaybePackage> {
        let key = manifest_path.parent().unwrap();
        match self.packages.entry(key.to_path_buf()) {
            Entry::Occupied(e) => Ok(e.into_mut()),
            Entry::Vacant(v) => {
                let manifest = read_manifest(manifest_path)?;

                Ok(v.insert(match manifest {
                    EitherManifest::Real(manifest) => {
                        MaybePackage::Package(Package::new(manifest, manifest_path))
                    }
                    EitherManifest::Virtual(vm) => MaybePackage::Virtual(vm),
                    EitherManifest::Rift(rm) => MaybePackage::Rift(rm),
                }))
            }
        }
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
        let mut ws = Workspace::new(&simple_workspace);
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
        ws.load_packages(&ws.root_manifest().to_path_buf().clone());
        let pkgs = ws.packages();
        pkgs.packages.iter().for_each(|(k, v)| {
            println!("{:?} {:?}", k, v);
        });
    }
}
