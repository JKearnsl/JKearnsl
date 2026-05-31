use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::NoteReader;
use crate::domain::models::note::{Note, NoteId};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetByIdNoteAdminRequest {
    pub id: NoteId,
}

#[derive(Debug, Serialize)]
pub struct GetByIdNoteAdminResult {
    pub note: Note,
}

pub struct GetByIdNoteAdmin<'a> {
    pub note_reader: &'a dyn NoteReader,
}

#[async_trait]
impl Interactor<GetByIdNoteAdminRequest, GetByIdNoteAdminResult> for GetByIdNoteAdmin<'_> {
    async fn execute(&self, data: GetByIdNoteAdminRequest) -> Result<GetByIdNoteAdminResult, ApplicationError> {
        let note = self.note_reader.get_by_id_admin(&data.id).await
            .ok_or(ApplicationError::NotFound)?;
        Ok(GetByIdNoteAdminResult { note })
    }
}
