use async_trait::async_trait;
use sqlx::Row;
use crate::application::common::session_gateway::{
    SessionGateway, SessionGatewayError, SessionReader, SessionRemover, SessionWriter,
};
use crate::domain::models::{
    hash::Hash,
    session::Session,
    user::UserId,
};

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
    async fn save(&self, model: Session) -> Result<(), SessionGatewayError> {
        sqlx::query("INSERT INTO sessions (token_hash, user_id, created_at) VALUES (?, ?, ?)")
            .bind(model.token_hash.0.as_slice())
            .bind(model.user_id)
            .bind(model.created_at.timestamp())
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| SessionGatewayError::Internal(e.to_string()))
    }
}

#[async_trait]
impl SessionReader for SqliteSessionGateway {
    async fn by_user_id(&self, user_id: &UserId) -> Result<Vec<Session>, SessionGatewayError> {
        let rows = sqlx::query(
            "SELECT token_hash, user_id, created_at FROM sessions WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| SessionGatewayError::Internal(e.to_string()))?;

        Ok(rows.into_iter()
            .filter_map(|row| {
                let token_hash: Vec<u8> = row.try_get("token_hash").ok()?;
                let user_id: UserId = row.try_get("user_id").ok()?;
                let created_at_ts: i64 = row.try_get("created_at").ok()?;
                let created_at = chrono::DateTime::from_timestamp(created_at_ts, 0)?.with_timezone(&chrono::Utc);
                Some(Session { token_hash: Hash(token_hash), user_id, created_at })
            })
            .collect())
    }

    async fn by_token_hash(&self, token_hash: &Hash) -> Result<Option<Session>, SessionGatewayError> {
        let row = sqlx::query(
            "SELECT token_hash, user_id, created_at FROM sessions WHERE token_hash = ?"
        )
        .bind(token_hash.0.as_slice())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| SessionGatewayError::Internal(e.to_string()))?;

        Ok(row.and_then(|r| {
            let token_hash: Vec<u8> = r.try_get("token_hash").ok()?;
            let user_id: UserId = r.try_get("user_id").ok()?;
            let created_at_ts: i64 = r.try_get("created_at").ok()?;
            let created_at = chrono::DateTime::from_timestamp(created_at_ts, 0)?.with_timezone(&chrono::Utc);
            Some(Session { token_hash: Hash(token_hash), user_id, created_at })
        }))
    }
}

#[async_trait]
impl SessionRemover for SqliteSessionGateway {
    async fn remove_by_token_hash(&self, token_hash: &Hash) -> Result<(), SessionGatewayError> {
        sqlx::query("DELETE FROM sessions WHERE token_hash = ?")
            .bind(token_hash.0.as_slice())
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| SessionGatewayError::Internal(e.to_string()))
    }

    async fn remove_older_than(&self, max_age_secs: i64) -> Result<u64, SessionGatewayError> {
        let cutoff = chrono::Utc::now().timestamp() - max_age_secs;
        sqlx::query("DELETE FROM sessions WHERE created_at < ?")
            .bind(cutoff)
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected())
            .map_err(|e| SessionGatewayError::Internal(e.to_string()))
    }
}

impl SessionGateway for SqliteSessionGateway {}
