use leptos::prelude::*;
use crate::domain::models::note::{Note, NoteListItem};
use crate::application::common::exceptions::ApplicationError;

#[server]
pub async fn list_by_category(category: Option<String>) -> Result<Vec<NoteListItem>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::list::Input;
    use crate::domain::models::note::Category;

    let ioc: Data<dyn InteractorFactory> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let category = category.and_then(|c| Category::try_from(c.as_str()).ok());
    Ok(ioc.list_notes().execute(Input { category, limit: 200, offset: 0 }).await?)
}

#[server]
pub async fn by_slug(slug: String) -> Result<Option<Note>, ApplicationError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::get_by_slug::Input;
    use crate::controller::web::markdown::render_markdown;

    let ioc: Data<dyn InteractorFactory> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    match ioc.get_note_by_slug().execute(Input { slug }).await {
        Ok(mut note) => {
            note.body = render_markdown(&note.body);
            Ok(Some(note))
        },
        Err(ApplicationError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

#[server]
pub async fn list_as_admin() -> Result<Vec<NoteListItem>, ApplicationError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::list::Input;

    let req: HttpRequest = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let token_processor: Data<TokenProcessor> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let ioc: Data<dyn InteractorFactory> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor).await;

    {
        use crate::application::common::id_provider::IdProvider;
        if !id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }
    }

    Ok(ioc.list_all_notes().execute(Input { category: None, limit: 1000, offset: 0 }).await?)
}

#[server]
pub async fn get_for_edit(id: String) -> Result<Option<Note>, ApplicationError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::get_by_id_admin::Input;

    let req: HttpRequest = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let token_processor: Data<TokenProcessor> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let ioc: Data<dyn InteractorFactory> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor).await;

    {
        use crate::application::common::id_provider::IdProvider;
        if !id_provider.is_auth() {
            return Err(ApplicationError::Unauthorized);
        }
    }

    match ioc.get_note_by_id_admin().execute(Input { id }).await {
        Ok(note) => Ok(Some(note)),
        Err(ApplicationError::NotFound) => Ok(None),
        Err(e) => Err(e),
    }
}

#[server]
pub async fn create(
    title: String,
    description: String,
    body: String,
    category: String,
    tags_raw: String,
    featured: bool,
    publish: bool,
) -> Result<String, ApplicationError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::create::Input;
    use crate::domain::models::note::Category;
    use std::collections::HashMap;

    let req: HttpRequest = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let token_processor: Data<TokenProcessor> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let ioc: Data<dyn InteractorFactory> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor).await;

    let tags: Vec<String> = tags_raw.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let category = Category::try_from(category.as_str())
        .map_err(|e| ApplicationError::ValidationError(HashMap::from([("category".to_string(), e)])))?;

    let result = ioc
        .create_note(Box::new(id_provider))
        .execute(Input { title, description, body, category, tags, featured, publish })
        .await?;

    Ok(result.slug)
}

#[server]
pub async fn update(
    id: String,
    title: String,
    description: String,
    body: String,
    category: String,
    tags_raw: String,
    featured: bool,
    publish: bool,
) -> Result<String, ApplicationError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::update::Input;
    use crate::domain::models::note::Category;
    use std::collections::HashMap;

    let req: HttpRequest = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let token_processor: Data<TokenProcessor> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let ioc: Data<dyn InteractorFactory> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor).await;

    let tags: Vec<String> = tags_raw.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
    let category = Category::try_from(category.as_str())
        .map_err(|e| ApplicationError::ValidationError(HashMap::from([("category".to_string(), e)])))?;

    let result = ioc
        .update_note(Box::new(id_provider))
        .execute(Input { id, title, description, body, category, tags, featured, publish })
        .await?;

    Ok(result.slug)
}

#[server]
pub async fn delete(id: String) -> Result<(), ApplicationError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::delete::Input;

    let req: HttpRequest = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let token_processor: Data<TokenProcessor> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let ioc: Data<dyn InteractorFactory> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor).await;

    ioc.delete_note(Box::new(id_provider)).execute(Input { id }).await
}
