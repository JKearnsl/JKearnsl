
pub trait IdProvider {
    fn token(&self) -> Option<&String>;
    fn username(&self) -> Option<&String>;
    fn is_auth(&self) -> &bool;
}
