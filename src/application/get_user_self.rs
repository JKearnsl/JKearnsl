use serde::Serialize;

use crate::application::common::exceptions::{ApplicationError, ErrorContent};
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;

#[derive(Debug, Serialize)]
pub struct UserSelfResultDTO{
    username: String
}


pub struct GetUserSelf {
    pub id_provider: Box<dyn IdProvider>,
}

impl Interactor<(), UserSelfResultDTO> for GetUserSelf {
    async fn execute(&self, _data: ()) -> Result<UserSelfResultDTO, ApplicationError> {

        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized(ErrorContent::from("Unauthorized")));
        }
        
        Ok(UserSelfResultDTO {
            username: self.id_provider.username().unwrap().to_string()
        })
    }
}
