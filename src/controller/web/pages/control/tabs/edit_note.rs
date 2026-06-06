use leptos::prelude::*;
use crate::controller::web::pages::control::AdminView;
use crate::domain::models::note::NoteListItem;
use crate::controller::web::lib::api;
use crate::controller::web::components::{
    notes::form::NoteFormFields,
    ui::button::Button,
};

#[component]
pub fn Section(
    note_id: String,
    notes: Resource<Vec<NoteListItem>>,
    view: RwSignal<AdminView>,
) -> impl IntoView {
    let note_id = StoredValue::new(note_id);

    let note_res = Resource::new(
        move || note_id.get_value(),
        |id| async move { api::notes::get_for_edit(id).await.ok().flatten() },
    );

    view! {
        <Suspense fallback=move || view! {
            <div class="p-20 text-center text-muted type-mono">"// загрузка записи..."</div>
        }>
            {move || note_res.get().map(|maybe_note| match maybe_note {
                None => view! {
                    <div class="p-[60px] text-center text-muted type-mono">"// запись не найдена"</div>
                }.into_any(),
                Some(note) => view! {
                    <EditForm note=note notes=notes.clone() view=view/>
                }.into_any(),
            })}
        </Suspense>
    }.into_any()
}

#[component]
fn EditForm(
    note: crate::domain::models::note::Note,
    notes: Resource<Vec<crate::domain::models::note::NoteListItem>>,
    view: RwSignal<AdminView>,
) -> impl IntoView {
    use crate::domain::models::note::State;

    let note_id      = StoredValue::new(note.id.clone());
    let title        = RwSignal::new(note.title.clone());
    let description  = RwSignal::new(note.description.clone());
    let body         = RwSignal::new(note.body.clone());
    let category     = RwSignal::new(note.category.as_str().to_string());
    let tags         = RwSignal::new(note.tags.clone());
    let featured     = RwSignal::new(note.featured);
    let publish      = RwSignal::new(note.state == State::Published);

    let success: RwSignal<Option<String>> = RwSignal::new(None);

    let update_action = Action::new(move |_: &()| {
        let id_val          = note_id.get_value();
        let title_val       = title.get();
        let description_val = description.get();
        let body_val        = body.get();
        let category_val    = category.get();
        let tags_val        = tags.get().join(", ");
        let featured_val    = featured.get();
        let publish_val     = publish.get();
        async move {
            api::notes::update(
                id_val, title_val, description_val, body_val,
                category_val, tags_val, featured_val, publish_val,
            ).await
        }
    });

    let result  = update_action.value();
    let pending = update_action.pending();

    Effect::new(move |_| {
        if let Some(Ok(slug)) = result.get() {
            success.set(Some(slug));
            notes.refetch();
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        success.set(None);
        update_action.dispatch(());
    };

    view! {
        <div class="flex flex-col gap-6 max-w-[860px]">
            <div class="mb-1">
                <button
                    class="font-mono text-[12px] tracking-[.06em] uppercase text-muted transition-colors hover:text-ink type-mono"
                    on:click=move |_| view.set(AdminView::Overview)
                >
                    "← назад к списку"
                </button>
            </div>

            {move || success.get().map(|slug| {
                let href = format!("/posts/{}", slug);
                view! {
                    <div class="flex items-center gap-3 py-[13px] px-[18px] rounded-[var(--radius-sm)] bg-terracotta/10 border border-terracotta/25">
                        <span class="size-2 rounded-full bg-terracotta shrink-0"/>
                        <span class="type-mono-lg">"// сохранено · "
                            <a href=href class="text-terracotta underline decoration-dotted underline-offset-[2px] transition-colors hover:text-ochre">"открыть →"</a>
                        </span>
                    </div>
                }
            })}

            <form class="flex flex-col gap-5" on:submit=on_submit>
                <NoteFormFields
                    title=title description=description body=body
                    category=category tags=tags
                    featured=featured publish=publish
                />

                {move || result.get().map(|r| match r {
                    Err(e) => view! {
                        <p class="font-mono text-[12px] text-rust py-[10px] px-[14px] bg-rust/8 rounded-[var(--radius-sm)] border-l-2 border-rust">{e.to_string()}</p>
                    }.into_any(),
                    Ok(_) => view! { <span/> }.into_any(),
                })}

                <div class="flex justify-end pt-2">
                    <Button submit=true pending=pending>
                        {move || if pending.get() { "Сохранение..." } else { "Сохранить изменения →" }}
                    </Button>
                </div>
            </form>
        </div>
    }.into_any()
}

