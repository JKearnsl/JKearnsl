use crate::adapters::database::models::notes::Note;
use crate::adapters::database::models::projects::Project;
use crate::adapters::database::models::CreateIFNotExists;
use crate::adapters::database::pool::DbPool;

pub async fn initial_models(db: &DbPool) -> Result<(), sqlx::Error> {
    Note::create_if_not_exists(db).await?;
    Project::create_if_not_exists(db).await?;
    Ok(())
}
