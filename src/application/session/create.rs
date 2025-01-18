use crate::application::common::exceptions::ApplicationError;
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::hash::Hash;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub username: String,
    pub password_hash: Hash,
}

pub struct CreateSession<'a> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_reader: &'a dyn UserReader,
}

#[async_trait]
impl Interactor<CreateSessionRequest, ()> for CreateSession<'_> {
    async fn execute(
        &self,
        data: CreateSessionRequest
    ) -> Result<(), ApplicationError> {

        if *self.id_provider.is_auth() {
            return Err(ApplicationError::Forbidden)
        }

        let user = self.user_reader.get_by_username(&data.username).await;
        
        if user.is_none() {
            return Err(ApplicationError::Unauthorized)
        }

        if user.unwrap().password_hash != data.password_hash {
            return Err(ApplicationError::Unauthorized)
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::common::hasher::test::MockHasher;
    use crate::application::common::id_provider::test::MockIdProvider;
    use crate::application::common::user_gateway::test::MockUserGateway;
    use crate::domain::models::user::User;

    #[tokio::test]
    async fn test_create_session() {
        let id_provider = Box::new(MockIdProvider {
            is_auth: false,
            session: None,
            username: None
        });

        let user_reader = MockUserGateway::new(vec![
            User::create(
                "test".to_string(),
                MockHasher.hash("password")
            ).unwrap()
        ]);

        let create_session = CreateSession {
            id_provider,
            user_reader: &user_reader
        };

        let create_session_dto = CreateSessionRequest {
            username: "jkearnsl".to_string(),
            password_hash: MockHasher.hash("password")
        };

        let result = create_session.execute(create_session_dto).await;
        assert!(result.is_ok());
        assert_eq!(user_reader.users.lock().await.len(), 2);
    }
}
