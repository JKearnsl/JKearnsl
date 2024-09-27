use crate::application::common::id_provider::IdProvider;
use crate::application::create_session::CreateSession;
use crate::application::get_user_self::GetUserSelf;

pub trait InteractorFactory {
    fn get_user_self(&self, id_provider: Box<dyn IdProvider>) -> GetUserSelf;
    fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession;
}
