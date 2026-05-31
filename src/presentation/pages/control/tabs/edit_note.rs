use leptos::prelude::*;
use crate::presentation::pages::control::AdminView;
use crate::domain::models::note::NoteListItem;
use crate::presentation::api;
use crate::presentation::components::ui::button::Button;
use crate::presentation::components::ui::form_field::FormField;
use crate::presentation::components::ui::input::Input;
use crate::presentation::components::ui::tags_input::TagsInput;

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
            <div class="admin-loading type-mono">"// загрузка записи..."</div>
        }>
            {move || note_res.get().map(|maybe_note| match maybe_note {
                None => view! {
                    <div class="admin-empty type-mono">"// запись не найдена"</div>
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
        <div class="new-post-wrap">
            <div class="edit-back-row">
                <button
                    class="back-btn type-mono"
                    on:click=move |_| view.set(AdminView::Overview)
                >
                    "← назад к списку"
                </button>
            </div>

            {move || success.get().map(|slug| {
                let href = format!("/posts/{}", slug);
                view! {
                    <div class="admin-success">
                        <span class="success-dot"/>
                        <span class="type-mono-lg">"// сохранено · "
                            <a href=href class="footer-hash-link">"открыть →"</a>
                        </span>
                    </div>
                }
            })}

            <form class="new-post-form" on:submit=on_submit>
                <NoteFormFields
                    title=title description=description body=body
                    category=category tags=tags
                    featured=featured publish=publish
                />

                {move || result.get().map(|r| match r {
                    Err(e) => view! {
                        <p class="form-error">{e.to_string()}</p>
                    }.into_any(),
                    Ok(_) => view! { <span/> }.into_any(),
                })}

                <div class="form-actions">
                    <Button submit=true pending=pending>
                        {move || if pending.get() { "Сохранение..." } else { "Сохранить изменения →" }}
                    </Button>
                </div>
            </form>
        </div>
    }.into_any()
}

#[component]
pub fn NoteFormFields(
    title: RwSignal<String>,
    description: RwSignal<String>,
    body: RwSignal<String>,
    category: RwSignal<String>,
    tags: RwSignal<Vec<String>>,
    featured: RwSignal<bool>,
    publish: RwSignal<bool>,
) -> impl IntoView {

    view! {
        <FormField label="Категория">
            <select
                on:change=move |ev| category.set(event_target_value(&ev))
            >
                <option value="prog" selected=move || category.get() == "prog">"Программирование"</option>
                <option value="math" selected=move || category.get() == "math">"Математика"</option>
                <option value="science" selected=move || category.get() == "science">"Наука"</option>
            </select>
        </FormField>

        <FormField label="Заголовок">
            <Input value=title placeholder="Заголовок публикации..." required=true/>
        </FormField>

        <FormField label="Краткое описание">
            <textarea
                rows="3"
                placeholder="Одно-два предложения о публикации..."
                prop:value=move || description.get()
                on:input=move |ev| description.set(event_target_value(&ev))
            />
        </FormField>

        <BodyEditor body/>

        <FormField label="Теги">
            <TagsInput value=tags placeholder="rust, async, tokio"/>
        </FormField>

        <div class="form-checks">
            <label class="check-label">
                <input
                    type="checkbox"
                    prop:checked=move || featured.get()
                    on:change=move |_| featured.update(|v| *v = !*v)
                />
                "Отметить как особую"
            </label>
            <label class="check-label">
                <input
                    type="checkbox"
                    prop:checked=move || publish.get()
                    on:change=move |_| publish.update(|v| *v = !*v)
                />
                "Опубликовать"
            </label>
        </div>
    }.into_any()
}

#[component]
fn BodyEditor(body: RwSignal<String>) -> impl IntoView {
    let preview_mode = RwSignal::new(false);

    let preview_action = Action::new(move |src: &String| {
        let src = src.clone();
        async move { api::notes::preview_markdown(src).await }
    });

    let preview_html    = preview_action.value();
    let preview_pending = preview_action.pending();

    let switch_to_preview = move |_: leptos::ev::MouseEvent| {
        preview_action.dispatch(body.get_untracked());
        preview_mode.set(true);
    };
    let switch_to_write = move |_: leptos::ev::MouseEvent| {
        preview_mode.set(false);
    };

    view! {
        <div class="form-field body-editor">
            <div class="editor-header">
                <span class="editor-label">"Тело публикации"</span>
                <div class="editor-tabs">
                    <button
                        type="button"
                        class="editor-tab"
                        class:editor-tab-active=move || !preview_mode.get()
                        on:click=switch_to_write
                    >
                        "✏ Писать"
                    </button>
                    <button
                        type="button"
                        class="editor-tab"
                        class:editor-tab-active=move || preview_mode.get()
                        on:click=switch_to_preview
                    >
                        "◉ Превью"
                    </button>
                </div>
            </div>

            {move || if preview_mode.get() {
                view! {
                    <div class="editor-preview-pane prose">
                        {move || if preview_pending.get() {
                            view! {
                                <span class="type-mono muted">"// рендеринг..."</span>
                            }.into_any()
                        } else {
                            match preview_html.get() {
                                Some(Ok(html)) if !html.is_empty() => view! {
                                    <div inner_html=html/>
                                }.into_any(),
                                Some(Ok(_)) => view! {
                                    <span class="type-mono muted">"// пусто"</span>
                                }.into_any(),
                                Some(Err(e)) => view! {
                                    <span class="type-mono muted">"// ошибка: "{e.to_string()}</span>
                                }.into_any(),
                                None => view! {
                                    <span class="type-mono muted">"// пусто"</span>
                                }.into_any(),
                            }
                        }}
                    </div>
                }.into_any()
            } else {
                view! {
                    <textarea
                        class="body-textarea"
                        placeholder="# Заголовок\n\nТекст публикации в Markdown..."
                        prop:value=move || body.get()
                        on:input=move |ev| body.set(event_target_value(&ev))
                    />
                }.into_any()
            }}
        </div>
    }.into_any()
}
