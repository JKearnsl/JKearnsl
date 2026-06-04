use async_trait::async_trait;
use serde::Serialize;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;

#[derive(Debug, Serialize)]
pub struct Output {
    pub username: String,
}

pub struct GetUserSelf {
    pub id_provider: Box<dyn IdProvider>,
}

#[async_trait]
impl Interactor<(), Output> for GetUserSelf {
    async fn execute(&self, _data: ()) -> Result<Output, ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }

        Ok(Output {
            username: self.id_provider.username().ok_or(ApplicationError::Unauthorized)?.to_string(),
        })
    }
}
