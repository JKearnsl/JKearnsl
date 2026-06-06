use crate::application::{
    common::{
        exceptions::ApplicationError,
        interactor::Interactor,
    },
    user::create::Input,
};
use crate::interactor_factory::InteractorFactory;

pub async fn run(ioc: &dyn InteractorFactory) -> Result<(), ApplicationError> {
    const USERNAME: &str = "admin";
    const PASSWORD: &str = "admin";

    match ioc.create_user()
        .execute(Input { username: USERNAME.to_string(), password: PASSWORD.to_string() })
        .await
    {
        Ok(()) => log::info!("default user created — login: {}, password: {}", USERNAME, PASSWORD),
        Err(ApplicationError::ValidationError(_)) => {}
        Err(e) => return Err(e),
    }
    Ok(())
}
