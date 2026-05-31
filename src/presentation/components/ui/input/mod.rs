use leptos::prelude::*;

#[component]
pub fn Input(
    value: RwSignal<String>,
    #[prop(default = "text")] r#type: &'static str,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    view! {
        <input
            type=r#type
            class=class
            placeholder=placeholder
            required=required
            prop:value=move || value.get()
            on:input=move |ev| value.set(event_target_value(&ev))
        />
    }
}
