use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::CredentialsProvider;
use crate::domain::services::validator::ValidatorService;

#[derive(Debug, Deserialize)]
pub struct CreateSessionDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct CreateSessionResultDTO{
    pub username: String,
}

pub struct CreateSession<'a> {
    pub password_hasher: &'a dyn Hasher,
    pub validator: &'a ValidatorService,
    pub id_provider: Box<dyn IdProvider>,
    pub credential_provider: &'a CredentialsProvider,
}

impl Interactor<CreateSessionDTO, CreateSessionResultDTO> for CreateSession<'_> {
    async fn execute(
        &self,
        data: CreateSessionDTO
    ) -> Result<CreateSessionResultDTO, ApplicationError> {

        if *self.id_provider.is_auth() {
            return Err(
                ApplicationError::Forbidden(
                    ErrorContent::from("Already authenticated")
                )
            )
        }

        let mut validator_err_map: HashMap<String, String> = HashMap::new();
        self.validator.validate_username(&data.username).unwrap_or_else(|e| {
            validator_err_map.insert("username".to_string(), e.to_string());
        });

        self.validator.validate_password(&data.password).unwrap_or_else(|e| {
            validator_err_map.insert("password".to_string(), e.to_string());
        });


        if !validator_err_map.is_empty() {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::from(validator_err_map)
                )
            )
        }

        if self.credential_provider.username != data.username {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::from("Invalid username and password pair")
                )
            )
        }
        
        let hashed_password = self.password_hasher.hash(&data.password).await;
        if self.credential_provider.hashed_password != hashed_password {
            return Err(
                ApplicationError::InvalidData(
                    ErrorContent::from("Invalid username and password pair")
                )
            )
        }
        
        Ok(CreateSessionResultDTO {
            username: data.username
        })
    }
}
