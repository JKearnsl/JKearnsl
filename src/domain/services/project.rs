use chrono::{DateTime, Utc};
use crate::domain::id_generator::generate_id;
use crate::domain::models::project::{Project, PROJECT_ID_SIZE};


pub struct ProjectService { }

impl ProjectService {

    pub fn create_project(
        &self,
        title: String,
        description: String,
        url: Option<String>,
        created_at: DateTime<Utc>
    ) -> Project {
        Project {
            id: generate_id(PROJECT_ID_SIZE),
            title,
            description,
            url,
            created_at
        }
    }

    pub fn update_project(
        &self,
        project: Project,
        title: String,
        description: String,
        url: Option<String>,
        created_at: DateTime<Utc>
    ) -> Project {
        Project {
            title,
            description,
            url,
            created_at,
            ..project
        }
    }
}