use crate::application::common::{
    exceptions::ApplicationError,
    interactor::Interactor,
    note_gateway::NoteReader
};
use crate::domain::models::note::Note;
use async_trait::async_trait;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Input {
    pub slug: String,
}

pub struct GetBySlugNote<'a> {
    pub note_reader: &'a dyn NoteReader,
}

#[async_trait]
impl Interactor<Input, Note> for GetBySlugNote<'_> {
    async fn execute(&self, data: Input) -> Result<Note, ApplicationError> {
        self.note_reader.by_slug(&data.slug).await.map_err(ApplicationError::from)
    }
}
