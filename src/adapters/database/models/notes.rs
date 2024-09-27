use chrono::{DateTime, Utc};
use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;
use crate::domain::models::note::{
    NOTE_ID_SIZE, 
    NOTE_TITLE_LENGTH, 
    NOTE_DESCRIPTION_LENGTH, 
    NOTE_BODY_LENGTH, 
    NoteId
};

pub const NOTE_TABLE: &str = "notes";

#[derive(sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

impl CreateIFNotExists for Note {
    async fn create_if_not_exists(db_pool: &DbPool) -> Result<(), sqlx::Error> {
        sqlx::query(format!(
            "CREATE TABLE IF NOT EXISTS {table} (
                id CHAR({id_size}) PRIMARY KEY,
                title VARCHAR({title_max}) NOT NULL,
                description VARCHAR({description_max}) NOT NULL,
                body VARCHAR({body_max}) NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITH TIME ZONE
            );",
            table = NOTE_TABLE,
            id_size = NOTE_ID_SIZE,
            title_max = NOTE_TITLE_LENGTH.1,
            description_max = NOTE_DESCRIPTION_LENGTH.1,
            body_max = NOTE_BODY_LENGTH.1
        ).as_str())
            .execute(db_pool)
            .await?;
        Ok(())
    }
}