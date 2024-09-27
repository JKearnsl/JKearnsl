pub mod notes;
pub mod projects;

use crate::adapters::database::pool::DbPool;

pub trait CreateIFNotExists {
    async fn create_if_not_exists(db_pool: &DbPool) -> Result<(), sqlx::Error>;
}
