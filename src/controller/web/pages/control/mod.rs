mod tabs;

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::hooks::use_navigate;
use crate::controller::web::api;

#[derive(Clone, PartialEq)]
pub enum AdminView {
    Overview,
    NewPost,
    EditPost(String),
}

#[component]
pub fn Page() -> impl IntoView {
    let navigate = use_navigate();
    let navigate = StoredValue::new(navigate);
    let view: RwSignal<AdminView> = RwSignal::new(AdminView::Overview);

    let current_user = Resource::new(|| (), |_| api::users::get_self());

    Effect::new(move |_| {
        if let Some(Ok(None)) = current_user.get() {
            navigate.with_value(|nav| nav("/sign-in", Default::default()));
        }
    });

    let notes = Resource::new(|| (), |_| async move {
        api::notes::list_as_admin().await.unwrap_or_default()
    });

    view! {
        <Title text="Панель управления"/>
        <main class="page pt-[56px] pb-[96px]">
            <div class="wrap">
                <Suspense>
                    {move || current_user.get().map(|user| match user {
                        Ok(Some(username)) => view! {
                            <div class="pb-8 border-b border-[var(--line)]">
                                <div class="type-eyebrow flex items-center gap-3 mb-[18px]">
                                    <span class="w-8 h-px bg-terracotta shrink-0"/>
                                    "// control"
                                </div>
                                <div class="flex items-baseline justify-between gap-4 flex-wrap">
                                    <h1 class="h-section">"Панель управления"</h1>
                                    <span class="py-[6px] px-[14px] rounded-full bg-terracotta/10 text-terracotta border border-terracotta/25 type-mono">"@"{username}</span>
                                </div>
                            </div>

                            <div class="flex mt-9 border-b border-[var(--line)]">
                                <button
                                    class="py-[14px] px-7 font-mono text-[12px] tracking-[.08em] uppercase border-b-2 -mb-px transition-colors duration-200 hover:text-ink"
                                    class:text-muted=move || view.get() != AdminView::Overview
                                    class:text-ink=move || view.get() == AdminView::Overview
                                    class:border-transparent=move || view.get() != AdminView::Overview
                                    class:border-terracotta=move || view.get() == AdminView::Overview
                                    on:click=move |_| view.set(AdminView::Overview)
                                >
                                    "Обзор"
                                </button>
                                <button
                                    class="py-[14px] px-7 font-mono text-[12px] tracking-[.08em] uppercase border-b-2 -mb-px transition-colors duration-200 hover:text-ink"
                                    class:text-muted=move || view.get() != AdminView::NewPost
                                    class:text-ink=move || view.get() == AdminView::NewPost
                                    class:border-transparent=move || view.get() != AdminView::NewPost
                                    class:border-terracotta=move || view.get() == AdminView::NewPost
                                    on:click=move |_| view.set(AdminView::NewPost)
                                >
                                    "Новая публикация"
                                </button>
                                {move || match view.get() {
                                    AdminView::EditPost(_) => view! {
                                        <button class="py-[14px] px-7 font-mono text-[12px] tracking-[.08em] uppercase text-ink border-b-2 border-terracotta -mb-px">
                                            "Редактирование"
                                        </button>
                                    }.into_any(),
                                    _ => view! { <span/> }.into_any(),
                                }}
                            </div>

                            <div class="pt-12">
                                {move || match view.get() {
                                    AdminView::Overview => view! {
                                        <tabs::stats::Section notes=notes.clone() view=view/>
                                    }.into_any(),
                                    AdminView::NewPost => view! {
                                        <tabs::new_note::Section notes=notes.clone()/>
                                    }.into_any(),
                                    AdminView::EditPost(id) => view! {
                                        <tabs::edit_note::Section note_id=id notes=notes.clone() view=view/>
                                    }.into_any(),
                                }}
                            </div>
                        }.into_any(),
                        _ => view! {
                            <div class="p-20 text-center text-muted type-mono">"// проверка авторизации..."</div>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </main>
    }.into_any()
}
