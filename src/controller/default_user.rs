use crate::adapters::argon2_password_hasher::Argon2PasswordHasher;
use crate::adapters::database::pool::DbPool;
use crate::adapters::database::user::SqliteUserGateway;
use crate::application::user::create_default::create_default_admin;

pub async fn run(db: &DbPool) {
    let gateway = SqliteUserGateway::new(db.clone());
    let hasher = Argon2PasswordHasher::new();

    match create_default_admin(&gateway, &gateway, &hasher).await {
        Ok(Some(creds)) => log::info!(
            "Default user created — login: {}, password: {}",
            creds.username,
            creds.password,
        ),
        Ok(None) => {}
        Err(e) => log::error!("Failed to create default user: {}", e),
    }
}
