use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::NoteReader;
use crate::domain::models::note::Note;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetBySlugNoteRequest {
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct GetBySlugNoteResult {
    pub note: Note,
}

pub struct GetBySlugNote<'a> {
    pub note_reader: &'a dyn NoteReader,
}

#[async_trait]
impl Interactor<GetBySlugNoteRequest, GetBySlugNoteResult> for GetBySlugNote<'_> {
    async fn execute(&self, data: GetBySlugNoteRequest) -> Result<GetBySlugNoteResult, ApplicationError> {
        let note = self.note_reader.get_by_slug(&data.slug).await
            .ok_or(ApplicationError::NotFound)?;
        Ok(GetBySlugNoteResult { note })
    }
}
