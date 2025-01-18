
pub trait IdProvider: Send + Sync {
    fn session(&self) -> Option<&String>;
    fn username(&self) -> Option<&String>;
    fn is_auth(&self) -> &bool;
}


#[cfg(test)]
pub mod test {
    use super::*;

    pub struct MockIdProvider {
        pub session: Option<String>,
        pub username: Option<String>,
        pub is_auth: bool,
    }

    impl IdProvider for MockIdProvider {
        fn session(&self) -> Option<&String> {
            self.session.as_ref()
        }

        fn username(&self) -> Option<&String> {
            self.username.as_ref()
        }

        fn is_auth(&self) -> &bool {
            &self.is_auth
        }
    }
}