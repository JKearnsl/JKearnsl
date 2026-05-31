use leptos::prelude::*;

#[component]
pub fn Chip(
    active: impl Fn() -> bool + Send + Sync + 'static,
    on_click: impl Fn(leptos::ev::MouseEvent) + Send + Sync + 'static,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class="chip"
            class:chip-active=active
            on:click=on_click
        >
            {children()}
        </button>
    }
}

/// Non-interactive chip for displaying tags.
#[component]
pub fn Tag(children: Children) -> impl IntoView {
    view! {
        <span class="chip">{children()}</span>
    }
}

/// Row container for chips/tags.
#[component]
pub fn ChipRow(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let full_class = if class.is_empty() {
        "chip-row".to_string()
    } else {
        format!("chip-row {}", class)
    };
    view! {
        <div class=full_class>{children()}</div>
    }
}
