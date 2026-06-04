use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Hash(pub Vec<u8>);

impl Hash {
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for Hash {
    fn from(v: Vec<u8>) -> Self {
        Hash(v)
    }
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
