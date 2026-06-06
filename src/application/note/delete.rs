use crate::application::common::{
    exceptions::ApplicationError,
    id_provider::IdProvider,
    interactor::Interactor,
    note_gateway::NoteRemover,
};
use crate::domain::models::note::NoteId;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub id: NoteId,
}

pub struct DeleteNote<'a> {
    pub note_remover: &'a dyn NoteRemover,
    pub id_provider: Box<dyn IdProvider>,
}

#[async_trait]
impl Interactor<Input, ()> for DeleteNote<'_> {
    async fn execute(&self, data: Input) -> Result<(), ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }
        self.note_remover.remove(&data.id).await;
        Ok(())
    }
}
