use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::application::note::list::ListNotes;
    use crate::application::note::get_by_slug::GetBySlugNote;
    use crate::application::note::get_by_id_admin::GetByIdNoteAdmin;
    use crate::application::note::create::CreateNote;
    use crate::application::note::update::UpdateNote;
    use crate::application::note::delete::DeleteNote;
    use crate::application::session::create::CreateSession;
    use crate::application::common::id_provider::IdProvider;

    pub trait InteractorFactory: Send + Sync {
        fn list_notes(&self) -> ListNotes<'_>;
        fn list_all_notes(&self) -> ListNotes<'_>;
        fn get_note_by_slug(&self) -> GetBySlugNote<'_>;
        fn get_note_by_id_admin(&self) -> GetByIdNoteAdmin<'_>;
        fn create_note(&self, id_provider: Box<dyn IdProvider>) -> CreateNote<'_>;
        fn update_note(&self, id_provider: Box<dyn IdProvider>) -> UpdateNote<'_>;
        fn delete_note(&self, id_provider: Box<dyn IdProvider>) -> DeleteNote<'_>;
        fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession<'_>;
    }
}}
