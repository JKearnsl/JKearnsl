use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type ProjectId = String;

pub const PROJECT_ID_SIZE: usize = 16;
pub const PROJECT_TITLE_LENGTH: (usize, usize) = (1, 128);
pub const PROJECT_DESCRIPTION_LENGTH: (usize, usize) = (1, 256);
pub const PROJECT_URL_LENGTH_MAX: usize = 2048;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub title: String,
    pub description: String,
    pub url: Option<String>,
    pub created_at: DateTime<Utc>
}
