use std::{path::PathBuf, rc::Rc};

#[allow(dead_code)]
pub enum MaybePackage {
    Package, // Target, Project, Plugin
    Virtual, // Workspace, Folder
}

#[allow(dead_code)]
pub struct Package {
    inner: Rc<PackageInner>,
}

#[allow(dead_code)]
pub struct PackageInner {
    manifest_path: PathBuf,
}
