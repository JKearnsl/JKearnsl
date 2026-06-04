use crate::application::common::exceptions::ApplicationError;
use crate::application::common::hasher::Hasher;
use crate::application::common::user_gateway::{UserReader, UserWriter};
use crate::domain::models::user::User;

pub struct DefaultCredentials {
    pub username: &'static str,
    pub password: &'static str,
}

pub async fn create_default_admin(
    user_reader: &dyn UserReader,
    user_writer: &dyn UserWriter,
    hasher: &dyn Hasher,
) -> Result<Option<DefaultCredentials>, ApplicationError> {
    const USERNAME: &str = "admin";
    const PASSWORD: &str = "admin";

    if user_reader.get_by_username(USERNAME).await.is_some() {
        return Ok(None);
    }

    let hash = hasher.hash(PASSWORD.as_bytes()).await;
    let password_hash = String::from_utf8(hash.0)
        .map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let user = User::new(USERNAME.to_string(), password_hash);
    user_writer.save(&user).await;

    Ok(Some(DefaultCredentials { username: USERNAME, password: PASSWORD }))
}
