use core::option::Option;

use async_trait::async_trait;

use crate::adapters::database::pool::DbPool;
use crate::application::common::project_gateway::{
    ProjectGateway as ProjectGatewayTrait,
    ProjectReader,
    ProjectRemover,
    ProjectWriter
};
use crate::domain::models::project::{Project as ProjectDomain, ProjectId};
use crate::adapters::database::models::projects::Project;


pub struct ProjectGateway{
    db: DbPool,
}

impl ProjectGateway {
    pub fn new(db: DbPool) -> Self {
        ProjectGateway {
            db,
        }
    }
}

#[async_trait]
impl ProjectReader for ProjectGateway {
    async fn get_project(&self, project_id: &ProjectId) -> Option<ProjectDomain> {
        let row: Option<Project> = sqlx::query_as(
            "SELECT * FROM projects WHERE id = $1"
        )
            .bind(project_id)
            .fetch_optional(&self.db).await.ok().flatten();

        match row {
            None => None,
            Some(row) => Some(map_project_model_to_domain(row))
        }
    }

    async fn get_projects_range(&self, limit: &u64, offset: &u64) -> Vec<ProjectDomain> {
        let rows: Vec<Project> = sqlx::query_as(
            "SELECT * FROM projects LIMIT $1 OFFSET $2"
        )
            .bind(*limit as i64)
            .bind(*offset as i64)
            .fetch_all(&self.db).await.unwrap_or_default();

        rows.into_iter().map(|row| map_project_model_to_domain(row)).collect()
    }
}

#[async_trait]
impl ProjectWriter for ProjectGateway {
    async fn save_project(&self, project: &ProjectDomain) {
        sqlx::query(
            "INSERT INTO projects (id, title, description, url, created_at) \
             VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET \
             title = $2, description = $3, url = $4, created_at = $5"
        )
            .bind(&project.id)
            .bind(&project.title)
            .bind(&project.description)
            .bind(&project.url)
            .bind(&project.created_at)
            .execute(&self.db).await.ok();
    }
}

#[async_trait]
impl ProjectRemover for ProjectGateway {
    async fn remove_project(&self, project_id: &ProjectId) {
        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(project_id)
            .execute(&self.db).await.ok();
    }
}

fn map_project_model_to_domain(project: Project) -> ProjectDomain {
    ProjectDomain {
        id: project.id,
        title: project.title,
        description: project.description,
        created_at: project.created_at,
        url: project.url,
    }
}

impl ProjectGatewayTrait for ProjectGateway {}
