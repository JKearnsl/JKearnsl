use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::domain::id_generator::generate_id;
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
    pub fn create(username: String, password_hash: Hash) -> anyhow::Result<Self, HashMap<String, String>> {
        if username.len() > USERNAME_MAX {
            return Err(HashMap::from([(
                "username".to_string(), 
                format!("is too long: {} > {}", username.len(), USERNAME_MAX))
            ]));
        }
        
        Ok(Self {
            id: generate_id(USER_ID_SIZE),
            username,
            password_hash
        })
    }
}
