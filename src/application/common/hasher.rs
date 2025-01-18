use async_trait::async_trait;
use crate::domain::models::hash::Hash;

#[async_trait]
pub trait Hasher {
    async fn hash(&self, value: &str) -> Hash;
    async fn verify(&self, value: &str, hash: &Hash) -> bool;
}


#[cfg(test)]
pub mod test {
    use super::*;

    pub struct MockHasher;

    #[async_trait]
    impl Hasher for MockHasher {
        async fn hash(&self, value: &str) -> Hash {
            let mut data = [0; Hash::SIZE];
            let bytes = value.as_bytes();
            let len = bytes.len().min(Hash::SIZE);
            data[..len].copy_from_slice(&bytes[..len]);
            Hash(data)
        }

        async fn verify(&self, value: &str, hash: &Hash) -> bool {
            self.hash(value).await == *hash
        }
    }
}
