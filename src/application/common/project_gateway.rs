use async_trait::async_trait;
use crate::domain::models::project::{Project, ProjectId};


#[async_trait]
pub trait ProjectReader{
    async fn get_project(&self, id: &ProjectId) -> Option<Project>;
    async fn get_projects_range(&self, limit: &u64, offset: &u64) -> Vec<Project>;
}

#[async_trait]
pub trait ProjectWriter{
    async fn save_project(&self, project: &Project);
}

#[async_trait]
pub trait ProjectRemover {
    async fn remove_project(&self, project_id: &ProjectId);
}

pub trait ProjectGateway: ProjectReader + ProjectWriter + ProjectRemover {}
