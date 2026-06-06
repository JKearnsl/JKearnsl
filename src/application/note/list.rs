use crate::application::common::{
    exceptions::ApplicationError,
    id_provider::IdProvider,
    interactor::Interactor,
    note_gateway::NoteReader,
};
use crate::domain::models::note::{Category, NoteListItem, State};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub category: Option<Category>,
    pub limit: u64,
    pub offset: u64,
}

pub struct ListNotes<'a> {
    pub note_reader: &'a dyn NoteReader,
    pub id_provider: Box<dyn IdProvider>,
}

#[async_trait]
impl Interactor<Input, Vec<NoteListItem>> for ListNotes<'_> {
    async fn execute(&self, data: Input) -> Result<Vec<NoteListItem>, ApplicationError> {
        let state = if !self.id_provider.is_auth() { Some(State::Published) } else { None };
        Ok(self.note_reader.list(&data.limit, &data.offset, state.as_ref(), data.category.as_ref(), None).await?)
    }
}
