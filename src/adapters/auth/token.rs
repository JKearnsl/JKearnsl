use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rand::random;
use crate::application::common::id_provider::IdProvider;

pub struct IdTokenProvider {
    token: Option<String>,
    username: Option<String>,
    is_auth: bool
}


impl IdTokenProvider {
    pub fn new(
        token: Option<String>,
        token_processor: &TokenProcessor,
    ) -> Result<Self, String> {
        match token {
            Some(token) => {
                let username = token_processor.get_token_session(&token)?;
                Ok(Self {
                    token: Some(token),
                    username: Some(username),
                    is_auth: true
                })
            }
            None => {
                Ok(Self {
                    token,
                    username: None,
                    is_auth: false
                })
            }
        }
    }
}

impl IdProvider for IdTokenProvider {
    fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
    fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }
    fn is_auth(&self) -> &bool {
        &self.is_auth
    }
}


pub struct TokenProcessor {
    data: Arc<RwLock<HashMap<String, String>>>,
}

impl TokenProcessor {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn set_token_session(&self, username: &String) -> String {
        let token = (0..128 / 2).map(
            |_| format!("{:02x}", random::<u8>())
        ).collect::<Vec<_>>().join("").to_string();
        
        let mut data = self.data.write().unwrap();
        data.insert(token.clone(), username.clone());
        token
    }

    pub fn get_token_session(&self, token: &str) -> Result<String, String> {
        let data = self.data.read().unwrap();
        match data.get(token) {
            Some(username) => Ok(username.clone()),
            None => Err("Token not valid".to_string())
        }
    }
}
