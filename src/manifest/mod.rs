use crate::manifest::{real::Manifest, rift::RiftManifest, r#virtual::VirtualManifest};

mod real;
mod rift;
mod r#virtual;

pub enum EitherManifest {
    Virtual(VirtualManifest),
    Real(Manifest),
    Rift(RiftManifest),
}
