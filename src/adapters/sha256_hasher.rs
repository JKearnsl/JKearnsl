use async_trait::async_trait;
use sha2::{Sha256, Digest};
use crate::application::common::hasher::Hasher;
use crate::domain::models::hash::Hash;

pub struct Sha256Hasher;

#[async_trait]
impl Hasher for Sha256Hasher {
    async fn hash(&self, bytes: &[u8]) -> Hash {
        Hash(Sha256::digest(bytes).to_vec())
    }

    async fn verify(&self, bytes: &[u8], hash: &Hash) -> bool {
        Sha256::digest(bytes).as_slice() == hash.0.as_slice()
    }
}
