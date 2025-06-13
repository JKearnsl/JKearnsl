use leptos::{component, IntoView, view};
use leptos_meta::Title;
use leptos::prelude::ElementChild;
use crate::presentation::app::NavBar;

#[component]
pub fn ProjectsPage() -> impl IntoView {
    view! {
        <Title text="projects" />
        
        <section>
            <h3>Projects</h3>
        </section>
    }
}
