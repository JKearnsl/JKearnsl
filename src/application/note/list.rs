use crate::application::common::{
    exceptions::ApplicationError,
    id_provider::IdProvider,
    interactor::Interactor,
    note_gateway::NoteReader,
};
use crate::domain::models::note::{Category, NoteListItem};
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
        if self.id_provider.is_auth() {
            return Ok(self.note_reader.range_all(&data.limit, &data.offset).await);
        }

        let notes = match data.category {
            Some(ref cat) => self.note_reader.range_by_category(cat.as_str(), &data.limit, &data.offset).await,
            None => self.note_reader.range(&data.limit, &data.offset).await,
        };
        Ok(notes)
    }
}
