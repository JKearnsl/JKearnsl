use argon2::{Argon2, Params, password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString}};
use async_trait::async_trait;
use crate::application::common::hasher::Hasher;
use crate::domain::models::hash::Hash;

pub struct Argon2PasswordHasher {
    hasher: Argon2<'static>,
}

impl Argon2PasswordHasher {
    pub fn new() -> Self {
        Self {
            hasher: Argon2::new(
                argon2::Algorithm::Argon2id,
                argon2::Version::V0x13,
                Params::new(2048, 64, 4, Some(32)).expect("valid Argon2 params"),
            ),
        }
    }
}

#[async_trait]
impl Hasher for Argon2PasswordHasher {
    async fn hash(&self, bytes: &[u8]) -> Hash {
        let hasher = self.hasher.clone();
        let bytes = bytes.to_vec();
        let phc_bytes = tokio::task::spawn_blocking(move || {
            let salt = SaltString::encode_b64(&rand::random::<[u8; 16]>()).expect("salt encoding failed");
            hasher
                .hash_password(&bytes, &salt)
                .expect("failed to hash")
                .to_string()
                .into_bytes()
        })
        .await
        .expect("spawn_blocking failed");
        Hash(phc_bytes)
    }

    async fn verify(&self, bytes: &[u8], hash: &[u8]) -> bool {
        let hasher = self.hasher.clone();
        let bytes = bytes.to_vec();
        let hash = hash.to_vec();
        tokio::task::spawn_blocking(move || {
            let hash_str = std::str::from_utf8(&hash).unwrap_or("");
            match PasswordHash::new(hash_str) {
                Ok(parsed) => hasher.verify_password(&bytes, &parsed).is_ok(),
                Err(_) => false,
            }
        })
        .await
        .unwrap_or(false)
    }
}
