use serde::{Deserialize, Serialize};
use crate::domain::models::identifier::generate;
use crate::domain::models::hash::Hash;

pub type UserId = String;

pub const USER_ID_SIZE: usize = 16;
pub const USERNAME_MAX: usize = 128;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password_hash: Hash
}

impl User {
    pub fn new(username: String, password_hash: Hash) -> Self {
        Self {
            id: generate(USER_ID_SIZE),
            username,
            password_hash
        }
    }
}
