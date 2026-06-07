use crate::domain::models::user::{User, UserId};

#[derive(sqlx::FromRow)]
pub struct UserRow {
    pub id: UserId,
    pub username: String,
    pub password_phc: String,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            username: row.username,
            password_hash: row.password_phc,
        }
    }
}
