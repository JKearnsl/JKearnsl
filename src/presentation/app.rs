use chrono::Datelike;
use crate::presentation::contact::ContactPage;
use crate::presentation::home::HomePage;
use crate::presentation::not_found::NotFound;
use crate::presentation::projects::ProjectsPage;
use leptos::prelude::ElementChild;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use crate::presentation::logged_in::{is_logged_in, set_logged_in, LoggedInContext};
use crate::presentation::sign_in::SignIn;
// use crate::presentation::nav_panel::logged_in;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let year = chrono::Utc::now().year();

    let logged_in = RwSignal::new(false);
    provide_context(LoggedInContext(logged_in));
    
    let current_username_resource: Resource<Result<String, ServerFnError>> = Resource::new(
        || (), move |_| current_user(),
    );
    
    let username_view = move || Suspend::new(async move {
        match current_username_resource.await {
            Ok(username) => Some({
                set_logged_in(true);
                view! { <p>Welcome, <b>{username}</b></p> }
            }),
            Err(_) => None
        }
    });
    

    view! {
        <Title formatter=|text| format!("{text} — JKearnsl")/>
        <Link rel="icon" href="assets/images/favicon.svg"/>
        <Stylesheet href="assets/css/normalize.css"/>
        <Stylesheet href="assets/css/sakura.css"/>
        <Stylesheet href="assets/css/style.css"/>

        <header>
            <img src="assets/images/logo/overlord.webp" draggable="false" alt="Logo: overlord" loading="eager"/>
            { username_view() }
        </header>

        <NavBar/>

        <Router>
            <main>
                <Routes fallback=NotFound>
                    <Route path=StaticSegment("/") view=HomePage/>
                    <Route path=StaticSegment("/projects") view=ProjectsPage/>
                    <Route path=StaticSegment("/contact") view=ContactPage/>
                    <Route path=StaticSegment("/login") view=SignIn/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>

        <hr/>
        <footer>
            <p>(c) {year} JKearnsl</p>
        </footer>
        
    }
}

#[component]
pub fn NavBar() -> impl IntoView {
    let logged_in = expect_context::<LoggedInContext>().0;
    view! {
        <nav>
            <hr/>
                <ul>
                    <li><a href="/">Home</a></li>
                    <li><a href="/projects">Projects</a></li>
                    <li><a href="/contact">Contact</a></li>
                    <Show when=move || logged_in.get()>
                        <li><a href="/logout">Logout</a></li>
                    </Show>
                </ul>
            <hr/>
        </nav>
    }
}

#[server]
pub async fn current_user() -> Result<String, ServerFnError> {
    use actix_web::web::Data;
    use leptos_actix::extract;
    use crate::interactor_factory::InteractorFactory;
    use crate::application::common::interactor::Interactor;
    use crate::adapters::auth::token::TokenProcessor;
    use super::id_provider::make_token_provider;


    let (ioc, token_processor): (Data<dyn InteractorFactory>, Data<TokenProcessor>) = extract().await?;
    let req = expect_context::<leptos_actix::Request>();

    let id_provider = make_token_provider(
        &req,
        &token_processor,
    )?;
    match ioc.get_user_self(id_provider).execute(()).await {
        Ok(data) => Ok(data.username),
        Err(err) => Err(ServerFnError::new(format!("Failed to get user self {:?}", err))),
    }
}
