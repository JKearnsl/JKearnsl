mod hero;
mod marquee;
mod featured;
mod filter;
mod news_letter;
mod post;

use leptos::prelude::*;
use leptos_meta::Style;
use crate::presentation::api;

#[component]
pub fn Page() -> impl IntoView {
    let filter = RwSignal::new("all".to_string());

    let notes = Resource::new(
        move || filter.get(),
        |cat| async move {
            let cat_arg = if cat == "all" { None } else { Some(cat) };
            api::notes::list_by_category(cat_arg).await.unwrap_or_default()
        },
    );

    view! {
        <Style id="notes-cover">{include_str!("../../components/notes/cover.css")}</Style>
        <Style id="ui-card">{include_str!("../../components/ui/card/card.css")}</Style>
        <Style id="ui-badge">{include_str!("../../components/ui/badge/badge.css")}</Style>
        <Style id="notes-note-card">{include_str!("../../components/notes/note_card/note_card.css")}</Style>
        <Style id="home-post-grid">{include_str!("./post.css")}</Style>
        <main class="page">
            <hero::Editorial notes=notes.clone()/>
            <marquee::Section/>
            <featured::Section notes=notes.clone()/>
            <filter::Bar filter=filter/>
            <Suspense fallback=move || view! { <div class="loading-posts">"// загрузка..."</div> }>
                {move || notes.get().map(|posts| view! { <post::Grid posts/> })}
            </Suspense>
            <news_letter::Section/>
        </main>
    }
}
