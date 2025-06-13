pub mod notes;
pub mod projects;

use async_trait::async_trait;
use crate::adapters::database::pool::DbPool;

#[async_trait]
pub trait CreateIFNotExists {
    async fn create_if_not_exists(db_pool: &DbPool) -> Result<(), sqlx::Error>;
}
