mod content;

use leptos::prelude::*;
use leptos_meta::Style;
use leptos_router::hooks::use_params_map;
use crate::presentation::api::notes::by_slug;


#[component]
pub fn Page() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());

    let note = Resource::new(slug, |s| async move { by_slug(s).await.unwrap_or(None) });

    view! {
        <Style id="notes-note-content">{include_str!("../../components/notes/note_content/note_content.css")}</Style>
        <Style id="notes-cover">{include_str!("../../components/notes/cover.css")}</Style>
        <Style id="ui-badge">{include_str!("../../components/ui/badge/badge.css")}</Style>
        <Style id="ui-chip">{include_str!("../../components/ui/chip/chip.css")}</Style>
        <main class="page">
            <Suspense fallback=move || view! { <div class="note-loading type-mono">"// загрузка..."</div> }>
                {move || note.get().map(|opt| match opt {
                    Some(post) => view! { <content::NoteContent post/> }.into_any(),
                    None => view! { <div class="note-not-found type-mono">"// запись не найдена"</div> }.into_any(),
                })}
            </Suspense>
        </main>
    }
}
