use sha2::Digest;
use sqlx::Row;
use crate::application::common::id_provider::IdProvider;
use crate::domain::models::user::UserId;

pub struct IdTokenProvider {
    token: Option<String>,
    user_id: Option<UserId>,
    username: Option<String>,
    is_auth: bool,
}

impl IdTokenProvider {
    pub async fn new(token: Option<String>, token_processor: &TokenProcessor) -> Self {
        match token {
            Some(token) => match token_processor.get_token_session(&token).await {
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
    pool: sqlx::SqlitePool,
}

impl TokenProcessor {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }

    fn decode_hex(hex: &str) -> Option<[u8; 32]> {
        if hex.len() != 64 { return None; }
        let mut bytes = [0u8; 32];
        for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
            bytes[i] = u8::from_str_radix(std::str::from_utf8(chunk).ok()?, 16).ok()?;
        }
        Some(bytes)
    }

    pub async fn get_token_session(&self, hex: &str) -> Option<(UserId, String)> {
        let raw = Self::decode_hex(hex)?;
        let hash: [u8; 32] = sha2::Sha256::digest(raw).into();
        let row = sqlx::query(
            "SELECT u.id, u.username FROM sessions s JOIN users u ON s.user_id = u.id WHERE s.token_hash = ?"
        )
        .bind(hash.as_slice())
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()?;
        let user_id: UserId = row.try_get("id").ok()?;
        let username: String = row.try_get("username").ok()?;
        Some((user_id, username))
    }

    pub async fn remove_token_session(&self, hex: &str) {
        if let Some(raw) = Self::decode_hex(hex) {
            let hash: [u8; 32] = sha2::Sha256::digest(raw).into();
            sqlx::query("DELETE FROM sessions WHERE token_hash = ?")
                .bind(hash.as_slice())
                .execute(&self.pool)
                .await
                .ok();
        }
    }
}
