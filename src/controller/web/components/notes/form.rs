use leptos::prelude::*;
use crate::controller::web::components::ui::{
    checkbox::Checkbox,
    form_field::FormField,
    input::Input,
    select::{Select, SelectOption},
    tags_input::TagsInput,
    textarea::Textarea,
};

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
            <Select value=category>
                <SelectOption value="prog" selected=move || category.get() == "prog">"Программирование"</SelectOption>
                <SelectOption value="math" selected=move || category.get() == "math">"Математика"</SelectOption>
                <SelectOption value="science" selected=move || category.get() == "science">"Наука"</SelectOption>
            </Select>
        </FormField>

        <FormField label="Заголовок">
            <Input value=title placeholder="Заголовок публикации..." required=true class="bg-paper"/>
        </FormField>

        <FormField label="Краткое описание">
            <Textarea
                value=description
                mono=false
                attr:rows="3"
                placeholder="Одно-два предложения о публикации..."
            />
        </FormField>

        <BodyEditor body/>

        <FormField label="Теги">
            <TagsInput value=tags placeholder="rust, async, tokio"/>
        </FormField>

        <div class="flex gap-7 flex-wrap">
            <label class="flex items-center gap-[9px] font-mono text-[12px] tracking-[.06em] uppercase text-muted cursor-pointer select-none">
                <Checkbox value=featured/>
                "Отметить как особую"
            </label>
            <label class="flex items-center gap-[9px] font-mono text-[12px] tracking-[.06em] uppercase text-muted cursor-pointer select-none">
                <Checkbox value=publish/>
                "Опубликовать"
            </label>
        </div>
    }.into_any()
}

#[component]
pub fn BodyEditor(body: RwSignal<String>) -> impl IntoView {
    use crate::controller::web::lib::markdown;

    let preview_mode = RwSignal::new(false);

    let preview_html = Memo::new(move |_| {
        if preview_mode.get() {
            markdown::render(&body.get())
        } else {
            String::new()
        }
    });

    view! {
        <div class="flex flex-col">
            <div class="flex items-center justify-between py-[10px] px-[14px] bg-cream-2 border border-[var(--line)] border-b-0 rounded-t-[var(--radius-sm)]">
                <span class="font-mono text-[11px] tracking-[.14em] uppercase text-muted">"Тело публикации"</span>
                <div class="flex gap-1">
                    <button
                        type="button"
                        class="font-mono text-[11px] tracking-[.06em] uppercase py-[5px] px-3 rounded-[6px] text-muted transition-colors hover:text-ink"
                        class:bg-ink=move || !preview_mode.get()
                        class:text-cream=move || !preview_mode.get()
                        on:click=move |_| preview_mode.set(false)
                    >
                        "✏ Писать"
                    </button>
                    <button
                        type="button"
                        class="font-mono text-[11px] tracking-[.06em] uppercase py-[5px] px-3 rounded-[6px] text-muted transition-colors hover:text-ink"
                        class:bg-ink=move || preview_mode.get()
                        class:text-cream=move || preview_mode.get()
                        on:click=move |_| preview_mode.set(true)
                    >
                        "◉ Превью"
                    </button>
                </div>
            </div>

            {move || if preview_mode.get() {
                let html = preview_html.get();
                view! {
                    <div class="min-h-[360px] py-5 px-6 bg-paper border border-[var(--line)] rounded-b-[var(--radius-sm)] prose">
                        {if html.is_empty() {
                            view! { <span class="type-mono muted">"// пусто"</span> }.into_any()
                        } else {
                            view! { <div inner_html=html/> }.into_any()
                        }}
                    </div>
                }.into_any()
            } else {
                view! {
                    <Textarea
                        value=body
                        rounded=false
                        class="min-h-[360px] py-4 px-[18px] rounded-b-[var(--radius-sm)] rounded-t-none"
                        placeholder="# Заголовок\n\nТекст публикации в Markdown..."
                    />
                }.into_any()
            }}
        </div>
    }.into_any()
}
