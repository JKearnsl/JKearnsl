use serde::{Deserialize, Deserializer, Serialize, Serializer};

const HASH_LENGTH: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hash(pub [u8; HASH_LENGTH]);

impl Hash {
    pub const SIZE: usize = HASH_LENGTH;
}

impl Serialize for Hash {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = <[u8; HASH_LENGTH]>::deserialize(deserializer)?;
        Ok(Hash(bytes))
    }
}

impl TryFrom<&[u8]> for Hash {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != HASH_LENGTH {
            return Err(format!(
                "expected {} bytes, got {}",
                HASH_LENGTH,
                value.len()
            ));
        }
        let mut hash = [0; HASH_LENGTH];
        hash.copy_from_slice(value);
        Ok(Hash(hash))
    }
}
