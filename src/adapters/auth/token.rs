use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use rand::random;
use crate::application::common::id_provider::IdProvider;

pub struct IdTokenProvider {
    token: Option<String>,
    username: Option<String>,
    is_auth: bool,
}

impl IdTokenProvider {
    pub fn new(token: Option<String>, token_processor: &TokenProcessor) -> Result<Self, String> {
        match token {
            Some(token) => {
                let username = token_processor.get_token_session(&token)?;
                Ok(Self {
                    token: Some(token),
                    username: Some(username),
                    is_auth: true,
                })
            }
            None => Ok(Self {
                token: None,
                username: None,
                is_auth: false,
            }),
        }
    }

    pub fn token(&self) -> Option<&String> {
        self.token.as_ref()
    }
}

impl IdProvider for IdTokenProvider {
    fn session(&self) -> Option<&String> {
        self.token.as_ref()
    }

    fn username(&self) -> Option<&String> {
        self.username.as_ref()
    }

    fn is_auth(&self) -> bool {
        self.is_auth
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

    pub fn set_token_session(&self, username: &str) -> String {
        let token = (0..64).map(|_| format!("{:02x}", random::<u8>())).collect::<String>();
        self.data.write().unwrap().insert(token.clone(), username.to_owned());
        token
    }

    pub fn get_token_session(&self, token: &str) -> Result<String, String> {
        self.data.read().unwrap()
            .get(token)
            .cloned()
            .ok_or_else(|| "token not valid".to_string())
    }

    pub fn remove_token_session(&self, token: &str) {
        self.data.write().unwrap().remove(token);
    }
}
