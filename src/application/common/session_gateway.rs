use async_trait::async_trait;
use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::hash::Hash;
use crate::domain::models::session::Session;
use crate::domain::models::user::UserId;

pub enum SessionGatewayError {
    Internal(String),
}

impl From<SessionGatewayError> for ApplicationError {
    fn from(e: SessionGatewayError) -> Self {
        match e {
            SessionGatewayError::Internal(msg) => ApplicationError::Internal(msg),
        }
    }
}

#[async_trait]
pub trait SessionWriter: Send + Sync {
    async fn save(&self, model: Session) -> Result<(), SessionGatewayError>;
}

#[async_trait]
pub trait SessionReader: Send + Sync {
    async fn by_user_id(&self, user_id: &UserId) -> Result<Vec<Session>, SessionGatewayError>;
    async fn by_token_hash(&self, token_hash: &Hash) -> Result<Option<Session>, SessionGatewayError>;
}

#[async_trait]
pub trait SessionRemover: Send + Sync {
    async fn remove_by_token_hash(&self, token_hash: &Hash) -> Result<(), SessionGatewayError>;
    async fn remove_older_than(&self, max_age_secs: i64) -> Result<u64, SessionGatewayError>;
}

pub trait SessionGateway: SessionWriter + SessionRemover + SessionReader {}
