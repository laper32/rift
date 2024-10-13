use std::sync::Arc;

use engine::shared::errors::RiftResult;

use crate::specifier::{self, RiftImportSpecifier, RiftModuleSpecifier};

pub struct ForwardManager {
    tx: tokio::sync::mpsc::UnboundedSender<Forward>,
}

impl ForwardManager {
    fn new() -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::spawn(async move {
            while let Some(fwd) = rx.recv().await {
                tokio::spawn(async move {
                    match fwd {
                        Forward::ResolveSpecifier {
                            specifier,
                            referrer,
                            resolved_tx,
                        } => {
                            let resolved = specifier::resolve(&specifier, &referrer);
                            let _ = resolved_tx.send(resolved);
                        }
                        Forward::ResolvePackage {} => todo!(),
                        Forward::ReadSpecifierContents {
                            specifier,
                            contents_tx,
                        } => {
                            let contents = specifier::read_specifier_contents(&specifier);
                            let _ = contents_tx.send(contents);
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

    pub fn resolve_specifier(
        &mut self,
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
    pub fn resolve_package() -> RiftResult<RiftModuleSpecifier> {
        todo!()
    }
    pub fn read_specifier_contents(
        &mut self,
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
}

pub enum Forward {
    ResolveSpecifier {
        specifier: RiftImportSpecifier,
        referrer: RiftModuleSpecifier,
        resolved_tx: std::sync::mpsc::Sender<RiftResult<RiftModuleSpecifier>>,
    },
    ResolvePackage {},
    ReadSpecifierContents {
        specifier: RiftModuleSpecifier,
        contents_tx: std::sync::mpsc::Sender<RiftResult<Arc<Vec<u8>>>>,
    },
}
