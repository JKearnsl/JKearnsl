use async_trait::async_trait;
use crate::domain::models::note::{Note, NoteId, NoteListItem};


#[async_trait]
pub trait NoteReader{
    async fn get(&self, id: &NoteId) -> Option<Note>;
    async fn range(&self, limit: &u64, offset: &u64) -> Vec<NoteListItem>;
}

#[async_trait]
pub trait NoteWriter{
    async fn save(&self, note: &Note);
}

#[async_trait]
pub trait NoteRemover {
    async fn remove(&self, note_id: &NoteId);
}

pub trait NoteGateway: NoteReader + NoteWriter + NoteRemover {}


#[cfg(test)]
pub mod test {
    use std::collections::HashMap;
    use crate::domain::models::note::{Note, NoteListItem};
    use crate::application::common::note_gateway::{NoteReader, NoteWriter, NoteRemover};
    use async_trait::async_trait;
    use tokio::sync::Mutex;
    use crate::domain::models::note::NoteId;

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
        async fn get(&self, id: &NoteId) -> Option<Note> {
            self.notes.lock().await.get(id).cloned()
        }

        async fn range(&self, limit: &u64, offset: &u64) -> Vec<NoteListItem> {
            self.notes.lock().await.values().cloned().collect().map(|n| NoteListItem {
                id: n.id,
                title: n.title,
                description: n.description,
                created_at: n.created_at,
                updated_at: n.updated_at
            }).collect()
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
