use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::NoteReader;
use crate::domain::models::note::{Note, NoteId};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub id: NoteId,
}

pub struct GetByIdNoteAdmin<'a> {
    pub note_reader: &'a dyn NoteReader,
}

#[async_trait]
impl Interactor<Input, Note> for GetByIdNoteAdmin<'_> {
    async fn execute(&self, data: Input) -> Result<Note, ApplicationError> {
        self.note_reader.get_by_id_admin(&data.id).await
            .ok_or(ApplicationError::NotFound)
    }
}
