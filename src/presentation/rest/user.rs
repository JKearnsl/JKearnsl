use actix_web::{get, HttpRequest, HttpResponse, Result, web};

use crate::adapters::auth::token::TokenProcessor;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::presentation::id_provider::make_token_provider;
use crate::presentation::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(user_self)
    );
}

#[get("/self")]
async fn user_self(
    ioc: web::Data<dyn InteractorFactory>,
    token_processor: web::Data<TokenProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_token_provider(
        &req,
        &token_processor,
    )?;
    let data = ioc.get_user_self(id_provider).execute(()).await?;
    Ok(HttpResponse::Ok().json(data))
}
