use crate::application::common::{
    exceptions::ApplicationError,
    id_provider::IdProvider,
    interactor::Interactor,
    note_gateway::{NoteReader, NoteWriter},
};
use crate::domain::models::note::{Category, Note, NoteId, NOTE_BODY_MAX, NOTE_TITLE_MAX};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub title: String,
    pub description: String,
    pub body: String,
    pub category: Category,
    pub tags: Vec<String>,
    pub featured: bool,
    pub publish: bool,
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub id: NoteId,
    pub slug: String,
    pub title: String,
}

pub struct CreateNote<'a> {
    pub note_reader: &'a dyn NoteReader,
    pub note_writer: &'a dyn NoteWriter,
    pub id_provider: Box<dyn IdProvider>,
}

#[async_trait]
impl Interactor<Input, Output> for CreateNote<'_> {
    async fn execute(&self, data: Input) -> Result<Output, ApplicationError> {
        if !self.id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }

        if data.title.len() > NOTE_TITLE_MAX {
            return Err(ApplicationError::ValidationError(HashMap::from([(
                "title".to_string(),
                format!("too long: {} > {}", data.title.len(), NOTE_TITLE_MAX),
            )])));
        }

        if data.body.len() > NOTE_BODY_MAX {
            return Err(ApplicationError::ValidationError(HashMap::from([(
                "body".to_string(),
                format!("too long: {} > {}", data.body.len(), NOTE_BODY_MAX),
            )])));
        }

        let no = self.note_reader.next_no().await;

        let mut note = Note::new(
            no,
            data.title,
            data.description,
            data.body,
            data.category,
            data.tags,
            data.featured,
        );

        if data.publish {
            note.publish();
        }

        self.note_writer.save(&note).await;

        Ok(Output { id: note.id, slug: note.slug, title: note.title })
    }
}
