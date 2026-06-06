use sha2::{Sha256, Digest};
use crate::application::common::{
    id_provider::IdProvider,
    session_gateway::{SessionReader, SessionRemover},
    user_gateway::UserReader,
};
use crate::adapters::database::{session::SqliteSessionGateway, user::SqliteUserGateway};
use crate::domain::models::{hash::Hash, user::UserId};

pub struct IdTokenProvider {
    token: Option<String>,
    user_id: Option<UserId>,
    username: Option<String>,
    is_auth: bool,
}

impl IdTokenProvider {
    pub async fn new(token: Option<String>, token_processor: &TokenProcessor) -> Self {
        match token {
            Some(token) => match token_processor.get_session(&token).await {
                Some((user_id, username)) => Self { token: Some(token), user_id: Some(user_id), username: Some(username), is_auth: true },
                None => Self { token: None, user_id: None, username: None, is_auth: false },
            },
            None => Self { token: None, user_id: None, username: None, is_auth: false },
        }
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
}

impl IdProvider for IdTokenProvider {
    fn session(&self) -> Option<&String> { self.token.as_ref() }
    fn user_id(&self) -> Option<&UserId> { self.user_id.as_ref() }
    fn username(&self) -> Option<&String> { self.username.as_ref() }
    fn is_auth(&self) -> bool { self.is_auth }
}


pub struct TokenProcessor {
    session_gateway: SqliteSessionGateway,
    user_gateway: SqliteUserGateway,
}

impl TokenProcessor {
    pub fn new(session_gateway: SqliteSessionGateway, user_gateway: SqliteUserGateway) -> Self {
        Self { session_gateway, user_gateway }
    }

    fn decode_hex(hex: &str) -> Option<[u8; 32]> {
        if hex.len() != 64 { return None; }
        let mut bytes = [0u8; 32];
        for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
            bytes[i] = u8::from_str_radix(std::str::from_utf8(chunk).ok()?, 16).ok()?;
        }
        Some(bytes)
    }

    pub async fn get_session(&self, hex: &str) -> Option<(UserId, String)> {
        let raw = Self::decode_hex(hex)?;
        let hash: [u8; 32] = Sha256::digest(raw).into();
        let session = self.session_gateway.by_token_hash(&Hash(hash.to_vec())).await.ok()??;
        let user = self.user_gateway.by_id(&session.user_id).await.ok()??;
        Some((user.id, user.username))
    }

    pub async fn remove_session(&self, hex: &str) {
        if let Some(raw) = Self::decode_hex(hex) {
            let hash: [u8; 32] = Sha256::digest(raw).into();
            self.session_gateway.remove_by_token_hash(&Hash(hash.to_vec())).await.ok();
        }
    }
}
