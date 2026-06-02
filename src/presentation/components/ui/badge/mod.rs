use leptos::prelude::*;

#[component]
pub fn Badge(
    #[prop(optional, into)] color: String,
    children: Children,
) -> impl IntoView {
    let style = if color.is_empty() { String::new() } else { format!("background:{}", color) };
    view! {
        <span
            class="inline-flex items-center py-[6px] px-[14px] rounded-full text-cream font-mono text-[11px] tracking-[.08em] uppercase"
            style=style
        >
            {children()}
        </span>
    }
}
