use crate::domain::models::user::UserId;

pub trait IdProvider: Send + Sync {
    fn session(&self) -> Option<&String>;
    fn user_id(&self) -> Option<&UserId>;
    fn username(&self) -> Option<&String>;
    fn is_auth(&self) -> bool;
}


#[cfg(test)]
pub mod test {
    use super::*;

    pub struct MockIdProvider {
        pub session: Option<String>,
        pub user_id: Option<UserId>,
        pub username: Option<String>,
        pub is_auth: bool,
    }

    impl IdProvider for MockIdProvider {
        fn session(&self) -> Option<&String> { self.session.as_ref() }
        fn user_id(&self) -> Option<&UserId> { self.user_id.as_ref() }
        fn username(&self) -> Option<&String> { self.username.as_ref() }
        fn is_auth(&self) -> bool { self.is_auth }
    }
}
