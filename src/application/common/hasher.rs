use async_trait::async_trait;
use crate::domain::models::hash::Hash;

#[async_trait]
pub trait Hasher: Send + Sync {
    async fn hash(&self, bytes: &[u8]) -> Hash;
    async fn verify(&self, bytes: &[u8], hash: &Hash) -> bool;
}


#[cfg(test)]
pub mod test {
    use super::*;

    pub struct MockHasher;

    #[async_trait]
    impl Hasher for MockHasher {
        async fn hash(&self, bytes: &[u8]) -> Hash {
            Hash(bytes.to_vec())
        }

        async fn verify(&self, bytes: &[u8], hash: &Hash) -> bool {
            bytes == hash.0.as_slice()
        }
    }
}
