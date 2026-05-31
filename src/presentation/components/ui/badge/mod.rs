use leptos::prelude::*;

#[component]
pub fn Badge(
    #[prop(optional, into)] color: String,
    children: Children,
) -> impl IntoView {
    let style = if color.is_empty() { String::new() } else { format!("background:{}", color) };
    view! {
        <span class="badge" style=style>{children()}</span>
    }
}
