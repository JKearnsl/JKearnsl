use leptos::{component, view, IntoView};
use leptos::prelude::*;
use crate::domain::models::note::NoteListItem;
use crate::controller::web::lib::api;
use crate::controller::web::components::{
    notes::form::NoteFormFields,
    ui::button::Button,
};

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
        <div class="flex flex-col gap-6 max-w-[860px]">
            {move || success.get().map(|slug| {
                let href = format!("/posts/{}", slug);
                view! {
                    <div class="flex items-center gap-3 py-[13px] px-[18px] rounded-[var(--radius-sm)] bg-terracotta/10 border border-terracotta/25">
                        <span class="size-2 rounded-full bg-terracotta shrink-0"/>
                        <span class="type-mono-lg">"// публикация создана · "
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
                        {move || if pending.get() { "Создание..." } else { "Создать публикацию →" }}
                    </Button>
                </div>
            </form>
        </div>
    }.into_any()
}
