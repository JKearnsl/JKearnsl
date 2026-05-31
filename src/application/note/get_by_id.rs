use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::NoteReader;
use crate::domain::models::note::{NoteId, NoteListItem};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetByIdNoteRequest {
    pub id: NoteId,
}

#[derive(Debug, Serialize)]
pub struct GetByIdNoteResult {
    pub note: NoteListItem,
}

pub struct GetByIdNote<'a> {
    pub note_reader: &'a dyn NoteReader,
}

#[async_trait]
impl Interactor<GetByIdNoteRequest, GetByIdNoteResult> for GetByIdNote<'_> {
    async fn execute(&self, data: GetByIdNoteRequest) -> Result<GetByIdNoteResult, ApplicationError> {
        let note = self.note_reader.get_by_id(&data.id).await
            .ok_or(ApplicationError::NotFound)?;
        Ok(GetByIdNoteResult { note: NoteListItem::from(note) })
    }
}
