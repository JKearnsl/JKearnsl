use async_trait::async_trait;
use crate::domain::models::session::Session;
use crate::domain::models::user::UserId;

#[async_trait]
pub trait SessionWriter: Send + Sync {
    async fn save(&self, raw_token: &[u8; 32], user_id: &UserId);
}

#[async_trait]
pub trait SessionRemover: Send + Sync {
    async fn remove_by_token(&self, raw_token: &[u8; 32]);
}

#[async_trait]
pub trait SessionReader: Send + Sync {
    async fn get_by_user_id(&self, user_id: &UserId) -> Vec<Session>;
}

#[async_trait]
pub trait SessionVacuum: Send + Sync {
    async fn remove_older_than(&self, max_age_secs: i64) -> u64;
}

pub trait SessionGateway: SessionWriter + SessionRemover + SessionReader + SessionVacuum {}
