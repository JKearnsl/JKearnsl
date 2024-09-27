use chrono::{DateTime, Utc};
use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use crate::domain::models::project::{
    PROJECT_ID_SIZE, 
    PROJECT_TITLE_LENGTH, 
    PROJECT_DESCRIPTION_LENGTH, 
    PROJECT_URL_LENGTH_MAX, 
    ProjectId
};

pub const PROJECT_TABLE: &str = "projects";

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct Project {
    pub id: ProjectId,
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl CreateIFNotExists for Project {
    async fn create_if_not_exists(db_pool: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(format!(
            "CREATE TABLE IF NOT EXISTS {table} (
                id CHAR({id_size}) PRIMARY KEY,
                title VARCHAR({title_max}) NOT NULL,
                description VARCHAR({description_max}) NOT NULL,
                url VARCHAR({url_max}) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL
            );",
            table = PROJECT_TABLE,
            id_size = PROJECT_ID_SIZE,
            title_max = PROJECT_TITLE_LENGTH.1,
            description_max = PROJECT_DESCRIPTION_LENGTH.1,
            url_max = PROJECT_URL_LENGTH_MAX
        ).as_str())
            .execute(db_pool)
            .await?;
        Ok(())
    }
}
