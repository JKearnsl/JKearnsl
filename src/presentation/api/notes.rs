use leptos::prelude::*;
use crate::domain::models::note::{Note, NoteListItem};

#[server]
pub async fn list_by_category(category: Option<String>) -> Result<Vec<NoteListItem>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::list::ListNotesRequest;
    use crate::domain::models::note::Category;

    let ioc: Data<dyn InteractorFactory> = extract().await?;
    let category = category.and_then(|c| Category::try_from(c.as_str()).ok());
    let result = ioc.list_notes().execute(ListNotesRequest {
        category,
        limit: 200,
        offset: 0,
    }).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(result.notes)
}

#[server]
pub async fn by_slug(slug: String) -> Result<Option<Note>, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::get_by_slug::GetBySlugNoteRequest;
    use pulldown_cmark::{Parser, Options, html};

    let ioc: Data<dyn InteractorFactory> = extract().await?;
    match ioc.get_note_by_slug().execute(GetBySlugNoteRequest { slug }).await {
        Ok(r) => {
            let mut note = r.note;
            let mut opts = Options::empty();
            opts.insert(Options::ENABLE_TABLES);
            opts.insert(Options::ENABLE_STRIKETHROUGH);
            opts.insert(Options::ENABLE_TASKLISTS);
            opts.insert(Options::ENABLE_FOOTNOTES);
            let parser = Parser::new_ext(&note.body, opts);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);
            note.body = html_output;
            Ok(Some(note))
        },
        Err(crate::application::common::exceptions::ApplicationError::NotFound) => Ok(None),
        Err(e) => Err(ServerFnError::new(e.to_string())),
    }
}

#[server]
pub async fn list_as_admin() -> Result<Vec<NoteListItem>, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::list::ListNotesRequest;

    let req: HttpRequest = extract().await?;
    let token_processor: Data<TokenProcessor> = extract().await?;
    let ioc: Data<dyn InteractorFactory> = extract().await?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor)
        .map_err(ServerFnError::new)?;

    {
        use crate::application::common::id_provider::IdProvider;
        if !id_provider.is_auth() {
            return Err(ServerFnError::new("Unauthorized"));
        }
    }

    let result = ioc
        .list_all_notes()
        .execute(ListNotesRequest { category: None, limit: 1000, offset: 0 })
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(result.notes)
}

#[server]
pub async fn get_for_edit(id: String) -> Result<Option<Note>, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::get_by_id_admin::GetByIdNoteAdminRequest;
    use crate::application::common::exceptions::ApplicationError;

    let req: HttpRequest = extract().await?;
    let token_processor: Data<TokenProcessor> = extract().await?;
    let ioc: Data<dyn InteractorFactory> = extract().await?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor)
        .map_err(ServerFnError::new)?;

    {
        use crate::application::common::id_provider::IdProvider;
        if !id_provider.is_auth() {
            return Err(ServerFnError::new("Unauthorized"));
        }
    }

    match ioc.get_note_by_id_admin().execute(GetByIdNoteAdminRequest { id }).await {
        Ok(r) => Ok(Some(r.note)),
        Err(ApplicationError::NotFound) => Ok(None),
        Err(e) => Err(ServerFnError::new(e.to_string())),
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
) -> Result<String, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::create::CreateNoteRequest;

    let req: HttpRequest = extract().await?;
    let token_processor: Data<TokenProcessor> = extract().await?;
    let ioc: Data<dyn InteractorFactory> = extract().await?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor)
        .map_err(ServerFnError::new)?;

    let tags: Vec<String> = tags_raw
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    use crate::domain::models::note::Category;
    let category = Category::try_from(category.as_str())
        .map_err(|e| ServerFnError::new(e))?;

    let result = ioc
        .create_note(Box::new(id_provider))
        .execute(CreateNoteRequest { title, description, body, category, tags, featured, publish })
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

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
) -> Result<String, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::update::UpdateNoteRequest;

    let req: HttpRequest = extract().await?;
    let token_processor: Data<TokenProcessor> = extract().await?;
    let ioc: Data<dyn InteractorFactory> = extract().await?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor)
        .map_err(ServerFnError::new)?;

    let tags: Vec<String> = tags_raw
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    use crate::domain::models::note::Category;
    let category = Category::try_from(category.as_str())
        .map_err(|e| ServerFnError::new(e))?;

    let result = ioc
        .update_note(Box::new(id_provider))
        .execute(UpdateNoteRequest { id, title, description, body, category, tags, featured, publish })
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(result.slug)
}

#[server]
pub async fn delete(id: String) -> Result<(), ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::application::note::delete::DeleteNoteRequest;

    let req: HttpRequest = extract().await?;
    let token_processor: Data<TokenProcessor> = extract().await?;
    let ioc: Data<dyn InteractorFactory> = extract().await?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor)
        .map_err(ServerFnError::new)?;

    ioc
        .delete_note(Box::new(id_provider))
        .execute(DeleteNoteRequest { id })
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server]
pub async fn preview_markdown(body: String) -> Result<String, ServerFnError> {
    use pulldown_cmark::{html, Options, Parser};
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_TASKLISTS);
    opts.insert(Options::ENABLE_FOOTNOTES);
    let parser = Parser::new_ext(&body, opts);
    let mut out = String::new();
    html::push_html(&mut out, parser);
    Ok(out)
}
