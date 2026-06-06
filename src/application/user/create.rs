use std::collections::HashMap;
use async_trait::async_trait;
use crate::application::common::{
    exceptions::ApplicationError,
    hasher::Hasher,
    id_provider::IdProvider,
    interactor::Interactor,
    user_gateway::UserGateway,
};
use crate::domain::models::user::{User, USERNAME_MAX};

pub struct Input {
    pub username: String,
    pub password: String,
}

pub struct CreateUser<'a> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_gateway: &'a dyn UserGateway,
    pub hasher: &'a dyn Hasher,
}

#[async_trait]
impl Interactor<Input, ()> for CreateUser<'_> {
    async fn execute(&self, data: Input) -> Result<(), ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }

        if data.username.len() > USERNAME_MAX {
            return Err(ApplicationError::ValidationError(HashMap::from([(
                "username".to_string(),
                format!("is too long: {} > {}", data.username.len(), USERNAME_MAX),
            )])));
        }

        if self.user_gateway.get_by_username(&data.username).await.is_some() {
            return Err(ApplicationError::ValidationError(HashMap::from([(
                "username".to_string(),
                "Username already exists".to_string(),
            )])));
        }

        let hash = self.hasher.hash(data.password.as_bytes()).await;
        let password_hash = String::from_utf8(hash.0)
            .map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;

        let user = User::new(data.username, password_hash);
        self.user_gateway.save(&user).await;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tokio::sync::Mutex;
    use crate::application::common::{
        hasher::test::MockHasher,
        id_provider::test::MockIdProvider,
        user_gateway::test::MockUserGateway,
    };
    use super::*;

    #[tokio::test]
    async fn test_create_user() {
        let hasher = MockHasher;
        let user_gateway = MockUserGateway { users: Mutex::new(vec![]) };
        let id_provider = Box::new(MockIdProvider {
            session: None,
            user_id: Some("test_user_id".to_string()),
            is_auth: true,
            username: Some("test_user".to_string()),
        });
        let interactor = CreateUser { id_provider, user_gateway: &user_gateway, hasher: &hasher };

        interactor.execute(Input { username: "test".to_string(), password: "password".to_string() })
            .await.expect("create user should succeed");

        let users = user_gateway.users.lock().await;
        assert_eq!(users.len(), 1);
    }
}
