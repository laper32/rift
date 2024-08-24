use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use anyhow::Context;

use crate::{paths::try_canonicalize, util::errors::RiftResult};

pub struct ScriptManager {
    scripts: Arc<RwLock<ScriptMap>>,
}

#[derive(Debug, Clone, Default)]
struct ScriptMap {
    contents: HashMap<uuid::Uuid, Arc<Vec<u8>>>,
    identities: HashMap<PathBuf, uuid::Uuid>,
}

impl ScriptManager {
    fn new() -> Self {
        Self {
            scripts: Default::default(),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<ScriptManager> =
            once_cell::sync::Lazy::new(|| ScriptManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub async fn load(&self, path: &Path) -> RiftResult<(uuid::Uuid, Arc<Vec<u8>>)> {
        let path = try_canonicalize(path)?;

        if let Some((uuid, contents)) = self.load_cached(&path)? {
            return Ok((uuid, contents));
        }

        let contents = tokio::fs::read(&path)
            .await
            .with_context(|| format!("failed to read file {}", path.display()))?;
        let contents = Arc::new(contents);

        let uuid = uuid::Uuid::new_v4();
        let mut inner = self
            .scripts
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock, err: {e}"))?;
        inner.contents.insert(uuid, contents.clone());
        inner.identities.insert(path.clone(), uuid);
        tracing::debug!(path = %path.display(), %uuid, "loaded file into VFS");
        Ok((uuid, contents))
    }

    pub fn update(&self, uuid: uuid::Uuid, contents: Arc<Vec<u8>>) -> RiftResult<()> {
        let mut inner = self
            .scripts
            .write()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock, err: {e}"))?;
        inner.contents.insert(uuid, contents.clone());
        Ok(())
    }

    pub fn load_cached(&self, path: &Path) -> RiftResult<Option<(uuid::Uuid, Arc<Vec<u8>>)>> {
        let path = try_canonicalize(path)?;
        let inner = self
            .scripts
            .read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock, err: {e}"))?;
        let Some(uuid) = inner.identities.get(path.as_path()) else {
            return Ok(None);
        };
        let content = inner.contents[uuid].clone();
        Ok(Some((*uuid, content)))
    }

    pub fn read(&self, path: &Path) -> RiftResult<Option<Arc<Vec<u8>>>> {
        let path = try_canonicalize(path)?;
        let inner = self
            .scripts
            .read()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock, err: {e}"))?;
        match inner.identities.get(path.as_path()) {
            Some(uuid) => {
                let value = inner.contents.get(uuid);
                Ok(value.cloned())
            }
            None => anyhow::bail!("No such script."),
        }
    }
}
