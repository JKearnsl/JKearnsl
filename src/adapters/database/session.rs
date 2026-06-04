use async_trait::async_trait;
use sha2::{Sha256, Digest};
use sqlx::Row;
use crate::application::common::session_gateway::{SessionGateway, SessionReader, SessionRemover, SessionVacuum, SessionWriter};
use crate::domain::models::hash::Hash;
use crate::domain::models::session::Session;
use crate::domain::models::user::UserId;

#[derive(Clone)]
pub struct SqliteSessionGateway {
    pool: sqlx::SqlitePool,
}

impl SqliteSessionGateway {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionWriter for SqliteSessionGateway {
    async fn save(&self, raw_token: &[u8; 32], user_id: &UserId) {
        let hash: [u8; 32] = Sha256::digest(raw_token).into();
        let created_at = chrono::Utc::now().timestamp();
        sqlx::query("INSERT INTO sessions (token_hash, user_id, created_at) VALUES (?, ?, ?)")
            .bind(hash.as_slice())
            .bind(user_id)
            .bind(created_at)
            .execute(&self.pool)
            .await
            .ok();
    }
}

#[async_trait]
impl SessionRemover for SqliteSessionGateway {
    async fn remove_by_token(&self, raw_token: &[u8; 32]) {
        let hash: [u8; 32] = Sha256::digest(raw_token).into();
        sqlx::query("DELETE FROM sessions WHERE token_hash = ?")
            .bind(hash.as_slice())
            .execute(&self.pool)
            .await
            .ok();
    }
}

#[async_trait]
impl SessionReader for SqliteSessionGateway {
    async fn get_by_user_id(&self, user_id: &UserId) -> Vec<Session> {
        let rows = match sqlx::query(
            "SELECT token_hash, user_id, created_at FROM sessions WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        {
            Ok(r) => r,
            Err(_) => return vec![],
        };

        rows.into_iter()
            .filter_map(|row| {
                let token_hash: Vec<u8> = row.try_get("token_hash").ok()?;
                let user_id: UserId = row.try_get("user_id").ok()?;
                let created_at_ts: i64 = row.try_get("created_at").ok()?;
                let created_at = chrono::DateTime::from_timestamp(created_at_ts, 0)?.with_timezone(&chrono::Utc);
                Some(Session { token_hash: Hash(token_hash), user_id, created_at })
            })
            .collect()
    }
}

#[async_trait]
impl SessionVacuum for SqliteSessionGateway {
    async fn remove_older_than(&self, max_age_secs: i64) -> u64 {
        let cutoff = chrono::Utc::now().timestamp() - max_age_secs;
        sqlx::query("DELETE FROM sessions WHERE created_at < ?")
            .bind(cutoff)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected())
            .unwrap_or(0)
    }
}

impl SessionGateway for SqliteSessionGateway {}
