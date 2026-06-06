use async_trait::async_trait;
use sqlx::Row;
use crate::application::common::user_gateway::{UserGateway, UserReader, UserRemover, UserWriter};
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
    async fn get_by_username(&self, username: &str) -> Option<User> {
        let row = sqlx::query("SELECT id, username, password_phc FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .ok()
            .flatten()?;

        Some(User {
            id: row.try_get("id").ok()?,
            username: row.try_get("username").ok()?,
            password_hash: row.try_get("password_phc").ok()?,
        })
    }

    async fn get_all(&self) -> Vec<User> {
        let rows = match sqlx::query("SELECT id, username, password_phc FROM users")
            .fetch_all(&self.pool)
            .await
        {
            Ok(r) => r,
            Err(_) => return vec![],
        };

        rows.into_iter()
            .filter_map(|row| {
                Some(User {
                    id: row.try_get("id").ok()?,
                    username: row.try_get("username").ok()?,
                    password_hash: row.try_get("password_phc").ok()?,
                })
            })
            .collect()
    }
}

#[async_trait]
impl UserWriter for SqliteUserGateway {
    async fn save(&self, user: &User) {
        sqlx::query(
            "INSERT INTO users (id, username, password_phc) VALUES (?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
               username = excluded.username,
               password_phc = excluded.password_phc"
        )
        .bind(&user.id)
        .bind(&user.username)
        .bind(&user.password_hash)
        .execute(&self.pool)
        .await
        .ok();
    }
}

#[async_trait]
impl UserRemover for SqliteUserGateway {
    async fn remove(&self, user_id: &UserId) {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .ok();
    }
}

impl UserGateway for SqliteUserGateway {}
