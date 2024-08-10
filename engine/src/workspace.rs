use std::{path::PathBuf, rc::Rc};

use ts_rs::TS;

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

#[derive(TS)]
#[ts(export, rename = "rift::PluginManifest")]
pub struct PluginManifest {
    name: String,
    version: String,
    authors: Vec<String>,
    description: String,
    dependencies: String,
    metadata: String,
}
