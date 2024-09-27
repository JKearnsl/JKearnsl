use actix_web::{HttpRequest, HttpResponse, post, Result, web};
use actix_web::cookie::Cookie;
use crate::adapters::auth::token::TokenProcessor;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::create_session::CreateSessionDTO;
use crate::presentation::id_provider::make_token_provider;
use crate::presentation::interactor_factory::InteractorFactory;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sessions")
            .service(create_session)
    );
}

#[post("")]
async fn create_session(
    data: web::Json<CreateSessionDTO>,
    ioc: web::Data<dyn InteractorFactory>,
    token_processor: web::Data<TokenProcessor>,
    req: HttpRequest
) -> Result<HttpResponse, ApplicationError> {
    let id_provider = make_token_provider(
        &req,
        &token_processor,
    )?;
    let result= ioc.create_session(id_provider).execute(
        data.into_inner()
    ).await?;

    let token = token_processor.set_token_session(&result.username);
    
    let mut response = HttpResponse::NoContent();
    response.cookie(
        Cookie::build("token", token)
            .path("/")
            .http_only(true)
            .finish()
    );

    Ok(response.finish())
}
