use crate::application::common::exceptions::ApplicationError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::{NoteReader, NoteWriter};
use crate::domain::models::note::{Note, NoteId};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetBySlugNoteRequest {
    pub slug: String
}

#[derive(Debug, Serialize)]
pub struct GetBySlugNoteResult {
    pub id: NoteId,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct GetBySlugNote<'a> {
    pub note_reader: &'a dyn NoteReader
}

#[async_trait]
impl Interactor<GetBySlugNoteRequest, GetBySlugNoteResult> for GetBySlugNote<'_> {
    async fn execute(
        &self,
        data: GetBySlugNoteRequest
    ) -> Result<GetBySlugNoteResult, ApplicationError> {

        let note = self.note_reader.get_by_slug(&data.slug).await
            .ok_or(ApplicationError::NotFound)?;
        
        Ok(GetBySlugNoteResult {
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
    use std::collections::HashMap;
    use crate::application::common::id_provider::test::MockIdProvider;
    use crate::application::common::note_gateway::test::MockNoteGateway;
    use crate::domain::models::note::{Note, NOTE_BODY_MAX, NOTE_TITLE_MAX};
    use super::*;

    #[tokio::test]
    async fn test_get_by_slug_note() {
        let note = Note::create("Supa title for you".to_string(), "Test".to_string()).unwrap();

        let note_gateway = MockNoteGateway::new(HashMap::from(
            vec![(note.id.clone(), note.clone())]
        ));

        let interactor = GetBySlugNote {
            note_reader: &note_gateway,
        };

        let request = GetBySlugNoteRequest {
            slug: "supa-title-for-you".to_string()
        };

        let result = interactor.execute(request).await.unwrap();

        assert_eq!(result.title, "Supa title for you");
    }
}