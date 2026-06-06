use async_trait::async_trait;
use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::user::{User, UserId};

pub enum UserGatewayError {
    Internal(String),
}

impl From<UserGatewayError> for ApplicationError {
    fn from(e: UserGatewayError) -> Self {
        match e {
            UserGatewayError::Internal(msg) => ApplicationError::Internal(msg),
        }
    }
}

#[async_trait]
pub trait UserReader: Send + Sync {
    async fn by_id(&self, user_id: &UserId) -> Result<Option<User>, UserGatewayError>;
    async fn by_username(&self, username: &str) -> Result<Option<User>, UserGatewayError>;
    async fn list(&self) -> Result<Vec<User>, UserGatewayError>;
}

#[async_trait]
pub trait UserWriter: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), UserGatewayError>;
}

#[async_trait]
pub trait UserRemover: Send + Sync {
    async fn remove(&self, user_id: &UserId) -> Result<(), UserGatewayError>;
}


pub trait UserGateway: UserReader + UserWriter + UserRemover {}


#[cfg(test)]
pub mod test {
    use async_trait::async_trait;
    use tokio::sync::Mutex;
    use super::*;

    pub struct MockUserGateway {
        pub users: Mutex<Vec<User>>,
    }

    impl MockUserGateway {
        pub fn new(users: Vec<User>) -> Self {
            Self {
                users: Mutex::new(users),
            }
        }
    }

    #[async_trait]
    impl UserReader for MockUserGateway {
        async fn by_id(&self, user_id: &UserId) -> Result<Option<User>, UserGatewayError> {
            Ok(self.users.lock().await.iter().find(|u| u.id == *user_id).cloned())
        }

        async fn by_username(&self, username: &str) -> Result<Option<User>, UserGatewayError> {
            Ok(self.users.lock().await.iter().find(|u| u.username == *username).cloned())
        }

        async fn list(&self) -> Result<Vec<User>, UserGatewayError> {
            Ok(self.users.lock().await.clone())
        }
    }

    #[async_trait]
    impl UserWriter for MockUserGateway {
        async fn save(&self, user: &User) -> Result<(), UserGatewayError> {
            self.users.lock().await.push(user.clone());
            Ok(())
        }
    }

    #[async_trait]
    impl UserRemover for MockUserGateway {
        async fn remove(&self, user_id: &UserId) -> Result<(), UserGatewayError> {
            self.users.lock().await.retain(|u| u.id != *user_id);
            Ok(())
        }
    }

    impl UserGateway for MockUserGateway {}
}
