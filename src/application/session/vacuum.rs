use async_trait::async_trait;
use crate::application::common::{
    exceptions::ApplicationError,
    interactor::Interactor,
    session_gateway::SessionRemover,
};

pub struct VacuumSessions<'a> {
    pub session_remover: &'a dyn SessionRemover,
    pub max_age_secs: i64,
}

#[async_trait]
impl Interactor<(), u64> for VacuumSessions<'_> {
    async fn execute(&self, _data: ()) -> Result<u64, ApplicationError> {
        Ok(self.session_remover.remove_older_than(self.max_age_secs).await?)
    }
}
