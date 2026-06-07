use chrono::DateTime;
use crate::domain::models::project::{Project as ProjectDomain, ProjectId};

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct ProjectRow {
    pub id: ProjectId,
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub created_at: i64,
}

impl From<ProjectRow> for ProjectDomain {
    fn from(row: ProjectRow) -> Self {
        ProjectDomain {
            id: row.id,
            title: row.title,
            description: row.description,
            url: row.url,
            created_at: DateTime::from_timestamp(row.created_at, 0)
                .unwrap_or_default()
                .with_timezone(&chrono::Utc),
        }
    }
}
