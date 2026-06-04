use serde::{Deserialize, Serialize};
use crate::domain::models::identifier::generate;

pub type UserId = String;

pub const USER_ID_SIZE: usize = 16;
pub const USERNAME_MAX: usize = 128;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        Self {
            id: generate(USER_ID_SIZE),
            username,
            password_hash,
        }
    }
}
