use crate::domain::models::user::{User, UserId};



#[allow(async_fn_in_trait)]
pub trait UserReader{
    async fn get_by_username(&self, username: &str) -> Option<User>;
    async fn get_all(&self) -> Vec<User>;
}


#[allow(async_fn_in_trait)]
pub trait UserWriter{
    async fn save(&self, user: &User);
}


#[allow(async_fn_in_trait)]
pub trait UserRemover {
    async fn remove(&self, user_id: &UserId);
}


pub trait UserGateway: UserReader + UserWriter + UserRemover {}


#[cfg(test)]
pub mod test {
    use tokio::sync::Mutex;
    use super::*;

    pub struct MockUserGateway {
        pub users: Mutex::<Vec<User>>
    }

    impl MockUserGateway {
        pub fn new(users: Vec<User>) -> Self {
            Self {
                users: Mutex::new(users)
            }
        }
    }

    impl UserReader for MockUserGateway {
        async fn get_by_username(&self, username: &str) -> Option<User> {
            self.users.lock().await.iter().find(|u| u.username == *username).map(|u| u.clone())
        }

        async fn get_all(&self) -> Vec<User> {
            self.users.lock().await.clone()
        }
    }

    impl UserWriter for MockUserGateway {
        async fn save(&self, user: &User) {
            self.users.lock().await.push(user.clone());
        }
    }

    impl UserRemover for MockUserGateway {
        async fn remove(&self, user_id: &UserId) {
            self.users.lock().await.retain(|u| u.id != *user_id);
        }
    }

    impl UserGateway for MockUserGateway {}
}