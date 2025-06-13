use leptos::{component, IntoView, view};
use leptos_meta::Title;
use leptos::prelude::ElementChild;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <Title text="contact" />
        <section>
            <h3>Contact</h3>
            <p>
                Welcome to my website, I am glad to see you!
                <br/>Here you can find my notes, projects and contact information
            </p>
        </section>
    }
}
