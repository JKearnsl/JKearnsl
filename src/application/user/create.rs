use std::collections::HashMap;
use async_trait::async_trait;
use serde::Serialize;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::{UserGateway, UserReader, UserWriter};
use crate::domain::models::hash::Hash;
use crate::domain::models::user::User;

#[derive(Debug, Serialize)]
pub struct CreateUserRequest{
    pub username: String,
    pub password_hash: Hash
}


pub struct CreateUser<'interactor_life> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_gateway: &'interactor_life dyn UserGateway,
}

#[async_trait]
impl Interactor<CreateUserRequest, ()> for CreateUser<'_> {
    async fn execute(&self, data: CreateUserRequest) -> Result<(), ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }
        
        let user = User::create(data.username, data.password_hash).map_err(|e| {
            ApplicationError::ValidationError(e)
        })?;
        
        if self.user_gateway.get_by_username(&user.username).await.is_some() {
            return Err(ApplicationError::ValidationError(HashMap::from([(
                "username".to_string(), 
                "Username already exists".to_string()
            )])));
        }
        
        self.user_gateway.save(&user).await;
        
        Ok(())
    }
}



#[cfg(test)]
mod test {
    use tokio::sync::Mutex;
    use crate::application::common::hasher::Hasher;
    use crate::application::common::hasher::test::MockHasher;
    use crate::application::common::id_provider::test::MockIdProvider;
    use crate::application::common::user_gateway::test::MockUserGateway;
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        let hasher = MockHasher;
        let user_gateway = MockUserGateway {
            users: Mutex::new(vec![])
        };
        let id_provider = Box::new(MockIdProvider {
            session: None,
            is_auth: true,
            username: Some("test_user".to_string())
        });
        let interactor = CreateUser {
            id_provider,
            user_gateway: &user_gateway
        };

        interactor.execute(CreateUserRequest {
            username: "test".to_string(),
            password_hash: hasher.hash("password").await.unwrap()
        }).await.unwrap();

        let users = user_gateway.users.lock().await;
        assert_eq!(users.len(), 1);
    }
}