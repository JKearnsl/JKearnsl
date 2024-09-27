use core::option::Option;

use async_trait::async_trait;

use crate::adapters::database::pool::DbPool;
use crate::application::common::note_gateway::{
    NoteGateway as NoteGatewayTrait,
    NoteReader,
    NoteRemover,
    NoteWriter
};
use crate::domain::models::note::{Note as NoteDomain, NoteId};
use crate::adapters::database::models::notes::{Note, NOTE_TABLE};


pub struct NoteGateway{
    db: DbPool,
}

impl NoteGateway {
    pub fn new(db: DbPool) -> Self {
        NoteGateway {
            db,
        }
    }
}

#[async_trait]
impl NoteReader for NoteGateway {
    async fn get_note(&self, note_id: &NoteId) -> Option<NoteDomain> {
        let row: Option<Note> = sqlx::query_as(
            format!("SELECT * FROM {} WHERE id = $1", NOTE_TABLE).as_str()
        )
            .bind(note_id)
            .fetch_optional(&self.db).await.unwrap();

        match row {
            None => None,
            Some(row) => Some(map_note_model_to_domain(row))
        }
    }

    async fn get_notes_without_body_range(&self, limit: &u64, offset: &u64) -> Vec<NoteDomain> {
        let rows: Vec<Note> = sqlx::query_as(format!(
            "SELECT id, title, description, created_at updated_at FROM {} LIMIT $1 OFFSET $2", 
            NOTE_TABLE
        ).as_str())
            .bind(limit.clone() as i64)
            .bind(offset.clone() as i64)
            .fetch_all(&self.db).await.unwrap();

        rows.into_iter().map(|row| map_note_model_to_domain(row)).collect()
    }
}

#[async_trait]
impl NoteWriter for NoteGateway {
    async fn save_note(&self, note: &NoteDomain) {
        sqlx::query(format!(
            "INSERT INTO {} (id, title, description, body, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO UPDATE SET \
             title = $2, description = $3, body = $4, created_at = $5, updated_at = $6",
            NOTE_TABLE
        ).as_str())
            .bind(&note.id)
            .bind(&note.title)
            .bind(&note.description)
            .bind(&note.body)
            .bind(&note.created_at)
            .bind(&note.updated_at)
            .execute(&self.db).await.unwrap();
    }
}

#[async_trait]
impl NoteRemover for NoteGateway {
    async fn remove_note(&self, note_id: &NoteId) {
        sqlx::query(format!("DELETE FROM {} WHERE id = $1", NOTE_TABLE).as_str())
            .bind(note_id)
            .execute(&self.db).await.unwrap();
    }
}

fn map_note_model_to_domain(note: Note) -> NoteDomain {
    NoteDomain {
        id: note.id,
        title: note.title,
        description: note.description,
        created_at: note.created_at,
        updated_at: note.updated_at,
        body: note.body,
    }
}

impl NoteGatewayTrait for NoteGateway {}
