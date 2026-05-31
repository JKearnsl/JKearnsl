use leptos::prelude::*;
use crate::presentation::pages::control::AdminView;
use crate::domain::models::note::{Category, NoteListItem};
use crate::presentation::api;
use crate::presentation::components::notes::category;

#[component]
pub fn Section(
    notes: Resource<Vec<NoteListItem>>,
    view: RwSignal<AdminView>,
) -> impl IntoView {
    let delete_action = Action::new(move |id: &String| {
        let id = id.clone();
        async move { api::notes::delete(id).await }
    });

    let delete_result = delete_action.value();

    Effect::new(move |_| {
        if let Some(Ok(())) = delete_result.get() {
            notes.refetch();
        }
    });

    view! {
        <Suspense fallback=move || view! {
            <div class="admin-loading type-mono">"// загрузка..."</div>
        }>
            {move || notes.get().map(|posts| {
                let total   = posts.len();
                let prog    = posts.iter().filter(|p| p.category == Category::Prog).count();
                let math    = posts.iter().filter(|p| p.category == Category::Math).count();
                let sci     = posts.iter().filter(|p| p.category == Category::Science).count();

                view! {
                    <div class="stats-grid">
                        <StatCard label="Публикаций" value=total.to_string()/>
                        <StatCard label="Программирование" value=prog.to_string()/>
                        <StatCard label="Математика" value=math.to_string()/>
                        <StatCard label="Наука" value=sci.to_string()/>
                    </div>

                    <div class="admin-section">
                        <div class="type-eyebrow admin-section-label">"// все записи"</div>
                        {if posts.is_empty() {
                            view! {
                                <div class="admin-empty type-mono">"// публикаций ещё нет"</div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="admin-table">
                                    {posts.into_iter().map(|post| {
                                        use chrono::Datelike;
                                        use crate::domain::models::note::State;
                                        let href = format!("/posts/{}", post.slug);
                                        let date = {
                                            let d = &post.created_at;
                                            format!("{}/{:02}/{:02}", d.year(), d.month(), d.day())
                                        };
                                        let cat = post.category.clone();
                                        let is_published = post.state == State::Published;
                                        let post_id = post.id.clone();
                                        let post_id_del = post.id.clone();
                                        view! {
                                            <div class="admin-row">
                                                <span class="admin-row-no type-mono muted">
                                                    "№"{post.no}
                                                </span>
                                                <a href=href class="admin-row-title">{post.title}</a>
                                                <span class=move || if is_published {
                                                    "admin-row-state state-published type-mono"
                                                } else {
                                                    "admin-row-state state-draft type-mono"
                                                }>
                                                    {if is_published { "pub" } else { "draft" }}
                                                </span>
                                                <span class="admin-row-cat type-mono">{category::label(&cat)}</span>
                                                <span class="admin-row-date type-mono muted">{date}</span>
                                                <span class="admin-row-actions">
                                                    <button
                                                        class="row-action-btn edit-btn"
                                                        title="Редактировать"
                                                        on:click=move |_| view.set(AdminView::EditPost(post_id.clone()))
                                                    >
                                                        "✏"
                                                    </button>
                                                    <button
                                                        class="row-action-btn delete-btn"
                                                        title="Удалить"
                                                        on:click=move |_| {
                                                            delete_action.dispatch(post_id_del.clone());
                                                        }
                                                    >
                                                        "✕"
                                                    </button>
                                                </span>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }}
                    </div>
                }
            })}
        </Suspense>
    }.into_any()
}

#[component]
fn StatCard(label: &'static str, value: String) -> impl IntoView {
    view! {
        <div class="stat-card card-surface">
            <div class="stat-card-value">{value}</div>
            <div class="type-eyebrow stat-card-label">{label}</div>
        </div>
    }.into_any()
}
