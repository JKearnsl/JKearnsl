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
