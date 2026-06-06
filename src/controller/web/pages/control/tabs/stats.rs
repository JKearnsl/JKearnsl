use leptos::prelude::*;
use crate::controller::web::pages::control::AdminView;
use crate::domain::models::note::{Category, NoteListItem};
use crate::controller::web::lib::api;
use crate::controller::web::components::notes::category;

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
            <div class="p-20 text-center text-muted type-mono">"// загрузка..."</div>
        }>
            {move || notes.get().map(|posts| {
                let total   = posts.len();
                let prog    = posts.iter().filter(|p| p.category == Category::Prog).count();
                let math    = posts.iter().filter(|p| p.category == Category::Math).count();
                let sci     = posts.iter().filter(|p| p.category == Category::Science).count();

                view! {
                    <div class="grid grid-cols-4 gap-4 mb-[40px] max-[900px]:grid-cols-2">
                        <StatCard label="Публикаций" value=total.to_string()/>
                        <StatCard label="Программирование" value=prog.to_string()/>
                        <StatCard label="Математика" value=math.to_string()/>
                        <StatCard label="Наука" value=sci.to_string()/>
                    </div>

                    <div class="mt-2">
                        <div class="type-eyebrow mb-5 block">"// все записи"</div>
                        {if posts.is_empty() {
                            view! {
                                <div class="p-[60px] text-center text-muted type-mono">"// публикаций ещё нет"</div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="flex flex-col divide-y divide-[var(--line)]">
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
                                            <div class="grid grid-cols-[48px_1fr_64px_140px_100px_72px] max-[900px]:grid-cols-[40px_1fr_56px_72px] items-center gap-3 py-[13px] px-4 rounded-[var(--radius-sm)] transition-colors duration-150 hover:bg-cream-2">
                                                <span class="shrink-0 type-mono muted">
                                                    "№"{post.no}
                                                </span>
                                                <a
                                                    href=href
                                                    class="font-sans text-[15px] font-medium text-ink whitespace-nowrap overflow-hidden text-ellipsis transition-colors duration-150 hover:text-terracotta"
                                                >
                                                    {post.title}
                                                </a>
                                                <span class=move || if is_published {
                                                    "text-[11px] tracking-[.1em] uppercase text-terracotta type-mono"
                                                } else {
                                                    "text-[11px] tracking-[.1em] uppercase text-muted type-mono"
                                                }>
                                                    {if is_published { "pub" } else { "draft" }}
                                                </span>
                                                <span class="text-muted type-mono max-[900px]:hidden">{category::label(&cat)}</span>
                                                <span class="text-muted type-mono max-[900px]:hidden">{date}</span>
                                                <span class="flex gap-[6px] justify-end">
                                                    <button
                                                        class="w-[30px] h-[30px] rounded-[8px] border border-[var(--line)] inline-flex items-center justify-center text-[13px] transition-[background,border-color,color] duration-150 hover:bg-ink hover:border-ink hover:text-cream"
                                                        title="Редактировать"
                                                        on:click=move |_| view.set(AdminView::EditPost(post_id.clone()))
                                                    >
                                                        "✏"
                                                    </button>
                                                    <button
                                                        class="w-[30px] h-[30px] rounded-[8px] border border-[var(--line)] inline-flex items-center justify-center text-[13px] transition-[background,border-color,color] duration-150 hover:bg-rust hover:border-rust hover:text-cream"
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
        <div class="py-6 px-7 rounded-[var(--radius)] flex flex-col gap-2 bg-paper border border-[var(--line)]">
            <div class="font-display text-[48px] font-semibold tracking-[-0.04em] leading-none text-ink">{value}</div>
            <div class="type-eyebrow mt-[2px]">{label}</div>
        </div>
    }.into_any()
}
