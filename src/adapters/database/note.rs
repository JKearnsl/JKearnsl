use async_trait::async_trait;
use sqlx::Row;
use crate::application::common::note_gateway::{
    NoteGateway as NoteGatewayTrait, NoteReader, NoteRemover, NoteWriter,
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

fn parse_tags(s: Option<&str>) -> Vec<String> {
    match s {
        Some(s) if !s.is_empty() => s.split(',').map(|t| t.to_string()).collect(),
        _ => vec![],
    }
}

fn row_to_note(row: &sqlx::sqlite::SqliteRow) -> Result<Note, sqlx::Error> {
    let state_str: String = row.try_get("state")?;
    let state = State::try_from(state_str.as_str()).unwrap_or(State::Draft);
    let category_str: String = row.try_get("category")?;
    let category = Category::try_from(category_str.as_str()).unwrap_or(Category::Prog);
    let tags_str: Option<String> = row.try_get("tags")?;
    let featured: i64 = row.try_get("featured")?;
    let no: i64 = row.try_get("no")?;

    Ok(Note {
        id: row.try_get("id")?,
        no: no as u32,
        slug: row.try_get("slug")?,
        category,
        title: row.try_get("title")?,
        description: row.try_get("description")?,
        body: row.try_get("body")?,
        tags: parse_tags(tags_str.as_deref()),
        featured: featured != 0,
        state,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

fn row_to_list_item(row: &sqlx::sqlite::SqliteRow) -> Result<NoteListItem, sqlx::Error> {
    let tags_str: Option<String> = row.try_get("tags")?;
    let featured: i64 = row.try_get("featured")?;
    let no: i64 = row.try_get("no")?;
    let state_str: String = row.try_get("state")?;
    let state = State::try_from(state_str.as_str()).unwrap_or(State::Draft);
    let category_str: String = row.try_get("category")?;
    let category = Category::try_from(category_str.as_str()).unwrap_or(Category::Prog);

    Ok(NoteListItem {
        id: row.try_get("id")?,
        no: no as u32,
        slug: row.try_get("slug")?,
        category,
        title: row.try_get("title")?,
        description: row.try_get("description")?,
        tags: parse_tags(tags_str.as_deref()),
        featured: featured != 0,
        state,
        created_at: row.try_get("created_at")?,
        updated_at: row.try_get("updated_at")?,
    })
}

#[async_trait]
impl NoteReader for NoteGateway {
    async fn get_by_id(&self, id: &NoteId) -> Option<Note> {
        let row = sqlx::query(
            "SELECT n.id, n.no, n.slug, n.category, n.title, n.description, n.body, \
             n.featured, n.state, n.created_at, n.updated_at, \
             GROUP_CONCAT(t.name, ',') as tags \
             FROM notes n \
             LEFT JOIN note_tags nt ON n.id = nt.note_id \
             LEFT JOIN tags t ON nt.tag_id = t.id \
             WHERE n.id = ? \
             GROUP BY n.id"
        )
        .bind(id)
        .fetch_optional(&self.inner)
        .await
        .ok()??;
        row_to_note(&row).ok()
    }

    async fn get_by_slug(&self, slug: &str) -> Option<Note> {
        let row = sqlx::query(
            "SELECT n.id, n.no, n.slug, n.category, n.title, n.description, n.body, \
             n.featured, n.state, n.created_at, n.updated_at, \
             GROUP_CONCAT(t.name, ',') as tags \
             FROM notes n \
             LEFT JOIN note_tags nt ON n.id = nt.note_id \
             LEFT JOIN tags t ON nt.tag_id = t.id \
             WHERE n.slug = ? AND n.state = 'Published' \
             GROUP BY n.id"
        )
        .bind(slug)
        .fetch_optional(&self.inner)
        .await
        .ok()??;
        row_to_note(&row).ok()
    }

    async fn range(&self, limit: &u64, offset: &u64) -> Vec<NoteListItem> {
        let rows = sqlx::query(
            "SELECT n.id, n.no, n.slug, n.category, n.title, n.description, \
             n.state, n.featured, n.created_at, n.updated_at, \
             GROUP_CONCAT(t.name, ',') as tags \
             FROM notes n \
             LEFT JOIN note_tags nt ON n.id = nt.note_id \
             LEFT JOIN tags t ON nt.tag_id = t.id \
             WHERE n.state = 'Published' \
             GROUP BY n.id \
             ORDER BY n.no DESC LIMIT ? OFFSET ?"
        )
        .bind(*limit as i64)
        .bind(*offset as i64)
        .fetch_all(&self.inner)
        .await
        .unwrap_or_default();

        rows.iter().filter_map(|r| row_to_list_item(r).ok()).collect()
    }

    async fn range_all(&self, limit: &u64, offset: &u64) -> Vec<NoteListItem> {
        let rows = sqlx::query(
            "SELECT n.id, n.no, n.slug, n.category, n.title, n.description, \
             n.state, n.featured, n.created_at, n.updated_at, \
             GROUP_CONCAT(t.name, ',') as tags \
             FROM notes n \
             LEFT JOIN note_tags nt ON n.id = nt.note_id \
             LEFT JOIN tags t ON nt.tag_id = t.id \
             GROUP BY n.id \
             ORDER BY n.no DESC LIMIT ? OFFSET ?"
        )
        .bind(*limit as i64)
        .bind(*offset as i64)
        .fetch_all(&self.inner)
        .await
        .unwrap_or_default();

        rows.iter().filter_map(|r| row_to_list_item(r).ok()).collect()
    }

    async fn range_by_category(&self, category: &str, limit: &u64, offset: &u64) -> Vec<NoteListItem> {
        let rows = sqlx::query(
            "SELECT n.id, n.no, n.slug, n.category, n.title, n.description, \
             n.state, n.featured, n.created_at, n.updated_at, \
             GROUP_CONCAT(t.name, ',') as tags \
             FROM notes n \
             LEFT JOIN note_tags nt ON n.id = nt.note_id \
             LEFT JOIN tags t ON nt.tag_id = t.id \
             WHERE n.state = 'Published' AND n.category = ? \
             GROUP BY n.id \
             ORDER BY n.no DESC LIMIT ? OFFSET ?"
        )
        .bind(category)
        .bind(*limit as i64)
        .bind(*offset as i64)
        .fetch_all(&self.inner)
        .await
        .unwrap_or_default();

        rows.iter().filter_map(|r| row_to_list_item(r).ok()).collect()
    }

    async fn range_by_tag(&self, tag: &str, limit: &u64, offset: &u64) -> Vec<NoteListItem> {
        let rows = sqlx::query(
            "SELECT n.id, n.no, n.slug, n.category, n.title, n.description, \
             n.state, n.featured, n.created_at, n.updated_at, \
             GROUP_CONCAT(t.name, ',') as tags \
             FROM notes n \
             LEFT JOIN note_tags nt ON n.id = nt.note_id \
             LEFT JOIN tags t ON nt.tag_id = t.id \
             WHERE n.state = 'Published' \
               AND EXISTS ( \
                   SELECT 1 FROM note_tags nt2 \
                   JOIN tags t2 ON nt2.tag_id = t2.id \
                   WHERE nt2.note_id = n.id AND t2.name = ? \
               ) \
             GROUP BY n.id \
             ORDER BY n.no DESC LIMIT ? OFFSET ?"
        )
        .bind(tag)
        .bind(*limit as i64)
        .bind(*offset as i64)
        .fetch_all(&self.inner)
        .await
        .unwrap_or_default();

        rows.iter().filter_map(|r| row_to_list_item(r).ok()).collect()
    }

    async fn next_no(&self) -> u32 {
        let row: (i64,) = sqlx::query_as(
            "SELECT COALESCE(MAX(no), 0) + 1 FROM notes"
        )
        .fetch_one(&self.inner)
        .await
        .unwrap_or((1,));
        row.0 as u32
    }
}

#[async_trait]
impl NoteWriter for NoteGateway {
    async fn save(&self, note: &Note) {
        let mut tx = match self.inner.begin().await {
            Ok(tx) => tx,
            Err(_) => return,
        };

        let state = note.state.as_str();
        let featured = note.featured as i64;
        let no = note.no as i64;

        if sqlx::query(
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
        .bind(note.created_at)
        .bind(note.updated_at)
        .execute(&mut *tx)
        .await
        .is_err()
        {
            return;
        }

        if sqlx::query("DELETE FROM note_tags WHERE note_id = ?")
            .bind(&note.id)
            .execute(&mut *tx)
            .await
            .is_err()
        {
            return;
        }

        for tag_name in &note.tags {
            let tag_id = nanoid::nanoid!(16);

            if sqlx::query("INSERT OR IGNORE INTO tags (id, name) VALUES (?, ?)")
                .bind(&tag_id)
                .bind(tag_name)
                .execute(&mut *tx)
                .await
                .is_err()
            {
                return;
            }

            if sqlx::query(
                "INSERT INTO note_tags (note_id, tag_id) \
                 SELECT ?, id FROM tags WHERE name = ?"
            )
            .bind(&note.id)
            .bind(tag_name)
            .execute(&mut *tx)
            .await
            .is_err()
            {
                return;
            }
        }

        tx.commit().await.ok();
    }
}

#[async_trait]
impl NoteRemover for NoteGateway {
    async fn remove(&self, note_id: &NoteId) {
        sqlx::query("DELETE FROM note_tags WHERE note_id = ?")
            .bind(note_id)
            .execute(&self.inner)
            .await
            .ok();
        sqlx::query("DELETE FROM notes WHERE id = ?")
            .bind(note_id)
            .execute(&self.inner)
            .await
            .ok();
    }
}

impl NoteGatewayTrait for NoteGateway {}
