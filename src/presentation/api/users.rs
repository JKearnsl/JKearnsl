use leptos::prelude::*;

#[server]
pub async fn get_self() -> Result<Option<String>, ServerFnError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::TokenProcessor;

    let req: HttpRequest = extract().await?;
    let token_processor: Data<TokenProcessor> = extract().await?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    Ok(match token {
        Some(t) => token_processor.get_token_session(&t).ok(),
        None => None,
    })
}
