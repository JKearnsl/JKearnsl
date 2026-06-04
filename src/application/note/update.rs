use crate::application::common::exceptions::ApplicationError;
use crate::application::common::id_provider::IdProvider;
use crate::application::common::interactor::Interactor;
use crate::application::common::note_gateway::{NoteReader, NoteWriter};
use crate::domain::models::note::{Category, NoteId, State, NOTE_BODY_MAX, NOTE_TITLE_MAX};
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Input {
    pub id: NoteId,
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
    pub slug: String,
}

pub struct UpdateNote<'a> {
    pub note_reader: &'a dyn NoteReader,
    pub note_writer: &'a dyn NoteWriter,
    pub id_provider: Box<dyn IdProvider>,
}

#[async_trait]
impl Interactor<Input, Output> for UpdateNote<'_> {
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

        let mut note = self.note_reader.get_by_id_admin(&data.id).await
            .ok_or(ApplicationError::NotFound)?;

        note.title = data.title;
        note.slug = slug::slugify(&note.title);
        note.description = data.description;
        note.body = data.body;
        note.category = data.category;
        note.tags = data.tags;
        note.featured = data.featured;
        note.state = if data.publish { State::Published } else { State::Draft };
        note.updated_at = Some(Utc::now());

        self.note_writer.save(&note).await;

        Ok(Output { slug: note.slug })
    }
}
