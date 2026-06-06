use async_trait::async_trait;
use crate::application::common::exceptions::ApplicationError;
use crate::domain::models::note::{Category, Note, NoteId, NoteListItem, State};

pub enum NoteGatewayError {
    NotFound,
    Internal(String),
}

impl From<NoteGatewayError> for ApplicationError {
    fn from(e: NoteGatewayError) -> Self {
        match e {
            NoteGatewayError::NotFound => ApplicationError::NotFound,
            NoteGatewayError::Internal(msg) => ApplicationError::Internal(msg),
        }
    }
}

#[async_trait]
pub trait NoteReader: Send + Sync {
    async fn by_id(&self, id: &NoteId) -> Result<Note, NoteGatewayError>;
    async fn by_slug(&self, slug: &str) -> Result<Note, NoteGatewayError>;
    async fn list(
        &self,
        limit: &u64,
        offset: &u64,
        state: Option<&State>,
        category: Option<&Category>,
        tag: Option<&str>,
    ) -> Result<Vec<NoteListItem>, NoteGatewayError>;
    async fn next_no(&self) -> Result<u32, NoteGatewayError>;
}

#[async_trait]
pub trait NoteWriter: Send + Sync {
    async fn save(&self, note: Note) -> Result<(), NoteGatewayError>;
}

#[async_trait]
pub trait NoteRemover: Send + Sync {
    async fn remove(&self, note_id: &NoteId) -> Result<(), NoteGatewayError>;
}

pub trait NoteGateway: NoteReader + NoteWriter + NoteRemover {}


#[cfg(test)]
pub mod test {
    use std::collections::HashMap;
    use crate::domain::models::note::{Category, Note, NoteId, NoteListItem, State};
    use crate::application::common::note_gateway::{NoteGatewayError, NoteReader, NoteWriter, NoteRemover};
    use async_trait::async_trait;
    use tokio::sync::Mutex;

    pub struct MockNoteGateway {
        pub notes: Mutex<HashMap<NoteId, Note>>
    }

    impl MockNoteGateway {
        pub fn new(notes: HashMap<NoteId, Note>) -> Self {
            Self { notes: Mutex::new(notes) }
        }
    }

    #[async_trait]
    impl NoteReader for MockNoteGateway {
        async fn by_id(&self, id: &NoteId) -> Result<Note, NoteGatewayError> {
            self.notes.lock().await.get(id).cloned().ok_or(NoteGatewayError::NotFound)
        }

        async fn by_slug(&self, slug: &str) -> Result<Note, NoteGatewayError> {
            self.notes.lock().await.values()
                .find(|n| n.slug == slug)
                .cloned()
                .ok_or(NoteGatewayError::NotFound)
        }

        async fn list(
            &self,
            limit: &u64,
            offset: &u64,
            state: Option<&State>,
            category: Option<&Category>,
            tag: Option<&str>,
        ) -> Result<Vec<NoteListItem>, NoteGatewayError> {
            Ok(self.notes.lock().await.values()
                .filter(|n| state.map_or(true, |s| &n.state == s))
                .filter(|n| category.map_or(true, |c| &n.category == c))
                .filter(|n| tag.map_or(true, |t| n.tags.iter().any(|tg| tg == t)))
                .skip(*offset as usize)
                .take(*limit as usize)
                .cloned()
                .map(NoteListItem::from)
                .collect())
        }

        async fn next_no(&self) -> Result<u32, NoteGatewayError> {
            Ok(self.notes.lock().await.values().map(|n| n.no).max().unwrap_or(0) + 1)
        }
    }

    #[async_trait]
    impl NoteWriter for MockNoteGateway {
        async fn save(&self, note: Note) -> Result<(), NoteGatewayError> {
            self.notes.lock().await.insert(note.id.clone(), note);
            Ok(())
        }
    }

    #[async_trait]
    impl NoteRemover for MockNoteGateway {
        async fn remove(&self, note_id: &NoteId) -> Result<(), NoteGatewayError> {
            self.notes.lock().await.remove(note_id);
            Ok(())
        }
    }
}
