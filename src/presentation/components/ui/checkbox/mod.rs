use leptos::prelude::*;

#[component]
pub fn Input(
    value: RwSignal<bool>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <input
            type="checkbox"
            class=format!(
                "outline-none transition-[border-color,box-shadow] duration-200 focus:border-terracotta \
                focus:shadow-[0_0_0_3px_color-mix(in_oklab,var(--terracotta)_12%,transparent)] \
                accent-terracotta w-[15px] h-[15px] cursor-pointer {}", class
            )
            required=required
            prop:value=move || value.get()
            on:input=move |_ev| value.update(|v| *v = !*v)
        />
    }
}
