use crate::{
    errors::RiftResult,
    manifest::{
        find_root_manifest, read_manifest, EitherManifest, Manifest, RiftManifest, VirtualManifest,
        MANIFEST_IDENTIFIER,
    },
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

    pub fn scan_all_possible_packages(&mut self, manifest_path: &Path) {
        let manifest = read_manifest(manifest_path);
        match manifest {
            Ok(em) => match em {
                // TODO: 还没解决project和target在一个manifest的情况
                EitherManifest::Real(m) => match m {
                    Manifest::Project(p) => {
                        if self.packages.contains_key(manifest_path) {
                            return;
                        }
                        let _ = self.load(manifest_path);
                        if p.members.is_some() {
                            p.members.unwrap().iter().for_each(|member| {
                                let full_path = manifest_path
                                    .parent()
                                    .unwrap()
                                    .join(member)
                                    .join(MANIFEST_IDENTIFIER);
                                self.scan_all_possible_packages(&full_path);
                            });
                        }
                    }
                    Manifest::Target(t) => {
                        if self.packages.contains_key(manifest_path) {
                            return;
                        }
                        let _ = self.load(manifest_path);
                    }
                },
                EitherManifest::Virtual(vm) => match vm {
                    VirtualManifest::Workspace(w) => {
                        if self.packages.contains_key(manifest_path) {
                            return;
                        }
                        w.members.iter().for_each(|member| {
                            let full_path = manifest_path
                                .parent()
                                .unwrap()
                                .join(member)
                                .join(MANIFEST_IDENTIFIER);
                            self.scan_all_possible_packages(&full_path);
                        })
                    }
                    VirtualManifest::Folder(f) => {
                        if self.packages.contains_key(manifest_path) {
                            return;
                        }
                        f.members.unwrap().iter().for_each(|member| {
                            let full_path = manifest_path
                                .parent()
                                .unwrap()
                                .join(member)
                                .join(MANIFEST_IDENTIFIER);
                            self.scan_all_possible_packages(&full_path);
                        });
                        let _ = self.load(manifest_path);
                    }
                },
                EitherManifest::Rift(_rm) => {
                    if self.packages.contains_key(manifest_path) {
                        return;
                    }
                    let _ = self.load(manifest_path);
                }
            },
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

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
    }
    #[test]
    fn test_simple_target() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("01_simple_target")
            .join("Rift.toml"); // 这里只有一个Workspace和Project，没有别的东西
        let mut ws = Workspace::new(&simple_workspace);
        ws.packages.scan_all_possible_packages(&simple_workspace);
        ws.packages.packages.iter().for_each(|(k, v)| {
            println!("Key: {:?}", k);
            println!("Value: {:?}", v);
        });
    }
    #[test]
    fn test_single_target_with_project() {
        let our_project_root = util::get_cargo_project_root().unwrap();
        let simple_workspace = our_project_root
            .join("sample")
            .join("02_single_target_with_project")
            .join("Rift.toml"); // 这里只有一个Workspace和Project，没有别的东西
        let mut ws = Workspace::new(&simple_workspace);
        ws.packages.scan_all_possible_packages(&simple_workspace);
        ws.packages.packages.iter().for_each(|(k, v)| {
            println!("Key: {:?}", k);
            println!("Value: {:?}", v);
        });
    }
}
