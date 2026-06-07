use async_trait::async_trait;
use crate::adapters::database::models::sessions::SessionRow;
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
        let rows: Vec<SessionRow> = sqlx::query_as(
            "SELECT token_hash, user_id, created_at FROM sessions WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| SessionGatewayError::Internal(e.to_string()))?;

        Ok(rows.into_iter().map(Session::from).collect())
    }

    async fn by_token_hash(&self, token_hash: &Hash) -> Result<Option<Session>, SessionGatewayError> {
        let row: Option<SessionRow> = sqlx::query_as(
            "SELECT token_hash, user_id, created_at FROM sessions WHERE token_hash = ?"
        )
        .bind(token_hash.0.as_slice())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| SessionGatewayError::Internal(e.to_string()))?;

        Ok(row.map(Session::from))
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
