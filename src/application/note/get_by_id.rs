use crate::application::common::{
    exceptions::ApplicationError,
    interactor::Interactor,
    note_gateway::NoteReader,
};
use crate::domain::models::note::{NoteId, NoteListItem};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub id: NoteId,
}

pub struct GetByIdNote<'a> {
    pub note_reader: &'a dyn NoteReader,
}

#[async_trait]
impl Interactor<Input, NoteListItem> for GetByIdNote<'_> {
    async fn execute(&self, data: Input) -> Result<NoteListItem, ApplicationError> {
        self.note_reader.get_by_id(&data.id).await
            .map(NoteListItem::from)
            .ok_or(ApplicationError::NotFound)
    }
}
