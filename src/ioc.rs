use crate::adapters::argon2_password_hasher::Argon2PasswordHasher;
use crate::adapters::database::note_db::NoteGateway;
use crate::adapters::database::pool::DbPool;
use crate::adapters::database::project_db::ProjectGateway;
use crate::application::common::id_provider::IdProvider;
use crate::application::create_session::CreateSession;
use crate::application::get_user_self::GetUserSelf;
use crate::CredentialsProvider;
use crate::domain::services::note::NoteService;
use crate::domain::services::project::ProjectService;
use crate::domain::services::validator::ValidatorService;
use crate::presentation::interactor_factory::InteractorFactory;

pub struct IoC {
    note_gateway: NoteGateway,
    note_service: NoteService,

    project_gateway: ProjectGateway,
    project_service: ProjectService,

    password_hasher: Argon2PasswordHasher,
    validator: ValidatorService,
    credential_provider: CredentialsProvider,
}

impl IoC {
    pub fn new(
        db_pool: DbPool,
        credential_provider: CredentialsProvider,
    ) -> Self {
        Self {
            note_gateway: NoteGateway::new(db_pool.clone()),
            note_service: NoteService { },

            project_gateway: ProjectGateway::new(db_pool.clone()),
            project_service: ProjectService { },

            password_hasher: Argon2PasswordHasher::new(),
            validator: ValidatorService::new(),
            credential_provider,
        }
    }
}

impl InteractorFactory for IoC {
    fn get_user_self(&self, id_provider: Box<dyn IdProvider>) -> GetUserSelf {
        GetUserSelf {
            id_provider,
        }
    }

    fn create_session(&self, id_provider: Box<dyn IdProvider>) -> CreateSession {
        CreateSession {
            id_provider,
            password_hasher: &self.password_hasher,
            validator: &self.validator,
            credential_provider: &self.credential_provider
        }
    }
}