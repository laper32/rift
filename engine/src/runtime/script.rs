use crate::blob::BlobHash;
use crate::paths::try_canonicalize;
use crate::util::errors::RiftResult;
use anyhow::{anyhow, bail, Context};
use std::collections::HashMap;
use std::fmt::format;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

pub struct ScriptManager {
    location_identity_map: Arc<RwLock<LocationIdentityMap>>,
}

struct LocationIdentityMap {
    contents: HashMap<ScriptIdentity, Arc<Vec<u8>>>,
    // 这个应该不用说？
    location_to_ids: HashMap<PathBuf, ScriptIdentity>,

    // 比如说多个binary才能组合成一个完整的script，这种情况还挺常见的。
    ids_to_locations: HashMap<ScriptIdentity, Vec<PathBuf>>,
}

impl ScriptManager {
    fn new() -> Self {
        Self {
            location_identity_map: Arc::new(RwLock::new(LocationIdentityMap {
                contents: HashMap::new(),
                location_to_ids: HashMap::new(),
                ids_to_locations: HashMap::new(),
            })),
        }
    }

    pub fn instance() -> &'static mut Self {
        static mut INSTANCE: once_cell::sync::Lazy<ScriptManager> =
            once_cell::sync::Lazy::new(|| ScriptManager::new());
        unsafe { &mut *INSTANCE }
    }

    pub async fn load(&self, path: &Path) -> RiftResult<(ScriptIdentity, Arc<Vec<u8>>)> {
        let path = try_canonicalize(path)?;

        {
            if let Some((script_id, contents)) = self.load_cached(&path)? {
                return Ok((script_id, contents));
            }
        }

        let contents = tokio::fs::read(&path).await.with_context(|| format!("Failed to read file {}", path.display()))?;
        let contents = Arc::new(contents);
        // TODO: Check whether it is binary file
        let script_id = ScriptIdentity::File(ulid::Ulid::new());
        let mut inner = self.location_identity_map.write().map_err(|_|anyhow!("Failed to acquire lock"))?;
        inner.contents.insert(script_id, contents.clone());
        inner.location_to_ids.insert(path.clone(), script_id);
        inner. ids_to_locations.entry(script_id).or_default().push(path.clone());
        tracing::debug!(path = %path.display(), %script_id, "Loaded to ScriptManager.");
        Ok((script_id, contents))
    }

    pub fn update(&self, script_id: ScriptIdentity, content: Arc<Vec<u8>>) -> RiftResult<()> {
        anyhow::ensure!(
            matches!(script_id, ScriptIdentity::File(_)),
            "Must be a file"
        );

        let mut inner = self
            .location_identity_map
            .write()
            .map_err(|_| anyhow::anyhow!("Failed to acquire lock"))?;

        inner.contents.insert(script_id, content.clone());
        let locations = inner.ids_to_locations.get_mut(&script_id).unwrap();
        for location in locations.iter() {
            tracing::debug!(path = %location.display(), %script_id, "edited file in ScriptManager");
        }
        Ok(())
    }

    pub fn load_cached(&self, path: &Path) -> RiftResult<Option<(ScriptIdentity, Arc<Vec<u8>>)>> {
        let path = try_canonicalize(path);
        let inner = self
            .location_identity_map
            .read()
            .map_err(|_| anyhow::anyhow!("failed to aquire lock"))?;
        let Some(file_id) = inner.location_to_ids.get(&path) else {
            return Ok(None);
        };
        let contents = inner.contents[file_id].clone();
        Ok(Some((*file_id, contents)))
    }

    pub fn read(&self, script_id: ScriptIdentity) -> RiftResult<Option<Arc<Vec<u8>>>> {
        Ok(self
            .location_identity_map
            .read()
            .map_err(|_| anyhow!("Failed to acquire lock"))?
            .contents
            .get(&script_id)
            .cloned())
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
)]
pub enum ScriptIdentity {
    /// 捆绑进去的，就自然是Blob咯。。
    Blob(BlobHash),

    /// 不用我说？
    File(ulid::Ulid),
}

impl ScriptIdentity {
    pub fn as_blob_hash(&self) -> RiftResult<BlobHash> {
        match self {
            ScriptIdentity::Blob(hash) => Ok(*hash),
            ScriptIdentity::File(_) => bail!("tried to get blob has for a file"),
        }
    }
}
impl std::fmt::Display for ScriptIdentity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blob(hash) => write!(f, "{hash}"),
            Self::File(ulid) => write!(f, "{}", ulid),
        }
    }
}

impl std::str::FromStr for ScriptIdentity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            26 => {
                let ulid = ulid::Ulid::from_str(s)?;
                Ok(Self::File(ulid))
            }
            64 => {
                let hash = s.parse()?;
                Ok(Self::Blob(hash))
            }
            _ => anyhow::bail!("invalid file ID: {}", s),
        }
    }
}
