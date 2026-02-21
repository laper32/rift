use crate::manifest::{real::Manifest, rift::RiftManifest, r#virtual::VirtualManifest};

pub mod converter;
pub mod real;
pub mod rift;
pub mod r#virtual;

pub use converter::{PackageKind, convert_toml_to_manifest};

pub enum EitherManifest {
    Virtual(VirtualManifest),
    Real(Manifest),
    Rift(RiftManifest),
}
