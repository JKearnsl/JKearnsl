use leptos::prelude::*;

#[component]
pub fn Input(
    value: RwSignal<String>,
    #[prop(default = "text")] r#type: &'static str,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match r#type {
        _ => "w-full bg-cream border border-[var(--line)] rounded-[var(--radius-sm)] \
              py-[13px] px-4 font-sans text-[15px] text-ink \
              placeholder:text-muted placeholder:opacity-60",
    };

    view! {
        <input
            type=r#type
            class=format!(
                "outline-none transition-[border-color,box-shadow] duration-200 \
                focus:border-terracotta \
                focus:shadow-[0_0_0_3px_color-mix(in_oklab,var(--terracotta)_12%,transparent)] \
                {} {}",
                variant_class, class
            )
            placeholder=placeholder
            required=required
            prop:value=move || value.get()
            on:input=move |ev| value.set(event_target_value(&ev))
        />
    }
}
