use async_trait::async_trait;
use serde::Serialize;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::user_gateway::UserReader;
use crate::domain::models::user::UserId;

type UserListResult = Vec<UserListItem>;

#[derive(Debug, Serialize)]
pub struct UserListItem{
    pub id: UserId,
    pub username: String
}


pub struct GetUserList<'interactor_life> {
    pub id_provider: Box<dyn IdProvider>,
    pub user_reader: &'interactor_life dyn UserReader
}

#[async_trait]
impl Interactor<(), UserListResult> for GetUserList {
    async fn execute(&self, _data: ()) -> Result<UserListResult, ApplicationError> {

        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }
        
        // This might not be the best strategy, but I won't have many users
        // other than me and a couple of bots
        let users = self.user_reader.get_all().await;
        
        Ok(users.into_iter().map(|u| UserListItem {
            id: u.id,
            username: u.username
        }).collect())
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
            is_auth: true,
            username: Some("test".parse().unwrap())
        };
        
        let user_gateway = MockUserGateway {
            users: Mutex::new(vec![
                User::create(
                    "user".to_string(),
                    MockHasher.hash("password")
                ).unwrap()
            ])
        };

        let interactor = GetUserList {
            id_provider: Box::new(id_provider),
            user_reader: &user_gateway
        };

        let result = interactor.execute(()).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].username, "user");
    }
}
