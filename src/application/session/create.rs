use async_trait::async_trait;
use crate::application::common::{
    exceptions::ApplicationError,
    hasher::Hasher,
    id_provider::IdProvider,
    interactor::Interactor,
    session_gateway::SessionWriter,
    token_generator::SessionTokenGenerator,
    user_gateway::UserReader,
};
use crate::domain::models::{hash::Hash, session::{Session, Token}};

pub struct Input {
    pub username: String,
    pub password: String,
}

pub struct CreateSession<'a> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_reader: &'a dyn UserReader,
    pub password_hasher: &'a dyn Hasher,
    pub session_hasher: &'a dyn Hasher,
    pub session_token_generator: &'a dyn SessionTokenGenerator,
    pub session_writer: &'a dyn SessionWriter,
}

#[async_trait]
impl Interactor<Input, Token> for CreateSession<'_> {
    async fn execute(&self, data: Input) -> Result<Token, ApplicationError> {
        if self.id_provider.is_auth() {
            return Err(ApplicationError::Forbidden);
        }

        let user = self.user_reader
            .by_username(&data.username)
            .await?
            .ok_or(ApplicationError::Unauthorized)?;

        let password_hash = Hash(user.password_hash.into_bytes());
        if !self.password_hasher.verify(data.password.as_bytes(), &password_hash).await {
            return Err(ApplicationError::Unauthorized);
        }

        let token = self.session_token_generator.generate().await;
        let token_hash = self.session_hasher.hash(&token).await;
        self.session_writer.save(Session::new(token_hash, user.id)).await?;

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::application::common::{
        hasher::test::MockHasher,
        id_provider::test::MockIdProvider,
        session_gateway::SessionGatewayError,
        user_gateway::test::MockUserGateway,
    };
    use crate::domain::models::{session::Session, user::User};

    struct MockSessionWriter;

    #[async_trait]
    impl SessionWriter for MockSessionWriter {
        async fn save(&self, _model: Session) -> Result<(), SessionGatewayError> {
            Ok(())
        }
    }

    struct MockSessionTokenGenerator;

    #[async_trait]
    impl SessionTokenGenerator for MockSessionTokenGenerator {
        async fn generate(&self) -> [u8; 32] {
            [0u8; 32]
        }
    }

    #[tokio::test]
    async fn test_create_session_ok() {
        let hasher = MockHasher;
        let id_provider = Box::new(MockIdProvider { is_auth: false, session: None, user_id: None, username: None });
        let user_reader = MockUserGateway::new(vec![
            User::new("test".to_string(), "password".to_string()),
        ]);

        let result = CreateSession {
            id_provider,
            user_reader: &user_reader,
            password_hasher: &hasher,
            session_hasher: &hasher,
            session_token_generator: &MockSessionTokenGenerator,
            session_writer: &MockSessionWriter,
        }
        .execute(Input { username: "test".to_string(), password: "password".to_string() })
        .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), [0u8; 32]);
    }

    #[tokio::test]
    async fn test_create_session_wrong_password() {
        let hasher = MockHasher;
        let id_provider = Box::new(MockIdProvider { is_auth: false, session: None, user_id: None, username: None });
        let user_reader = MockUserGateway::new(vec![
            User::new("test".to_string(), "password".to_string()),
        ]);

        let result = CreateSession {
            id_provider,
            user_reader: &user_reader,
            password_hasher: &hasher,
            session_hasher: &hasher,
            session_token_generator: &MockSessionTokenGenerator,
            session_writer: &MockSessionWriter,
        }
        .execute(Input { username: "test".to_string(), password: "wrong".to_string() })
        .await;

        assert!(matches!(result, Err(ApplicationError::Unauthorized)));
    }

    #[tokio::test]
    async fn test_create_session_already_auth() {
        let hasher = MockHasher;
        let id_provider = Box::new(MockIdProvider {
            is_auth: true,
            session: Some("token".to_string()),
            user_id: Some("test_id".to_string()),
            username: Some("test".to_string()),
        });
        let user_reader = MockUserGateway::new(vec![]);

        let result = CreateSession {
            id_provider,
            user_reader: &user_reader,
            password_hasher: &hasher,
            session_hasher: &hasher,
            session_token_generator: &MockSessionTokenGenerator,
            session_writer: &MockSessionWriter,
        }
        .execute(Input { username: "test".to_string(), password: "password".to_string() })
        .await;

        assert!(matches!(result, Err(ApplicationError::Forbidden)));
    }
}
