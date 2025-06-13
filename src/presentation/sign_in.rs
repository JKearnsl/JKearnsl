use leptos::attr::value;
use leptos::prelude::ElementChild;
use leptos::prelude::*;
use leptos_meta::Title;
use leptos::form::ActionForm;
use crate::presentation::logged_in::set_logged_in;

#[component]
pub fn SignIn() -> impl IntoView {
    let sign_in_action = ServerAction::<SignIn>::new();

    let value = sign_in_action.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));


    view! {
        <Title text="login" />

        <main>
            <section>
                <h3>Login</h3>
                <p>
                    Enter your username and password to sign in
                </p>
            </section>
            
            <section>

                {move || {
                    let text = if has_error() {
                        value.with(|val| match val {
                            Some(Err(err)) => err.to_string(),
                            _ => unreachable!()
                        })
                    } else {
                        set_logged_in(true);
                        "".to_string()
                    };
                    view! {
                        <p class="error">{text}</p>
                    }
                }}

                <ActionForm action=sign_in_action>
                    <input type="text" placeholder="username" name="username"/><br/>
                    <input type="password" placeholder="********" name="password"/><br/>
                    <input type="submit"/>
                </ActionForm>
        
            </section>
        
        </main>
    }
}

#[server]
pub async fn sign_in(
    username: String,
    password: String
) -> Result<(), ServerFnError> {
    use actix_web::cookie::Cookie;
    use actix_web::http::header;
    use actix_web::http::header::HeaderValue;
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::interactor_factory::InteractorFactory;
    use crate::application::create_session::CreateSessionDTO;
    use crate::application::common::interactor::Interactor;
    use crate::adapters::auth::token::TokenProcessor;
    use super::id_provider::make_token_provider;


    let (ioc, token_processor): (Data<dyn InteractorFactory>, Data<TokenProcessor>) = extract().await?;
    let req = expect_context::<leptos_actix::Request>();
    let res = expect_context::<leptos_actix::ResponseOptions>();

    let id_provider = make_token_provider(
        &req,
        &token_processor,
    )?;
    let username = match ioc.create_session(id_provider).execute(
        CreateSessionDTO {
            username,
            password,
        }
    ).await {
        Ok(data) => data.username,
        Err(err) => return Err(ServerFnError::new(err.content().to_string())),
    };

    let token = token_processor.set_token_session(&username);

    res.insert_header(header::SET_COOKIE, HeaderValue::from_str(
        &Cookie::build("token", token)
            .path("/")
            .http_only(true)
            .finish().to_string()
    ).unwrap());
    leptos_actix::redirect("/");
    Ok(())
}
