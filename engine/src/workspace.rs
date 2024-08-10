use std::{path::PathBuf, rc::Rc};

pub enum MaybePackage {
    Package, // Target, Project, Plugin
    Virtual, // Workspace, Folder
}

pub struct Package {
    inner: Rc<PackageInner>,
}

pub struct PackageInner {
    manifest_path: PathBuf,
}
