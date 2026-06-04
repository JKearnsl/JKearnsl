use leptos::prelude::*;

const BASE: &str = "inline-flex items-center gap-[6px] py-2 px-[14px] rounded-full \
    font-mono text-[11px] tracking-[.08em] uppercase border \
    transition-all duration-200 cursor-pointer";

#[component]
pub fn Chip(
    active: impl Fn() -> bool + Send + Sync + 'static,
    on_click: impl Fn(leptos::ev::MouseEvent) + Send + Sync + 'static,
    children: Children,
) -> impl IntoView {
    let cls = move || {
        if active() {
            format!("{BASE} bg-ink text-cream border-ink")
        } else {
            format!("{BASE} bg-transparent text-ink border-[var(--line)] hover:border-ink")
        }
    };
    view! {
        <button class=cls on:click=on_click>
            {children()}
        </button>
    }
}

#[component]
pub fn Tag(children: Children) -> impl IntoView {
    view! {
        <span class=format!("{BASE} bg-transparent text-ink border-[var(--line)]")>
            {children()}
        </span>
    }
}

#[component]
pub fn ChipRow(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let full_class = if class.is_empty() {
        "flex gap-2 flex-wrap".to_string()
    } else {
        format!("flex gap-2 flex-wrap {}", class)
    };
    view! {
        <div class=full_class>{children()}</div>
    }
}
