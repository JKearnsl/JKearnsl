use async_trait::async_trait;
use sqlx::AssertSqlSafe;
use crate::adapters::database::models::notes::{NoteListItemRow, NoteRow};
use crate::application::common::note_gateway::{
    NoteGateway as NoteGatewayTrait, NoteGatewayError, NoteReader, NoteRemover, NoteWriter,
};
use crate::domain::models::note::{Category, Note, NoteId, NoteListItem, State};

pub struct NoteGateway {
    inner: sqlx::SqlitePool,
}

impl NoteGateway {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self { inner: pool }
    }
}

const NOTE_SELECT: &str =
    "SELECT n.id, n.no, n.slug, n.category, n.title, n.description, n.body, \
     n.featured, n.state, n.created_at, n.updated_at, \
     GROUP_CONCAT(t.name, ',') as tags \
     FROM notes n \
     LEFT JOIN note_tags nt ON n.id = nt.note_id \
     LEFT JOIN tags t ON nt.tag_id = t.id";

const LIST_SELECT: &str =
    "SELECT n.id, n.no, n.slug, n.category, n.title, n.description, \
     n.state, n.featured, n.created_at, n.updated_at, \
     GROUP_CONCAT(t.name, ',') as tags \
     FROM notes n \
     LEFT JOIN note_tags nt ON n.id = nt.note_id \
     LEFT JOIN tags t ON nt.tag_id = t.id";

#[async_trait]
impl NoteReader for NoteGateway {
    async fn by_id(&self, id: &NoteId) -> Result<Note, NoteGatewayError> {
        let row: NoteRow = sqlx::query_as(AssertSqlSafe(format!(
            "{} WHERE n.id = ? GROUP BY n.id",
            NOTE_SELECT
        )))
        .bind(id)
        .fetch_optional(&self.inner)
        .await
        .map_err(|e| NoteGatewayError::Internal(e.to_string()))?
        .ok_or(NoteGatewayError::NotFound)?;

        Ok(Note::from(row))
    }

    async fn by_slug(&self, slug: &str) -> Result<Note, NoteGatewayError> {
        let row: NoteRow = sqlx::query_as(AssertSqlSafe(format!(
            "{} WHERE n.slug = ? AND n.state = 'Published' GROUP BY n.id",
            NOTE_SELECT
        )))
        .bind(slug)
        .fetch_optional(&self.inner)
        .await
        .map_err(|e| NoteGatewayError::Internal(e.to_string()))?
        .ok_or(NoteGatewayError::NotFound)?;

        Ok(Note::from(row))
    }

    async fn list(
        &self,
        limit: &u64,
        offset: &u64,
        state: Option<&State>,
        category: Option<&Category>,
        tag: Option<&str>,
    ) -> Result<Vec<NoteListItem>, NoteGatewayError> {
        let mut conditions: Vec<&str> = Vec::new();
        if state.is_some() {
            conditions.push("n.state = ?");
        }
        if category.is_some() {
            conditions.push("n.category = ?");
        }
        if tag.is_some() {
            conditions.push(
                "EXISTS (SELECT 1 FROM note_tags nt2 \
                 JOIN tags t2 ON nt2.tag_id = t2.id \
                 WHERE nt2.note_id = n.id AND t2.name = ?)",
            );
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!(" WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "{}{} GROUP BY n.id ORDER BY n.no DESC LIMIT ? OFFSET ?",
            LIST_SELECT, where_clause
        );

        let mut q = sqlx::query_as::<_, NoteListItemRow>(AssertSqlSafe(sql));
        if let Some(s) = state {
            q = q.bind(s.as_str());
        }
        if let Some(cat) = category {
            q = q.bind(cat.as_str());
        }
        if let Some(t) = tag {
            q = q.bind(t);
        }
        q = q.bind(*limit as i64).bind(*offset as i64);

        let rows = q
            .fetch_all(&self.inner)
            .await
            .map_err(|e| NoteGatewayError::Internal(e.to_string()))?;

        Ok(rows.into_iter().map(NoteListItem::from).collect())
    }

    async fn next_no(&self) -> Result<u32, NoteGatewayError> {
        let row: (i64,) = sqlx::query_as("SELECT COALESCE(MAX(no), 0) + 1 FROM notes")
            .fetch_one(&self.inner)
            .await
            .map_err(|e| NoteGatewayError::Internal(e.to_string()))?;
        Ok(row.0 as u32)
    }
}

#[async_trait]
impl NoteWriter for NoteGateway {
    async fn save(&self, note: Note) -> Result<(), NoteGatewayError> {
        let mut tx = self.inner.begin().await
            .map_err(|e| NoteGatewayError::Internal(e.to_string()))?;

        let state = note.state.as_str();
        let featured = note.featured as i64;
        let no = note.no as i64;

        sqlx::query(
            "INSERT INTO notes \
             (id, no, slug, category, title, description, body, featured, state, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) \
             ON CONFLICT(id) DO UPDATE SET \
             no=excluded.no, slug=excluded.slug, category=excluded.category, \
             title=excluded.title, description=excluded.description, body=excluded.body, \
             featured=excluded.featured, state=excluded.state, \
             updated_at=excluded.updated_at"
        )
        .bind(&note.id)
        .bind(no)
        .bind(&note.slug)
        .bind(note.category.as_str())
        .bind(&note.title)
        .bind(&note.description)
        .bind(&note.body)
        .bind(featured)
        .bind(state)
        .bind(note.created_at.timestamp())
        .bind(note.updated_at.map(|dt| dt.timestamp()))
        .execute(&mut *tx)
        .await
        .map_err(|e| NoteGatewayError::Internal(e.to_string()))?;

        sqlx::query("DELETE FROM note_tags WHERE note_id = ?")
            .bind(&note.id)
            .execute(&mut *tx)
            .await
            .map_err(|e| NoteGatewayError::Internal(e.to_string()))?;

        for tag_name in &note.tags {
            let tag_id = nanoid::nanoid!(16);

            sqlx::query("INSERT OR IGNORE INTO tags (id, name) VALUES (?, ?)")
                .bind(&tag_id)
                .bind(tag_name)
                .execute(&mut *tx)
                .await
                .map_err(|e| NoteGatewayError::Internal(e.to_string()))?;

            sqlx::query(
                "INSERT INTO note_tags (note_id, tag_id) \
                 SELECT ?, id FROM tags WHERE name = ?"
            )
            .bind(&note.id)
            .bind(tag_name)
            .execute(&mut *tx)
            .await
            .map_err(|e| NoteGatewayError::Internal(e.to_string()))?;
        }

        tx.commit().await
            .map(|_| ())
            .map_err(|e| NoteGatewayError::Internal(e.to_string()))
    }
}

#[async_trait]
impl NoteRemover for NoteGateway {
    async fn remove(&self, note_id: &NoteId) -> Result<(), NoteGatewayError> {
        sqlx::query("DELETE FROM note_tags WHERE note_id = ?")
            .bind(note_id)
            .execute(&self.inner)
            .await
            .map_err(|e| NoteGatewayError::Internal(e.to_string()))?;

        sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(note_id)
            .execute(&self.inner)
            .await
            .map(|_| ())
            .map_err(|e| NoteGatewayError::Internal(e.to_string()))
    }
}

impl NoteGatewayTrait for NoteGateway {}
