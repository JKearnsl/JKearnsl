use chrono::DateTime;
use crate::domain::models::hash::Hash;
use crate::domain::models::session::Session;
use crate::domain::models::user::UserId;

#[derive(sqlx::FromRow)]
pub struct SessionRow {
    pub token_hash: Vec<u8>,
    pub user_id: UserId,
    pub created_at: i64,
}

impl From<SessionRow> for Session {
    fn from(row: SessionRow) -> Self {
        Session {
            token_hash: Hash(row.token_hash),
            user_id: row.user_id,
            created_at: DateTime::from_timestamp(row.created_at, 0)
                .unwrap_or_default()
                .with_timezone(&chrono::Utc),
        }
    }
}
