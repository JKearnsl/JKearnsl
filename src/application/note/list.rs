use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::NoteReader;
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
    pub all: bool,
}

#[async_trait]
impl Interactor<Input, Vec<NoteListItem>> for ListNotes<'_> {
    async fn execute(&self, data: Input) -> Result<Vec<NoteListItem>, ApplicationError> {
        let notes = if self.all {
            self.note_reader.range_all(&data.limit, &data.offset).await
        } else {
            match data.category {
                Some(ref cat) => self.note_reader.range_by_category(cat.as_str(), &data.limit, &data.offset).await,
                None => self.note_reader.range(&data.limit, &data.offset).await,
            }
        };
        Ok(notes)
    }
}
