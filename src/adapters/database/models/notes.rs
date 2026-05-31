use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use async_trait::async_trait;

pub const NOTE_TABLE: &str = "notes";

pub struct Note;

#[async_trait]
impl CreateIFNotExists for Note {
    async fn create_if_not_exists(db_pool: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS notes (
                id TEXT PRIMARY KEY,
                no INTEGER NOT NULL DEFAULT 0,
                slug TEXT UNIQUE NOT NULL,
                category TEXT NOT NULL DEFAULT 'prog',
                title TEXT NOT NULL,
                description TEXT NOT NULL DEFAULT '',
                body TEXT NOT NULL DEFAULT '',
                featured INTEGER NOT NULL DEFAULT 0,
                state TEXT NOT NULL DEFAULT 'Draft',
                created_at TEXT NOT NULL,
                updated_at TEXT
            );"
        )
        .execute(db_pool)
        .await?;
        Ok(())
    }
}
