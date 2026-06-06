use async_trait::async_trait;
use crate::adapters::database::pool::DbPool;
use crate::application::common::project_gateway::{
    ProjectGateway as ProjectGatewayTrait, ProjectGatewayError, ProjectReader, ProjectRemover, ProjectWriter,
};
use crate::domain::models::project::{Project as ProjectDomain, ProjectId};
use crate::adapters::database::models::projects::Project;


pub struct ProjectGateway {
    db: DbPool,
}

impl ProjectGateway {
    pub fn new(db: DbPool) -> Self {
        ProjectGateway { db }
    }
}

#[async_trait]
impl ProjectReader for ProjectGateway {
    async fn by_id(&self, id: &ProjectId) -> Result<ProjectDomain, ProjectGatewayError> {
        let row: Project = sqlx::query_as("SELECT * FROM projects WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| ProjectGatewayError::Internal(e.to_string()))?
            .ok_or(ProjectGatewayError::NotFound)?;

        Ok(map_project_model_to_domain(row))
    }

    async fn list(&self, limit: &u64, offset: &u64) -> Result<Vec<ProjectDomain>, ProjectGatewayError> {
        let rows: Vec<Project> = sqlx::query_as("SELECT * FROM projects LIMIT $1 OFFSET $2")
            .bind(*limit as i64)
            .bind(*offset as i64)
            .fetch_all(&self.db)
            .await
            .map_err(|e| ProjectGatewayError::Internal(e.to_string()))?;

        Ok(rows.into_iter().map(map_project_model_to_domain).collect())
    }
}

#[async_trait]
impl ProjectWriter for ProjectGateway {
    async fn save(&self, project: ProjectDomain) -> Result<(), ProjectGatewayError> {
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
        .execute(&self.db)
        .await
        .map(|_| ())
        .map_err(|e| ProjectGatewayError::Internal(e.to_string()))
    }
}

#[async_trait]
impl ProjectRemover for ProjectGateway {
    async fn remove(&self, id: &ProjectId) -> Result<(), ProjectGatewayError> {
        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| ProjectGatewayError::Internal(e.to_string()))
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
