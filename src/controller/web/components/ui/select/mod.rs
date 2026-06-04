use leptos::prelude::*;

#[component]
pub fn Select(
    value: RwSignal<String>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let full_class = format!(
        "w-full bg-paper border border-[var(--line)] rounded-[var(--radius-sm)] \
         py-[13px] px-4 font-sans text-[15px] text-ink outline-none cursor-pointer \
         transition-[border-color,box-shadow] duration-200 \
         focus:border-terracotta \
         focus:shadow-[0_0_0_3px_color-mix(in_oklab,var(--terracotta)_12%,transparent)] {}",
        class
    );
    view! {
        <select
            class=full_class
            on:change=move |ev| value.set(event_target_value(&ev))
        >
            {children()}
        </select>
    }
}

#[component]
pub fn SelectOption(
    #[prop(into)] value: String,
    selected: impl Fn() -> bool + Send + Sync + 'static,
    children: Children,
) -> impl IntoView {
    view! {
        <option value=value selected=move || selected()>{children()}</option>
    }
}
