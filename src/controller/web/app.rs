use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    ParamSegment, StaticSegment, WildcardSegment,
};

use super::store::session::SessionStore;
use super::components::{header, footer, wipe_overlay};
use super::pages::{
    about,
    control,
    gallery,
    home,
    music,
    note,
    not_found,
    sign_in,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    provide_context(SessionStore::new());

    let theme = RwSignal::new("light".to_string());
    provide_context(theme);

    Effect::new(move |_| {
        let t = theme.get();
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(doc) = window.document() {
                    doc.document_element()
                        .and_then(|el| el.set_attribute("data-theme", &t).ok());
                    if let Some(storage) = window.local_storage().ok().flatten() {
                        storage.set_item("jk-theme", &t).ok();
                    }
                }
            }
        }
        let _ = t;
    });

    Effect::new(move |_| {
        #[cfg(feature = "hydrate")]
        {
            if let Some(window) = web_sys::window() {
                if let Some(storage) = window.local_storage().ok().flatten() {
                    if let Ok(Some(stored)) = storage.get_item("jk-theme") {
                        theme.set(stored);
                    }
                }
            }
        }
    });

    view! {
        <Title formatter=|t| format!("{t} — JKearnsl")/>
        <Meta name="description" content="Заметки о коде, числах и материи"/>
        <Link rel="icon" href="/assets/images/favicon.svg"/>
        <Stylesheet href="/pkg/jkearnsl.css"/>
        <Router>
            <wipe_overlay::WipeOverlay/>
            <header::Header theme/>
            <Routes fallback=not_found::Page>
                <Route path=StaticSegment("") view=home::Page/>
                <Route path=(StaticSegment("posts"), ParamSegment("slug")) view=note::Page/>
                <Route path=StaticSegment("about") view=about::Page/>
                <Route path=StaticSegment("music") view=music::Page/>
                <Route path=StaticSegment("gallery") view=gallery::Page/>
                <Route path=StaticSegment("sign-in") view=sign_in::Page/>
                <Route path=StaticSegment("control") view=control::Page/>
                <Route path=WildcardSegment("any") view=not_found::Page/>
            </Routes>
            <footer::Footer/>
        </Router>
    }
}
