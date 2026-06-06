use crate::domain::models::hash::Hash;
use crate::domain::models::user::UserId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type Token = [u8; 32];

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Session {
    pub token_hash: Hash,
    pub user_id: UserId,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn new(token_hash: Hash, user_id: UserId) -> Self {
        Self { token_hash, user_id, created_at: Utc::now() }
    }
}
