use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::domain::id_generator::generate_id;

pub type NoteId = String;

pub const NOTE_ID_SIZE: usize = 16;
pub const NOTE_TITLE_MAX: usize = 128;
pub const NOTE_DESCRIPTION_MAX: usize = 256;
pub const NOTE_BODY_MAX: usize = 32768;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Note {
    pub id: NoteId,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

impl Note {
    pub fn create(title: String, body: String) -> anyhow::Result<Self, HashMap<String, String>> {
        if title.len() > NOTE_TITLE_MAX {
            return Err(HashMap::from([(
                "title".to_string(), 
                format!("is too long: {} > {}", title.len(), NOTE_TITLE_MAX)
            )]));
        }
        
        if body.len() > NOTE_BODY_MAX {
            return Err(HashMap::from([(
                "body".to_string(), 
                format!("is too long: {} > {}", body.len(), NOTE_BODY_MAX)
            )]));
        }
        
        Ok(Self {
            id: generate_id(NOTE_ID_SIZE),
            title,
            body,
            created_at: Utc::now(),
            updated_at: None
        })
    }

    pub fn update(&mut self, title: String, body: String) {
        self.title = title;
        self.body = body;
        self.updated_at = Some(Utc::now());
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NoteWithDescription {
    pub id: NoteId,
    pub title: String,
    pub description: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}
