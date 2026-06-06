use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::application::common::{
    exceptions::ApplicationError,
    id_provider::IdProvider,
    interactor::Interactor,
    session_gateway::SessionReader,
};
use crate::domain::models::hash::Hash;

#[derive(Debug, Serialize)]
pub struct SessionItem {
    pub token_hash: Hash,
    pub created_at: DateTime<Utc>,
}

pub struct ListSessions<'a> {
    pub id_provider: Box<dyn IdProvider>,
    pub session_reader: &'a dyn SessionReader,
}

#[async_trait]
impl Interactor<(), Vec<SessionItem>> for ListSessions<'_> {
    async fn execute(&self, _data: ()) -> Result<Vec<SessionItem>, ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }

        let user_id = self.id_provider.user_id().ok_or(ApplicationError::Unauthorized)?;
        let sessions = self.session_reader.get_by_user_id(user_id).await;

        Ok(sessions.into_iter().map(|s| SessionItem {
            token_hash: s.token_hash,
            created_at: s.created_at,
        }).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use tokio::sync::Mutex;
    use crate::application::common::id_provider::test::MockIdProvider;
    use crate::domain::models::{
        hash::Hash,
        session::Session,
        user::UserId,
    };

    struct MockSessionReader {
        sessions: Mutex<Vec<Session>>,
    }

    #[async_trait]
    impl SessionReader for MockSessionReader {
        async fn get_by_user_id(&self, user_id: &UserId) -> Vec<Session> {
            self.sessions.lock().await
                .iter()
                .filter(|s| &s.user_id == user_id)
                .cloned()
                .collect()
        }
    }

    #[tokio::test]
    async fn test_list_sessions_ok() {
        let user_id = "user-1".to_string();
        let session = Session::new(Hash(vec![0u8; 32]), user_id.clone());
        let session_reader = MockSessionReader { sessions: Mutex::new(vec![session]) };
        let id_provider = Box::new(MockIdProvider {
            is_auth: true,
            session: None,
            user_id: Some(user_id),
            username: Some("alice".to_string()),
        });

        let result = ListSessions { id_provider, session_reader: &session_reader }
            .execute(())
            .await
            .expect("should return session list");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].token_hash, Hash(vec![0u8; 32]));
    }

    #[tokio::test]
    async fn test_list_sessions_unauthorized() {
        let session_reader = MockSessionReader { sessions: Mutex::new(vec![]) };
        let id_provider = Box::new(MockIdProvider {
            is_auth: false,
            session: None,
            user_id: None,
            username: None,
        });

        let result = ListSessions { id_provider, session_reader: &session_reader }
            .execute(())
            .await;

        assert!(matches!(result, Err(ApplicationError::Unauthorized)));
    }
}
