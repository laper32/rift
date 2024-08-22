use sha2::Digest;

use crate::util::errors::RiftResult;

#[derive(Debug, Clone, Hash, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[serde_with::serde_as]
pub enum Hash {
    Sha256 {
        #[serde_as(as = "serde_with::hex::Hex")]
        value: Vec<u8>,
    },
}

impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Hash::Sha256 { value } => write!(f, "sha256:{}", hex::encode(value)),
        }
    }
}

pub enum Hasher {
    Sha256(sha2::Sha256),
}

impl Hasher {
    pub fn new_sha256() -> Self {
        Self::Sha256(sha2::Sha256::new())
    }
    pub fn for_hash(hash: &Hash) -> Self {
        match hash {
            Hash::Sha256 { .. } => Self::Sha256(sha2::Sha256::new()),
        }
    }
    pub fn update(&mut self, bytes: &[u8]) {
        match self {
            Self::Sha256(hasher) => hasher.update(bytes),
        }
    }

    pub fn finish(self) -> RiftResult<Hash> {
        match self {
            Self::Sha256(hasher) => {
                let hash = hasher.finalize();
                Ok(Hash::Sha256 {
                    value: hash.as_slice().to_vec(),
                })
            }
        }
    }
}
