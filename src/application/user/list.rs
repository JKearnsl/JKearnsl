use async_trait::async_trait;
use crate::application::common::{
    exceptions::ApplicationError,
    id_provider::IdProvider,
    interactor::Interactor,
    user_gateway::UserReader,
};
use crate::domain::models::user::UserSummary;

pub struct GetUserList<'a> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_reader: &'a dyn UserReader,
}

#[async_trait]
impl Interactor<(), Vec<UserSummary>> for GetUserList<'_> {
    async fn execute(&self, _data: ()) -> Result<Vec<UserSummary>, ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }

        // This might not be the best strategy, but I won't have many users
        // other than me and a couple of bots
        let users = self.user_reader.get_all().await;

        Ok(users.into_iter().map(|u| UserSummary { id: u.id, username: u.username }).collect())
    }
}

#[cfg(test)]
mod test {
    use tokio::sync::Mutex;
    use crate::application::common::{
        hasher::{Hasher, test::MockHasher},
        id_provider::test::MockIdProvider,
        user_gateway::test::MockUserGateway,
    };
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
