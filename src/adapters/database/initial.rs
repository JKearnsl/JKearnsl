use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use crate::adapters::database::models::notes::Note;
use crate::adapters::database::models::users::User;
use crate::adapters::database::models::tags::Tag;
use crate::adapters::database::models::note_tags::NoteTag;
use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;

pub async fn initial_models(db: &DbPool) -> Result<(), sqlx::Error> {
    Note::create_if_not_exists(db).await?;
    Tag::create_if_not_exists(db).await?;
    NoteTag::create_if_not_exists(db).await?;
    User::create_if_not_exists(db).await?;
    seed_default_admin(db).await?;
    Ok(())
}

async fn seed_default_admin(db: &DbPool) -> Result<(), sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await?;

    if count.0 > 0 {
        return Ok(());
    }

    let password_phc = tokio::task::spawn_blocking(|| {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(b"admin", &salt)
            .expect("failed to hash default admin password")
            .to_string()
    })
    .await
    .expect("spawn_blocking failed");

    sqlx::query(
        "INSERT INTO users (id, username, password_phc) VALUES (?, ?, ?)"
    )
    .bind(nanoid::nanoid!(16))
    .bind("admin")
    .bind(password_phc)
    .execute(db)
    .await?;

    log::info!("Created default admin user (admin/admin) — change the password!");
    Ok(())
}
