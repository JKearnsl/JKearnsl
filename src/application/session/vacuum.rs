use async_trait::async_trait;
use crate::application::common::exceptions::ApplicationError;
use crate::application::common::interactor::Interactor;
use crate::application::common::session_gateway::SessionVacuum;

pub struct VacuumSessions<'a> {
    pub session_vacuum: &'a dyn SessionVacuum,
    pub max_age_secs: i64,
}

#[async_trait]
impl Interactor<(), u64> for VacuumSessions<'_> {
    async fn execute(&self, _data: ()) -> Result<u64, ApplicationError> {
        Ok(self.session_vacuum.remove_older_than(self.max_age_secs).await)
    }
}
