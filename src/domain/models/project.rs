use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::identifier::generate;

pub type ProjectId = String;

pub const PROJECT_ID_SIZE: usize = 16;
pub const PROJECT_TITLE_MAX: usize = 128;
pub const PROJECT_DESCRIPTION_MAX: usize = 256;
pub const PROJECT_URL_MAX: usize = 2048;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub created_at: DateTime<Utc>
}

impl Project {
    pub fn create(title: String, description: String, url: Option<String>) -> Self {
        Self {
            id: generate(PROJECT_ID_SIZE),
            title,
            description,
            url,
            created_at: Utc::now()
        }
    }

    pub fn update(&mut self, title: String, description: String, url: Option<String>) {
        self.title = title;
        self.description = description;
        self.url = url;
    }
}