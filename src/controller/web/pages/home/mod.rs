mod hero;
mod marquee;
mod featured;
mod filter;
mod news_letter;
mod post;

use leptos::prelude::*;
use crate::controller::web::lib::api;

#[component]
pub fn Page() -> impl IntoView {
    let filter = RwSignal::new("all".to_string());

    let notes = Resource::new(
        move || filter.get(),
        |cat| async move {
            use crate::domain::models::note::Category;
            let cat_arg = if cat == "all" { None } else { Category::try_from(cat.as_str()).ok() };
            api::notes::list(cat_arg, 200, 0).await.unwrap_or_default()
        },
    );

    view! {
        <main class="page">
            <hero::Editorial notes=notes.clone()/>
            <marquee::Section/>
            <featured::Section notes=notes.clone()/>
            <filter::Bar filter=filter/>
            <Suspense fallback=move || view! { <div class="p-12 text-center text-muted type-mono">"// загрузка..."</div> }>
                {move || notes.get().map(|posts| view! { <post::Grid posts/> })}
            </Suspense>
            <news_letter::Section/>
        </main>
    }
}
