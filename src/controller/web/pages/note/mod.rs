mod content;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use crate::controller::web::api::notes::by_slug;


#[component]
pub fn Page() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());

    let note = Resource::new(slug, |s| async move { by_slug(s).await.unwrap_or(None) });

    view! {
        <main class="page">
            <Suspense fallback=move || view! { <div class="p-[120px] text-center text-muted type-mono">"// загрузка..."</div> }>
                {move || note.get().map(|opt| match opt {
                    Some(post) => view! { <content::NoteContent post/> }.into_any(),
                    None => view! { <div class="p-[120px] text-center text-muted type-mono">"// запись не найдена"</div> }.into_any(),
                })}
            </Suspense>
        </main>
    }
}
