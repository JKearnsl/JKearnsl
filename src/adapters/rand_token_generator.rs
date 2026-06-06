use async_trait::async_trait;
use crate::application::common::token_generator::SessionTokenGenerator;
use crate::domain::models::session::Token;

pub struct RandSessionTokenGenerator;

#[async_trait]
impl SessionTokenGenerator for RandSessionTokenGenerator {
    async fn generate(&self) -> Token {
        rand::random()
    }
}
