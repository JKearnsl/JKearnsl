use async_trait::async_trait;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::hasher::Hasher;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionWriter;
use crate::application::common::user_gateway::UserReader;

pub struct Input {
    pub username: String,
    pub password: String,
}

pub struct CreateSession<'a> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_reader: &'a dyn UserReader,
    pub hasher: &'a dyn Hasher,
    pub session_writer: &'a dyn SessionWriter,
}

#[async_trait]
impl Interactor<Input, [u8; 32]> for CreateSession<'_> {
    async fn execute(&self, data: Input) -> Result<[u8; 32], ApplicationError> {
        if self.id_provider.is_auth() {
            return Err(ApplicationError::Forbidden);
        }

        let user = self.user_reader
            .get_by_username(&data.username)
            .await
            .ok_or(ApplicationError::Unauthorized)?;

        if !self.hasher.verify(data.password.as_bytes(), user.password_hash.as_bytes()).await {
            return Err(ApplicationError::Unauthorized);
        }

        let token: [u8; 32] = rand::random();
        self.session_writer.save(&token, &user.id).await;

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::common::hasher::test::MockHasher;
    use crate::application::common::id_provider::test::MockIdProvider;
    use crate::application::common::user_gateway::test::MockUserGateway;
    use crate::domain::models::user::User;
    use async_trait::async_trait;
    use crate::domain::models::user::UserId;

    struct MockSessionWriter;

    #[async_trait]
    impl SessionWriter for MockSessionWriter {
        async fn save(&self, _raw_token: &[u8; 32], _user_id: &UserId) {}
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
            hasher: &hasher,
            session_writer: &MockSessionWriter,
        }
        .execute(Input { username: "test".to_string(), password: "password".to_string() })
        .await;

        assert!(result.is_ok());
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
            hasher: &hasher,
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
            hasher: &hasher,
            session_writer: &MockSessionWriter,
        }
        .execute(Input { username: "test".to_string(), password: "password".to_string() })
        .await;

        assert!(matches!(result, Err(ApplicationError::Forbidden)));
    }
}
