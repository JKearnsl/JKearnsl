use leptos::prelude::*;
use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::user::UserSummary;

#[server]
pub async fn get_self() -> Result<Option<UserSummary>, ApplicationError> {
    use actix_web::{web::Data, HttpRequest};
    use leptos_actix::extract;
    use crate::adapters::auth::token::{TokenProcessor, IdTokenProvider};
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;

    let req: HttpRequest = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let token_processor: Data<TokenProcessor> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;
    let ioc: Data<dyn InteractorFactory> = extract().await.map_err(|e| ApplicationError::UnexpectedError(e.to_string()))?;

    let token = req.cookie("session").map(|c| c.value().to_string());
    let id_provider = IdTokenProvider::new(token, &token_processor).await;

    match ioc.get_user_self(Box::new(id_provider)).execute(()).await {
        Ok(user) => Ok(Some(user)),
        Err(ApplicationError::Unauthorized) => Ok(None),
        Err(e) => Err(e),
    }
}
