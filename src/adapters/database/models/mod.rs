pub mod notes;
pub mod projects;
pub mod users;
pub mod tags;
pub mod note_tags;

use async_trait::async_trait;
use crate::adapters::database::pool::DbPool;

#[async_trait]
pub trait CreateIFNotExists {
    async fn create_if_not_exists(db_pool: &DbPool) -> Result<(), sqlx::Error>;
}
