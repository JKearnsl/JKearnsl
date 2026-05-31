use async_trait::async_trait;
use crate::domain::models::note::{Note, NoteId, NoteListItem};


#[async_trait]
pub trait NoteReader: Send + Sync {
    async fn get_by_id(&self, id: &NoteId) -> Option<Note>;
    async fn get_by_id_admin(&self, id: &NoteId) -> Option<Note>;
    async fn get_by_slug(&self, slug: &str) -> Option<Note>;
    async fn range(&self, limit: &u64, offset: &u64) -> Vec<NoteListItem>;
    async fn range_all(&self, limit: &u64, offset: &u64) -> Vec<NoteListItem>;
    async fn range_by_category(&self, category: &str, limit: &u64, offset: &u64) -> Vec<NoteListItem>;
    async fn range_by_tag(&self, tag: &str, limit: &u64, offset: &u64) -> Vec<NoteListItem>;
    async fn next_no(&self) -> u32;
}

#[async_trait]
pub trait NoteWriter: Send + Sync {
    async fn save(&self, note: &Note);
}

#[async_trait]
pub trait NoteRemover: Send + Sync {
    async fn remove(&self, note_id: &NoteId);
}

pub trait NoteGateway: NoteReader + NoteWriter + NoteRemover {}


#[cfg(test)]
pub mod test {
    use std::collections::HashMap;
    use crate::domain::models::note::{Note, NoteId, NoteListItem};
    use crate::application::common::note_gateway::{NoteReader, NoteWriter, NoteRemover};
    use async_trait::async_trait;
    use tokio::sync::Mutex;

    pub struct MockNoteGateway {
        pub notes: Mutex<HashMap<NoteId, Note>>
    }

    impl MockNoteGateway {
        pub fn new(notes: HashMap<NoteId, Note>) -> Self {
            Self {
                notes: Mutex::new(notes)
            }
        }
    }

    #[async_trait]
    impl NoteReader for MockNoteGateway {
        async fn get_by_id(&self, id: &NoteId) -> Option<Note> {
            use crate::domain::models::note::State;
            self.notes.lock().await.get(id)
                .filter(|n| n.state == State::Published)
                .cloned()
        }

        async fn get_by_id_admin(&self, id: &NoteId) -> Option<Note> {
            self.notes.lock().await.get(id).cloned()
        }

        async fn get_by_slug(&self, slug: &str) -> Option<Note> {
            self.notes.lock().await.values().find(|n| n.slug == slug).cloned()
        }

        async fn range(&self, limit: &u64, offset: &u64) -> Vec<NoteListItem> {
            use crate::domain::models::note::State;
            self.notes.lock().await.values()
                .filter(|n| n.state == State::Published)
                .skip(*offset as usize)
                .take(*limit as usize)
                .cloned()
                .map(NoteListItem::from)
                .collect()
        }

        async fn range_all(&self, limit: &u64, offset: &u64) -> Vec<NoteListItem> {
            self.notes.lock().await.values()
                .skip(*offset as usize)
                .take(*limit as usize)
                .cloned()
                .map(NoteListItem::from)
                .collect()
        }

        async fn range_by_category(&self, category: &str, limit: &u64, offset: &u64) -> Vec<NoteListItem> {
            use crate::domain::models::note::State;
            self.notes.lock().await.values()
                .filter(|n| n.category == category && n.state == State::Published)
                .skip(*offset as usize)
                .take(*limit as usize)
                .cloned()
                .map(NoteListItem::from)
                .collect()
        }

        async fn range_by_tag(&self, tag: &str, limit: &u64, offset: &u64) -> Vec<NoteListItem> {
            use crate::domain::models::note::State;
            self.notes.lock().await.values()
                .filter(|n| n.tags.iter().any(|t| t == tag) && n.state == State::Published)
                .skip(*offset as usize)
                .take(*limit as usize)
                .cloned()
                .map(NoteListItem::from)
                .collect()
        }

        async fn next_no(&self) -> u32 {
            self.notes.lock().await.values()
                .map(|n| n.no)
                .max()
                .unwrap_or(0) + 1
        }
    }

    #[async_trait]
    impl NoteWriter for MockNoteGateway {
        async fn save(&self, note: &Note) {
            self.notes.lock().await.insert(note.id.clone(), note.clone());
        }
    }

    #[async_trait]
    impl NoteRemover for MockNoteGateway {
        async fn remove(&self, note_id: &NoteId) {
            self.notes.lock().await.remove(note_id);
        }
    }
}
