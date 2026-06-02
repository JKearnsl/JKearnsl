use leptos::prelude::*;

#[component]
pub fn Checkbox(
    value: RwSignal<bool>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            class=format!(
                "accent-terracotta w-[15px] h-[15px] cursor-pointer \
                outline-none transition-[border-color,box-shadow] duration-200 \
                focus:border-terracotta \
                focus:shadow-[0_0_0_3px_color-mix(in_oklab,var(--terracotta)_12%,transparent)] {}",
                class
            )
            required=required
            prop:checked=move || value.get()
            on:change=move |_| value.update(|v| *v = !*v)
        />
    }
}
