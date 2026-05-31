use leptos::prelude::*;

#[server]
pub async fn remove_self() -> Result<(), ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::{extract, ResponseOptions};
    use actix_web::http::header;
    use crate::adapters::auth::token::TokenProcessor;

    let req: HttpRequest = extract().await?;
    let token_processor: Data<TokenProcessor> = extract().await?;

    if let Some(cookie) = req.cookie("session") {
        token_processor.remove_token_session(cookie.value());
    }

    let opts = expect_context::<ResponseOptions>();
    opts.insert_header(
        header::SET_COOKIE,
        header::HeaderValue::from_str(
            "session=; HttpOnly; SameSite=Strict; Path=/; Max-Age=0",
        )
            .map_err(|e| ServerFnError::new(e.to_string()))?,
    );

    Ok(())
}

#[server]
pub async fn create(username: String, password: String) -> Result<(), ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::{extract, ResponseOptions};
    use actix_web::http::header;
    use crate::adapters::database::user_verifier::SqliteUserVerifier;
    use crate::adapters::auth::token::TokenProcessor;

    let verifier: Data<SqliteUserVerifier> = extract().await?;
    let token_processor: Data<TokenProcessor> = extract().await?;

    if !verifier.verify(&username, &password).await {
        return Err(ServerFnError::new("Неверный логин или пароль"));
    }

    let token = token_processor.set_token_session(&username);

    let opts = expect_context::<ResponseOptions>();
    opts.insert_header(
        header::SET_COOKIE,
        header::HeaderValue::from_str(&format!(
            "session={}; HttpOnly; SameSite=Strict; Path=/; Max-Age=604800",
            token
        ))
            .map_err(|e| ServerFnError::new(e.to_string()))?,
    );

    Ok(())
}