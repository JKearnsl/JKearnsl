use async_trait::async_trait;
use crate::application::common::token_generator::SessionTokenGenerator;

pub struct RandSessionTokenGenerator;

#[async_trait]
impl SessionTokenGenerator for RandSessionTokenGenerator {
    async fn generate(&self) -> [u8; 32] {
        rand::random()
    }
}
