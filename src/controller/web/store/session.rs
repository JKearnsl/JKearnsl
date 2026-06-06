use leptos::prelude::*;
use crate::application::common::exceptions::ApplicationError;
use crate::controller::web::lib::api::users;
use crate::domain::models::user::UserSummary;

#[derive(Clone, Copy)]
pub struct SessionStore {
    user: Resource<Result<Option<UserSummary>, ApplicationError>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self {
            user: Resource::new(|| (), |_| users::get_self()),
        }
    }

    pub fn refetch(&self) {
        self.user.refetch();
    }

    pub fn get(&self) -> Option<UserSummary> {
        self.user.get()?.ok()?
    }

    pub fn is_auth(&self) -> bool {
        self.get().is_some()
    }
}
