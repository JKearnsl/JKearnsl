use cfg_if::cfg_if;


cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::adapters::auth::token::{IdTokenProvider, TokenProcessor};
    use crate::application::common::id_provider::IdProvider;
    use actix_web::HttpRequest;
    use leptos::prelude::ServerFnError;

    pub fn make_token_provider(
        req: &HttpRequest,
        token_processor: &TokenProcessor,
    ) -> Result<Box<dyn IdProvider>, ServerFnError> {
        let token = req.cookie("token").map(|cookie| cookie.value().to_string());
        match IdTokenProvider::new(token, token_processor) {
            Ok(provider) => Ok(Box::new(provider)),
            Err(error) => Err(ServerFnError::ServerError(error))
        }
    }
}}
