use leptos::prelude::*;
use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::user::UserSummary;

#[server]
pub async fn get_self() -> Result<Option<UserSummary>, ApplicationError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::TokenProcessor;

    let req: HttpRequest = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let token_processor: Data<TokenProcessor> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    Ok(match token {
        Some(t) => token_processor.get_token_session(&t).await
            .map(|(id, username)| UserSummary { id, username }),
        None => None,
    })
}
