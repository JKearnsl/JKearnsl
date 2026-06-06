use crate::application::common::{
    exceptions::ApplicationError,
    id_provider::IdProvider,
    interactor::Interactor,
    note_gateway::NoteReader,
};
use crate::domain::models::note::{Note, NoteId, State};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub id: NoteId,
}

pub struct GetByIdNote<'a> {
    pub note_reader: &'a dyn NoteReader,
    pub id_provider: Box<dyn IdProvider>,
}

#[async_trait]
impl Interactor<Input, Note> for GetByIdNote<'_> {
    async fn execute(&self, data: Input) -> Result<Note, ApplicationError> {
        let note = self.note_reader.get_by_id(&data.id).await
            .ok_or(ApplicationError::NotFound)?;

        if note.state != State::Published && !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }

        Ok(note)
    }
}
