use chrono::{DateTime, Utc};
use crate::domain::models::note::{Category, Note, NoteListItem, State, NoteId};

#[derive(sqlx::FromRow)]
pub struct NoteRow {
    pub id: NoteId,
    pub no: i64,
    pub slug: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub featured: i64,
    pub state: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub tags: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct NoteListItemRow {
    pub id: NoteId,
    pub no: i64,
    pub slug: String,
    pub category: String,
    pub title: String,
    pub description: String,
    pub featured: i64,
    pub state: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub tags: Option<String>,
}

fn parse_tags(s: Option<String>) -> Vec<String> {
    match s {
        Some(s) if !s.is_empty() => s.split(',').map(|t| t.to_string()).collect(),
        _ => vec![],
    }
}

fn ts_to_dt(ts: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(ts, 0)
        .unwrap_or_default()
        .with_timezone(&Utc)
}

impl From<NoteRow> for Note {
    fn from(row: NoteRow) -> Self {
        Note {
            id: row.id,
            no: row.no as u32,
            slug: row.slug,
            category: Category::try_from(row.category.as_str()).unwrap_or(Category::Prog),
            title: row.title,
            description: row.description,
            body: row.body,
            tags: parse_tags(row.tags),
            featured: row.featured != 0,
            state: State::try_from(row.state.as_str()).unwrap_or(State::Draft),
            created_at: ts_to_dt(row.created_at),
            updated_at: row.updated_at.and_then(|ts| DateTime::from_timestamp(ts, 0))
                .map(|dt| dt.with_timezone(&Utc)),
        }
    }
}

impl From<NoteListItemRow> for NoteListItem {
    fn from(row: NoteListItemRow) -> Self {
        NoteListItem {
            id: row.id,
            no: row.no as u32,
            slug: row.slug,
            category: Category::try_from(row.category.as_str()).unwrap_or(Category::Prog),
            title: row.title,
            description: row.description,
            tags: parse_tags(row.tags),
            featured: row.featured != 0,
            state: State::try_from(row.state.as_str()).unwrap_or(State::Draft),
            created_at: ts_to_dt(row.created_at),
            updated_at: row.updated_at.and_then(|ts| DateTime::from_timestamp(ts, 0))
                .map(|dt| dt.with_timezone(&Utc)),
        }
    }
}
