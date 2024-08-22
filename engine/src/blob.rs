pub struct SaveBlobPermit<'a> {
    _permit: tokio::sync::SemaphorePermit<'a>,
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
pub struct BlobHash(blake3::Hash);

impl BlobHash {
    pub fn from_blake3(hash: blake3::Hash) -> Self {
        Self(hash)
    }

    pub fn to_blake3(&self) -> blake3::Hash {
        self.0
    }

    pub fn for_content(content: &[u8]) -> BlobHash {
        let hash = blake3::hash(content);
        BlobHash(hash)
    }

    pub fn validate_matches(&self, content: &[u8]) -> anyhow::Result<()> {
        let expected_hash = &self.0;
        let actual_hash = blake3::hash(content);
        anyhow::ensure!(
            expected_hash == &actual_hash,
            "blob does not match expected hash"
        );
        Ok(())
    }
}

impl std::fmt::Display for BlobHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_hex())
    }
}

impl std::str::FromStr for BlobHash {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hash = blake3::Hash::from_hex(s)?;
        Ok(Self(hash))
    }
}
