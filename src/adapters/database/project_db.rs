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
use crate::adapters::database::models::projects::{Project, PROJECT_TABLE};


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
            format!("SELECT * FROM {} WHERE id = $1", PROJECT_TABLE).as_str()
        )
            .bind(project_id)
            .fetch_optional(&self.db).await.unwrap();

        match row {
            None => None,
            Some(row) => Some(map_project_model_to_domain(row))
        }
    }

    async fn get_projects_range(&self, limit: &u64, offset: &u64) -> Vec<ProjectDomain> {
        let rows: Vec<Project> = sqlx::query_as(format!(
            "SELECT * FROM {} LIMIT $1 OFFSET $2", 
            PROJECT_TABLE
        ).as_str())
            .bind(limit.clone() as i64)
            .bind(offset.clone() as i64)
            .fetch_all(&self.db).await.unwrap();

        rows.into_iter().map(|row| map_project_model_to_domain(row)).collect()
    }
}

#[async_trait]
impl ProjectWriter for ProjectGateway {
    async fn save_project(&self, project: &ProjectDomain) {
        sqlx::query(format!(
            "INSERT INTO {} (id, title, description, url, created_at) \
             VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET \
             title = $2, description = $3, url = $4, created_at = $5",
            PROJECT_TABLE
        ).as_str())
            .bind(&project.id)
            .bind(&project.title)
            .bind(&project.description)
            .bind(&project.url)
            .bind(&project.created_at)
            .execute(&self.db).await.unwrap();
    }
}

#[async_trait]
impl ProjectRemover for ProjectGateway {
    async fn remove_project(&self, project_id: &ProjectId) {
        sqlx::query(format!("DELETE FROM {} WHERE id = $1", PROJECT_TABLE).as_str())
            .bind(project_id)
            .execute(&self.db).await.unwrap();
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
