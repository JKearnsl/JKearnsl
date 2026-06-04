use leptos::prelude::*;

#[component]
pub fn Textarea(
    value: RwSignal<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(default = true)] mono: bool,
    #[prop(default = true)] rounded: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let font = mono
        .then_some("font-mono text-[14px] leading-[1.65]")
        .unwrap_or("font-sans text-[15px]");
    let radius = rounded
        .then_some("rounded-[var(--radius-sm)]")
        .unwrap_or_default();
    let full_class = format!(
        "w-full resize-y bg-paper border border-[var(--line)] {radius} py-3 px-4 \
         {font} text-ink outline-none transition-[border-color,box-shadow] duration-200 \
         focus:border-terracotta focus:shadow-[0_0_0_3px_color-mix(in_oklab,var(--terracotta)_12%,transparent)] \
         placeholder:text-muted placeholder:opacity-60 {class}"
    );
    view! {
        <textarea
            class=full_class
            placeholder=placeholder
            prop:value=move || value.get()
            on:input=move |ev| value.set(event_target_value(&ev))
        />
    }
}
