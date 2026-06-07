use async_trait::async_trait;
use chrono::Utc;
use crate::adapters::database::models::users::UserRow;
use crate::application::common::user_gateway::{UserGateway, UserGatewayError, UserReader, UserRemover, UserWriter};
use crate::domain::models::user::{User, UserId};

#[derive(Clone)]
pub struct SqliteUserGateway {
    pool: sqlx::SqlitePool,
}

impl SqliteUserGateway {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserReader for SqliteUserGateway {
    async fn by_id(&self, user_id: &UserId) -> Result<Option<User>, UserGatewayError> {
        let row: Option<UserRow> = sqlx::query_as(
            "SELECT id, username, password_phc FROM users WHERE id = ?"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserGatewayError::Internal(e.to_string()))?;

        Ok(row.map(User::from))
    }

    async fn by_username(&self, username: &str) -> Result<Option<User>, UserGatewayError> {
        let row: Option<UserRow> = sqlx::query_as(
            "SELECT id, username, password_phc FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| UserGatewayError::Internal(e.to_string()))?;

        Ok(row.map(User::from))
    }

    async fn list(&self) -> Result<Vec<User>, UserGatewayError> {
        let rows: Vec<UserRow> = sqlx::query_as(
            "SELECT id, username, password_phc FROM users"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| UserGatewayError::Internal(e.to_string()))?;

        Ok(rows.into_iter().map(User::from).collect())
    }
}

#[async_trait]
impl UserWriter for SqliteUserGateway {
    async fn save(&self, user: &User) -> Result<(), UserGatewayError> {
        sqlx::query(
            "INSERT INTO users (id, username, password_phc, created_at) VALUES (?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
               username = excluded.username,
               password_phc = excluded.password_phc"
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(Utc::now().timestamp())
        .execute(&self.pool)
        .await
        .map(|_| ())
        .map_err(|e| UserGatewayError::Internal(e.to_string()))
    }
}

#[async_trait]
impl UserRemover for SqliteUserGateway {
    async fn remove(&self, user_id: &UserId) -> Result<(), UserGatewayError> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|e| UserGatewayError::Internal(e.to_string()))
    }
}

impl UserGateway for SqliteUserGateway {}
