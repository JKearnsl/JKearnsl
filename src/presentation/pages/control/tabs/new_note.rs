use leptos::{component, view, IntoView};
use leptos::prelude::*;
use crate::domain::models::note::NoteListItem;
use crate::presentation::api;
use crate::presentation::pages::control::tabs::edit_note::NoteFormFields;
use crate::presentation::components::ui::button::Button;

#[component]
pub fn Section(
    notes: Resource<Vec<NoteListItem>>,
) -> impl IntoView {
    let title       = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let body        = RwSignal::new(String::new());
    let category    = RwSignal::new("prog".to_string());
    let tags        = RwSignal::new(Vec::<String>::new());
    let featured    = RwSignal::new(false);
    let publish     = RwSignal::new(true);

    let success: RwSignal<Option<String>> = RwSignal::new(None);

    let create_action = Action::new(move |_: &()| {
        let title_val       = title.get();
        let description_val = description.get();
        let body_val        = body.get();
        let category_val    = category.get();
        let tags_val        = tags.get().join(", ");
        let featured_val    = featured.get();
        let publish_val     = publish.get();
        async move {
            api::notes::create(
                title_val, description_val, body_val,
                category_val, tags_val, featured_val, publish_val,
            ).await
        }
    });

    let result  = create_action.value();
    let pending = create_action.pending();

    Effect::new(move |_| {
        if let Some(Ok(slug)) = result.get() {
            success.set(Some(slug));
            notes.refetch();
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        success.set(None);
        create_action.dispatch(());
    };

    view! {
        <div class="new-post-wrap">
            {move || success.get().map(|slug| {
                let href = format!("/posts/{}", slug);
                view! {
                    <div class="admin-success">
                        <span class="success-dot"/>
                        <span class="type-mono-lg">"// публикация создана · "
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
                        {move || if pending.get() { "Создание..." } else { "Создать публикацию →" }}
                    </Button>
                </div>
            </form>
        </div>
    }
}
