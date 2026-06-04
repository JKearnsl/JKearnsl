use leptos::prelude::*;
use crate::application::common::exceptions::ApplicationError;

#[server]
pub async fn remove_self() -> Result<(), ApplicationError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::{extract, ResponseOptions};
    use actix_web::http::header;
    use crate::adapters::auth::token::TokenProcessor;

    let req: HttpRequest = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let token_processor: Data<TokenProcessor> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;

    if let Some(cookie) = req.cookie("session") {
        token_processor.remove_token_session(cookie.value()).await;
    }

    let opts = expect_context::<ResponseOptions>();
    opts.insert_header(
        header::SET_COOKIE,
        header::HeaderValue::from_str("session=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0")
            .map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?,
    );

    Ok(())
}

#[server]
pub async fn create(username: String, password: String) -> Result<(), ApplicationError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::{extract, ResponseOptions};
    use actix_web::http::header;
    use crate::adapters::auth::token::{IdTokenProvider, TokenProcessor};
    use crate::adapters::database::user::SqliteUserGateway;
    use crate::adapters::database::session::SqliteSessionGateway;
    use crate::adapters::argon2_password_hasher::Argon2PasswordHasher;
    use crate::application::session::create::{CreateSession, Input};
    use crate::application::common::interactor::Interactor;

    let req: HttpRequest = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let user_gateway: Data<SqliteUserGateway> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let session_gateway: Data<SqliteSessionGateway> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let token_processor: Data<TokenProcessor> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;

    let session_token = req.cookie("session").map(|c| c.value().to_owned());
    let id_provider = Box::new(IdTokenProvider::new(session_token, &token_processor).await);

    let hasher = Argon2PasswordHasher::new();

    let token = CreateSession {
        id_provider,
        user_reader: user_gateway.as_ref(),
        hasher: &hasher,
        session_writer: session_gateway.as_ref(),
    }
    .execute(Input { username, password })
    .await?;

    let hex: String = token.iter().map(|b| format!("{:02x}", b)).collect();

    let opts = expect_context::<ResponseOptions>();
    opts.insert_header(
        header::SET_COOKIE,
        header::HeaderValue::from_str(&format!(
            "session={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=604800",
            hex
        ))
        .map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?,
    );

    Ok(())
}
