use async_trait::async_trait;
use crate::domain::models::note::{Note, NoteId};


#[async_trait]
pub trait NoteReader{
    async fn get_note(&self, id: &NoteId) -> Option<Note>;
    async fn get_notes_without_body_range(&self, limit: &u64, offset: &u64) -> Vec<Note>;
}

#[async_trait]
pub trait NoteWriter{
    async fn save_note(&self, note: &Note);
}

#[async_trait]
pub trait NoteRemover {
    async fn remove_note(&self, note_id: &NoteId);
}

pub trait NoteGateway: NoteReader + NoteWriter + NoteRemover {}
