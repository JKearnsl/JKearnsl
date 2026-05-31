pub mod state;
pub mod category;

pub use category::Category;
pub use state::State;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::domain::models::identifier::generate;

pub type NoteId = String;

pub const NOTE_ID_SIZE: usize = 16;
pub const NOTE_SLUG_MAX: usize = 64;
pub const NOTE_TITLE_MAX: usize = 128;
pub const NOTE_DESCRIPTION_MAX: usize = 256;
pub const NOTE_BODY_MAX: usize = 32768;
pub const NOTE_CATEGORY_MAX: usize = 32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: NoteId,
    pub no: u32,
    pub slug: String,
    pub category: category::Category,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tags: Vec<String>,
    pub featured: bool,
    pub state: state::State,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
impl Note {
    pub fn new(
        no: u32,
        title: String,
        description: String,
        body: String,
        category: category::Category,
        tags: Vec<String>,
        featured: bool,
    ) -> Self {
        let slug = slug::slugify(&title);
        Self {
            id: generate(NOTE_ID_SIZE),
            no,
            slug,
            category,
            title,
            description,
            body,
            tags,
            featured,
            state: state::State::Draft,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn publish(&mut self) {
        self.state = state::State::Published;
        self.updated_at = Some(Utc::now());
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteListItem {
    pub id: NoteId,
    pub no: u32,
    pub slug: String,
    pub category: category::Category,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub featured: bool,
    pub state: state::State,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<Note> for NoteListItem {
    fn from(note: Note) -> Self {
        Self {
            id: note.id,
            no: note.no,
            slug: note.slug,
            category: note.category,
            title: note.title,
            description: note.description,
            tags: note.tags,
            featured: note.featured,
            state: note.state,
            created_at: note.created_at,
            updated_at: note.updated_at,
        }
    }
}
