use leptos::prelude::*;
use leptos_meta::Style;

#[component]
pub fn Badge(
    #[prop(optional, into)] color: String,
    children: Children,
) -> impl IntoView {
    let style = if color.is_empty() { String::new() } else { format!("background:{}", color) };
    view! {
        <Style id="ui-badge">{include_str!("./badge.css")}</Style>
        <span class="badge" style=style>{children()}</span>
    }
}
