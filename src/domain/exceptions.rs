use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
pub enum DomainError {
    #[display("Необходима авторизация")]
    AuthorizationRequired,

    #[display("У Вас нет доступа к этому ресурсу")]
    AccessDenied,
}
