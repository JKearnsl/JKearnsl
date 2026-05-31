use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::NoteReader;
use crate::domain::models::note::NoteListItem;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ListNotesRequest {
    pub category: Option<String>,
    pub limit: u64,
    pub offset: u64,
}

#[derive(Debug, Serialize)]
pub struct ListNotesResult {
    pub notes: Vec<NoteListItem>,
}

pub struct ListNotes<'a> {
    pub note_reader: &'a dyn NoteReader,
    pub all: bool,
}

#[async_trait]
impl Interactor<ListNotesRequest, ListNotesResult> for ListNotes<'_> {
    async fn execute(&self, data: ListNotesRequest) -> Result<ListNotesResult, ApplicationError> {
        let notes = if self.all {
            self.note_reader.range_all(&data.limit, &data.offset).await
        } else {
            match data.category {
                Some(ref cat) if cat != "all" => {
                    self.note_reader.range_by_category(cat, &data.limit, &data.offset).await
                }
                _ => self.note_reader.range(&data.limit, &data.offset).await,
            }
        };
        Ok(ListNotesResult { notes })
    }
}
