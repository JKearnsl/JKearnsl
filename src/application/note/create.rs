use crate::application::common::exceptions::ApplicationError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::NoteWriter;
use crate::domain::models::note::{Note, NoteId};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub body: String
}

#[derive(Debug, Serialize)]
pub struct CreateNoteResult {
    pub id: NoteId,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct CreateNote<'a> {
    pub note_writer: &'a dyn NoteWriter,
    pub id_provider: Box<dyn IdProvider>
}

#[async_trait]
impl Interactor<CreateNoteRequest, CreateNoteResult> for CreateNote<'_> {
    async fn execute(
        &self,
        data: CreateNoteRequest
    ) -> Result<CreateNoteResult, ApplicationError> {

        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }

        let note = Note::create(data.title, data.body).map_err(|e| {
            ApplicationError::ValidationError(e)
        })?;
        
        self.note_writer.save(&note).await;
        
        Ok(CreateNoteResult {
            id: note.id,
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
    async fn test_create_note() {
        let id_provider = MockIdProvider {
            session: None,
            is_auth: true,
            username: Some("test".parse().unwrap())
        };

        let note_gateway = MockNoteGateway::new(HashMap::default());

        let interactor = CreateNote {
            note_writer: &note_gateway,
            id_provider: Box::new(id_provider)
        };

        let request = CreateNoteRequest {
            title: "Test".to_string(),
            body: "Test".to_string()
        };

        let result = interactor.execute(request).await.unwrap();

        assert_eq!(result.title, "Test");
        assert_eq!(result.body, "Test");
    }

    #[tokio::test]
    async fn test_create_note_too_long_title() {
        let id_provider = MockIdProvider {
            session: None,
            is_auth: true,
            username: Some("test".parse().unwrap())
        };

        let note_gateway = MockNoteGateway::new(
            HashMap::from([(
                "test".to_string(),
                Note::create(
                    "Test".to_string(),
                    "Test".to_string()
                ).unwrap()
            )])
        );

        let interactor = CreateNote {
            note_writer: &note_gateway,
            id_provider: Box::new(id_provider)
        };

        let request = CreateNoteRequest {
            title: "a".repeat(NOTE_TITLE_MAX + 1),
            body: "Test".to_string()
        };

        let result = interactor.execute(request).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_note_too_long_body() {
        let id_provider = MockIdProvider {
            session: None,
            is_auth: true,
            username: Some("test".parse().unwrap())
        };

        let note_gateway = MockNoteGateway::new(
            HashMap::from([(
                "test".to_string(),
                Note::create(
                    "Test".to_string(),
                    "Test".to_string()
                ).unwrap()
            )])
        );

        let interactor = CreateNote {
            note_writer: &note_gateway,
            id_provider: Box::new(id_provider)
        };

        let request = CreateNoteRequest {
            title: "Test".to_string(),
            body: "a".repeat(NOTE_BODY_MAX + 1)
        };
        
        let result = interactor.execute(request).await;
        
        assert!(result.is_err());
    }
}