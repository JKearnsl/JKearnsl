use crate::application::common::exceptions::ApplicationError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::{NoteReader, NoteWriter};
use crate::domain::models::note::{Note, NoteId};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetByIdNoteRequest {
    pub id: NoteId
}

#[derive(Debug, Serialize)]
pub struct GetByIdNoteResult {
    pub id: NoteId,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct GetByIdNote<'a> {
    pub note_reader: &'a dyn NoteReader
}

#[async_trait]
impl Interactor<GetByIdNoteRequest, GetByIdNoteResult> for GetByIdNote<'_> {
    async fn execute(
        &self,
        data: GetByIdNoteRequest
    ) -> Result<GetByIdNoteResult, ApplicationError> {

        let note = self.note_reader.get_by_id(&data.id).await
            .ok_or(ApplicationError::NotFound)?;
        
        Ok(GetByIdNoteResult {
            id: note.id,
            slug: note.slug,
            title: note.title,
            description: note.description,
            body: note.body,
            created_at: note.created_at,
            updated_at: note.updated_at
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::common::note_gateway::test::MockNoteGateway;
    use crate::domain::models::note::Note;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_by_slug_note() {
        let note = Note::create("Supa title for you".to_string(), "Test".to_string()).unwrap();

        let note_gateway = MockNoteGateway::new(HashMap::from(
            vec![(note.id.clone(), note.clone())]
        ));

        let interactor = GetByIdNote {
            note_reader: &note_gateway,
        };

        let request = GetByIdNoteRequest {
            id: note.id
        };

        let result = interactor.execute(request).await.unwrap();

        assert_eq!(result.id, note.id);
    }
}