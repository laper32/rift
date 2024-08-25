use std::sync::Arc;

use crate::{
    runtime::specifier::{self, RiftImportSpecifier, RiftModuleSpecifier},
    util::errors::RiftResult,
};

pub struct ForwardManager {
    tx: tokio::sync::mpsc::UnboundedSender<Forward>,
}

impl ForwardManager {
    fn new() -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                tokio::spawn(async move {
                    match message {
                        Forward::ReadSpecifierContents {
                            specifier,
                            contents_tx,
                        } => {
                            let contents = specifier::read_specifier_contents(&specifier);
                            let _ = contents_tx.send(contents);
                        }
                        Forward::ResolveSpecifier {
                            specifier,
                            referrer,
                            resolved_tx,
                        } => {
                            let resolved = specifier::resolve(&specifier, &referrer);
                            let _ = resolved_tx.send(resolved);
                        }
                    }
                });
            }
        });
        Self { tx }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<ForwardManager> =
            once_cell::sync::Lazy::new(|| ForwardManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub fn read_specifier_contents(
        &self,
        specifier: RiftModuleSpecifier,
    ) -> RiftResult<Arc<Vec<u8>>> {
        let (contents_tx, contents_rx) = std::sync::mpsc::channel();
        self.tx.send(Forward::ReadSpecifierContents {
            specifier,
            contents_tx,
        })?;
        let contents = contents_rx.recv()??;
        Ok(contents)
    }

    pub fn resolve_specifier(
        &self,
        specifier: RiftImportSpecifier,
        referrer: RiftModuleSpecifier,
    ) -> RiftResult<RiftModuleSpecifier> {
        let (resolved_tx, resolved_rx) = std::sync::mpsc::channel();
        self.tx.send(Forward::ResolveSpecifier {
            specifier,
            referrer,
            resolved_tx,
        })?;
        let resolved = resolved_rx.recv()??;
        Ok(resolved)
    }
}

pub enum Forward {
    ReadSpecifierContents {
        specifier: RiftModuleSpecifier,
        contents_tx: std::sync::mpsc::Sender<RiftResult<Arc<Vec<u8>>>>,
    },
    ResolveSpecifier {
        specifier: RiftImportSpecifier,
        referrer: RiftModuleSpecifier,
        resolved_tx: std::sync::mpsc::Sender<RiftResult<RiftModuleSpecifier>>,
    },
}
