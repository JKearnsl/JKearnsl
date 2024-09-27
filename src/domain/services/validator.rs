use crate::domain::models::project::{
    PROJECT_TITLE_LENGTH, 
    PROJECT_DESCRIPTION_LENGTH, 
    PROJECT_URL_LENGTH_MAX
};
use crate::domain::models::note::{
    NOTE_TITLE_LENGTH,
    NOTE_DESCRIPTION_LENGTH,
    NOTE_BODY_LENGTH
};


pub struct ValidatorService {
}

impl ValidatorService {

    pub fn new() -> Self {
        Self {}
    }


    pub fn validate_project_title(&self, title: &str) -> Result<(), String> {
        
        if title.len() < PROJECT_TITLE_LENGTH.0 || title.len() > PROJECT_TITLE_LENGTH.1 {
            return Err(
                format!(
                    "Title should be between {} and {} characters",
                    PROJECT_TITLE_LENGTH.0,
                    PROJECT_TITLE_LENGTH.1
                )
            );
        }
        Ok(())
    }

    pub fn validate_project_description(&self, description: &str) -> Result<(), String> {

        if description.len() < PROJECT_DESCRIPTION_LENGTH.0 || description.len() > PROJECT_DESCRIPTION_LENGTH.1 {
            return Err(
                format!(
                    "Description should be between {} and {} characters",
                    PROJECT_DESCRIPTION_LENGTH.0,
                    PROJECT_DESCRIPTION_LENGTH.1
                )
            );
        }
        Ok(())
    }

    pub fn validate_project_url(&self, url: &str) -> Result<(), String> {
        if url.len() > PROJECT_URL_LENGTH_MAX {
            return Err(format!(
                "Url should be less than {} characters",
                PROJECT_URL_LENGTH_MAX
            ));
        }

        Ok(())
    }

    pub fn validate_note_title(&self, title: &str) -> Result<(), String> {

        if title.len() < NOTE_TITLE_LENGTH.0 || title.len() > NOTE_TITLE_LENGTH.1 {
            return Err(
                format!(
                    "Title should be between {} and {} characters",
                    NOTE_TITLE_LENGTH.0,
                    NOTE_TITLE_LENGTH.1
                )
            );
        }
        Ok(())
    }

    pub fn validate_note_description(&self, description: &str) -> Result<(), String> {

        if description.len() < NOTE_DESCRIPTION_LENGTH.0 || description.len() > NOTE_DESCRIPTION_LENGTH.1 {
            return Err(
                format!(
                    "Description should be between {} and {} characters",
                    NOTE_DESCRIPTION_LENGTH.0,
                    NOTE_DESCRIPTION_LENGTH.1
                )
            );
        }
        Ok(())
    }

    pub fn validate_note_body(&self, body: &str) -> Result<(), String> {

        if body.len() < NOTE_BODY_LENGTH.0 || body.len() > NOTE_BODY_LENGTH.1 {
            return Err(
                format!(
                    "Body should be between {} and {} characters",
                    NOTE_BODY_LENGTH.0,
                    NOTE_BODY_LENGTH.1
                )
            );
        }
        Ok(())
    }
    
    pub fn validate_username(&self, username: &str) -> Result<(), String> {
        if username.len() > 1024 {
            return Err("Username should be less than 1024 characters".to_string());
        }
        Ok(())
    }
    
    pub fn validate_password(&self, password: &str) -> Result<(), String> {
        if password.len() > 1024 {
            return Err("Password should be less than 1024 characters".to_string());
        }
        Ok(())
    }


    pub fn validate_page(&self, page: &u64) -> Result<(), String> {
        if *page == 0 {
            return Err("Page number should be greater than 0".to_string());
        }
        Ok(())
    }

    pub fn validate_per_page(&self, per_page: &u64) -> Result<(), String> {
        if *per_page == 0 {
            return Err("Number of elements per page should be greater than 0".to_string());
        } else if *per_page > 100 {
            return Err("Number of elements per page should be less than 100".to_string());
        }
        Ok(())
    }
}