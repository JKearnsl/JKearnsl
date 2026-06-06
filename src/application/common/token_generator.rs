use async_trait::async_trait;
use crate::domain::models::session::Token;

#[async_trait]
pub trait SessionTokenGenerator: Send + Sync {
    async fn generate(&self) -> Token;
}
