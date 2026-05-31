use crate::application::common::exceptions::ApplicationError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::NoteRemover;
use crate::domain::models::note::NoteId;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DeleteNoteRequest {
    pub id: NoteId,
}

pub struct DeleteNote<'a> {
    pub note_remover: &'a dyn NoteRemover,
    pub id_provider: Box<dyn IdProvider>,
}

#[async_trait]
impl Interactor<DeleteNoteRequest, ()> for DeleteNote<'_> {
    async fn execute(&self, data: DeleteNoteRequest) -> Result<(), ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }
        self.note_remover.remove(&data.id).await;
        Ok(())
    }
}
