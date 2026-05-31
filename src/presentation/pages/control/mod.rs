mod tabs;

use leptos::prelude::*;
use leptos_meta::{Style, Title};
use leptos_router::hooks::use_navigate;
use crate::presentation::api;

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
        <Style id="page-control">{include_str!("./control.css")}</Style>
        <Style id="ui-card">{include_str!("../../components/ui/card/card.css")}</Style>
        <Title text="Панель управления"/>
        <main class="page admin-page">
            <div class="wrap">
                <Suspense>
                    {move || current_user.get().map(|user| match user {
                        Ok(Some(username)) => view! {
                            <div class="admin-header">
                                <div class="type-eyebrow admin-eyebrow">
                                    <span class="eyebrow-line"/>
                                    "// /control"
                                </div>
                                <div class="admin-title-row">
                                    <h1 class="h-section">"Панель управления"</h1>
                                    <span class="admin-user type-mono">"@"{username}</span>
                                </div>
                            </div>

                            <div class="admin-tabs">
                                <button
                                    class="admin-tab"
                                    class:admin-tab-active=move || view.get() == AdminView::Overview
                                    on:click=move |_| view.set(AdminView::Overview)
                                >
                                    "Обзор"
                                </button>
                                <button
                                    class="admin-tab"
                                    class:admin-tab-active=move || view.get() == AdminView::NewPost
                                    on:click=move |_| view.set(AdminView::NewPost)
                                >
                                    "Новая публикация"
                                </button>
                                {move || match view.get() {
                                    AdminView::EditPost(_) => view! {
                                        <button class="admin-tab admin-tab-active">
                                            "Редактирование"
                                        </button>
                                    }.into_any(),
                                    _ => view! { <span/> }.into_any(),
                                }}
                            </div>

                            <div class="admin-content">
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
                            <div class="admin-loading type-mono">"// проверка авторизации..."</div>
                        }.into_any(),
                    })}
                </Suspense>
            </div>
        </main>
    }.into_any()
}
