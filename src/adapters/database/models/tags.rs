pub type TagId = String;

#[derive(sqlx::FromRow)]
pub struct TagRow {
    pub id: TagId,
    pub name: String,
}
