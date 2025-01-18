use serde::{Deserialize, Deserializer, Serialize, Serializer};

const HASH_LENGTH: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hash(pub [u8; HASH_LENGTH]);

impl Hash {
    pub const SIZE: usize = HASH_LENGTH;
    
    pub fn to_string(&self) -> String {
        hex::encode(&self.0)
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

impl Deserialize for Hash {
    fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let bytes = hex::decode(s).map_err(serde::de::Error::custom)?;
        if bytes.len() != HASH_LENGTH {
            return Err(serde::de::Error::custom(format!(
                "expected {} bytes, got {}",
                HASH_LENGTH,
                bytes.len()
            )));
        }
        let mut hash = [0; HASH_LENGTH];
        hash.copy_from_slice(&bytes);
        Ok(Hash(hash))
    }
}