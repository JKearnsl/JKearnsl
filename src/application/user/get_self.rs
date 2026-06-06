use async_trait::async_trait;
use crate::application::common::{
    exceptions::ApplicationError,
    id_provider::IdProvider,
    interactor::Interactor,
};
use crate::domain::models::user::UserSummary;

pub struct GetUserSelf {
    pub id_provider: Box<dyn IdProvider>,
}

#[async_trait]
impl Interactor<(), UserSummary> for GetUserSelf {
    async fn execute(&self, _data: ()) -> Result<UserSummary, ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }

        Ok(UserSummary {
            id: self.id_provider.user_id().ok_or(ApplicationError::Unauthorized)?.to_string(),
            username: self.id_provider.username().ok_or(ApplicationError::Unauthorized)?.to_string(),
        })
    }
}
