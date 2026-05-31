use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::domain::models::project::ProjectId;

pub const PROJECT_TABLE: &str = "projects";

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct Project {
    pub id: ProjectId,
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[async_trait]
impl CreateIFNotExists for Project {
    async fn create_if_not_exists(db_pool: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS projects (
                id CHAR(16) PRIMARY KEY,
                title VARCHAR(128) NOT NULL,
                description VARCHAR(256) NOT NULL,
                url VARCHAR(2048),
                created_at TIMESTAMP WITH TIME ZONE NOT NULL
            );"
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }
}
