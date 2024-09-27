use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type NoteId = String;

pub const NOTE_ID_SIZE: usize = 16;
pub const NOTE_TITLE_LENGTH: (usize, usize) = (1, 128);
pub const NOTE_DESCRIPTION_LENGTH: (usize, usize) = (1, 256);
pub const NOTE_BODY_LENGTH: (usize, usize) = (1, 32768);


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}
