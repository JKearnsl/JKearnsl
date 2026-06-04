use async_trait::async_trait;
use serde::Serialize;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::UserId;

#[derive(Debug, Serialize)]
pub struct UserListItem {
    pub id: UserId,
    pub username: String,
}

pub struct GetUserList<'a> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_reader: &'a dyn UserReader,
}

#[async_trait]
impl Interactor<(), Vec<UserListItem>> for GetUserList<'_> {
    async fn execute(&self, _data: ()) -> Result<Vec<UserListItem>, ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }

        // This might not be the best strategy, but I won't have many users
        // other than me and a couple of bots
        let users = self.user_reader.get_all().await;

        Ok(users.into_iter().map(|u| UserListItem { id: u.id, username: u.username }).collect())
    }
}

#[cfg(test)]
mod test {
    use tokio::sync::Mutex;
    use crate::application::common::hasher::Hasher;
    use crate::application::common::hasher::test::MockHasher;
    use crate::application::common::id_provider::test::MockIdProvider;
    use crate::application::common::user_gateway::test::MockUserGateway;
    use crate::domain::models::user::User;
    use super::*;

    #[tokio::test]
    async fn test_get_user_list() {
        let id_provider = MockIdProvider {
            session: None,
            user_id: Some("test_id".to_string()),
            is_auth: true,
            username: Some("test".to_string()),
        };

        let hash = MockHasher.hash("password".as_bytes()).await;
        let password_hash = String::from_utf8(hash.0).expect("MockHasher returns valid UTF-8");
        let user = User::new("user".to_string(), password_hash);
        let user_gateway = MockUserGateway { users: Mutex::new(vec![user]) };

        let result = GetUserList { id_provider: Box::new(id_provider), user_reader: &user_gateway }
            .execute(()).await.expect("get user list should succeed");

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].username, "user");
    }
}
