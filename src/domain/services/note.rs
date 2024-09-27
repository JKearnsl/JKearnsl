use chrono::Utc;
use crate::domain::id_generator::generate_id;
use crate::domain::models::note::{Note, NOTE_ID_SIZE};

pub struct NoteService { }

impl NoteService {

    pub fn create_note(
        &self,
        title: String,
        description: String,
        body: String,
    ) -> Note {
        Note {
            id: generate_id(NOTE_ID_SIZE),
            title,
            description,
            body,
            created_at: Utc::now(),
            updated_at: None,
        }
    }

    pub fn update_note(
        &self,
        note: Note,
        title: String,
        description: String,
        body: String,
    ) -> Note {
        Note {
            title,
            description,
            body,
            updated_at: Some(Utc::now()),
            ..note
        }
    }
}