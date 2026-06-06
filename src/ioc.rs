use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    use crate::application::session::vacuum::VacuumSessions;
    use crate::application::note::list::ListNotes;
    use crate::application::note::get_by_slug::GetBySlugNote;
    use crate::application::note::get_by_id::GetByIdNote;
    use crate::application::note::create::CreateNote;
    use crate::application::note::update::UpdateNote;
    use crate::application::note::delete::DeleteNote;
    use crate::application::session::create::CreateSession;
    use crate::application::user::create::CreateUser;
    use crate::application::user::get_self::GetUserSelf;
    use crate::application::common::id_provider::IdProvider;
    use crate::domain::models::user::UserId;

    struct BootstrapIdProvider;

    impl IdProvider for BootstrapIdProvider {
        fn session(&self) -> Option<&String> { None }
        fn user_id(&self) -> Option<&UserId> { None }
        fn username(&self) -> Option<&String> { None }
        fn is_auth(&self) -> bool { true }
    }
    use crate::interactor_factory::InteractorFactory;
    use crate::adapters::database::note::NoteGateway;
    use crate::adapters::database::user::SqliteUserGateway;
    use crate::adapters::database::session::SqliteSessionGateway;
    use crate::adapters::argon2_password_hasher::Argon2PasswordHasher;
    use crate::adapters::database::pool::DbPool;

    pub struct IoC {
        note_gateway: NoteGateway,
        user_gateway: SqliteUserGateway,
        session_gateway: SqliteSessionGateway,
        hasher: Argon2PasswordHasher,
    }

    impl IoC {
        pub fn new(db_pool: DbPool) -> Self {
            Self {
                note_gateway: NoteGateway::new(db_pool.clone()),
                user_gateway: SqliteUserGateway::new(db_pool.clone()),
                session_gateway: SqliteSessionGateway::new(db_pool),
                hasher: Argon2PasswordHasher::new(),
            }
        }
    }

    const SESSION_TTL_SECS: i64 = 60 * 60 * 24 * 7; // 7 days

    impl InteractorFactory for IoC {
        fn list_notes(&self, id_provider: Box<dyn IdProvider>) -> ListNotes<'_> {
            ListNotes { note_reader: &self.note_gateway, id_provider }
        }

        fn get_note_by_slug(&self) -> GetBySlugNote<'_> {
            GetBySlugNote { note_reader: &self.note_gateway }
        }

        fn get_note_by_id(&self, id_provider: Box<dyn IdProvider>) -> GetByIdNote<'_> {
            GetByIdNote { note_reader: &self.note_gateway, id_provider }
        }

        fn create_note(&self, id_provider: Box<dyn IdProvider>) -> CreateNote<'_> {
            CreateNote {
                note_reader: &self.note_gateway,
                note_writer: &self.note_gateway,
                id_provider,
            }
        }

        fn update_note(&self, id_provider: Box<dyn IdProvider>) -> UpdateNote<'_> {
            UpdateNote {
                note_reader: &self.note_gateway,
                note_writer: &self.note_gateway,
                id_provider,
            }
        }

        fn delete_note(&self, id_provider: Box<dyn IdProvider>) -> DeleteNote<'_> {
            DeleteNote {
                note_remover: &self.note_gateway,
                id_provider,
            }
        }

        fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession<'_> {
            CreateSession {
                id_provider,
                user_reader: &self.user_gateway,
                hasher: &self.hasher,
                session_writer: &self.session_gateway,
            }
        }

        fn create_user(&self) -> CreateUser<'_> {
            CreateUser {
                id_provider: Box::new(BootstrapIdProvider),
                user_gateway: &self.user_gateway,
                hasher: &self.hasher,
            }
        }

        fn get_user_self(&self, id_provider: Box<dyn IdProvider>) -> GetUserSelf {
            GetUserSelf { id_provider }
        }

        fn vacuum_sessions(&self) -> VacuumSessions<'_> {
            VacuumSessions { session_vacuum: &self.session_gateway, max_age_secs: SESSION_TTL_SECS }
        }
    }
}}
