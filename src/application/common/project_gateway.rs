use async_trait::async_trait;
use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::project::{Project, ProjectId};

pub enum ProjectGatewayError {
    NotFound,
    Internal(String),
}

impl From<ProjectGatewayError> for ApplicationError {
    fn from(e: ProjectGatewayError) -> Self {
        match e {
            ProjectGatewayError::NotFound => ApplicationError::NotFound,
            ProjectGatewayError::Internal(msg) => ApplicationError::Internal(msg),
        }
    }
}

#[async_trait]
pub trait ProjectReader: Send + Sync {
    async fn by_id(&self, id: &ProjectId) -> Result<Project, ProjectGatewayError>;
    async fn list(&self, limit: &u64, offset: &u64) -> Result<Vec<Project>, ProjectGatewayError>;
}

#[async_trait]
pub trait ProjectWriter: Send + Sync {
    async fn save(&self, project: Project) -> Result<(), ProjectGatewayError>;
}

#[async_trait]
pub trait ProjectRemover: Send + Sync {
    async fn remove(&self, id: &ProjectId) -> Result<(), ProjectGatewayError>;
}

pub trait ProjectGateway: ProjectReader + ProjectWriter + ProjectRemover {}
