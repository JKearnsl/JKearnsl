use leptos::prelude::*;
use leptos_meta::Style;

#[component]
pub fn Card(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] href: Option<String>,
    children: Children,
) -> impl IntoView {
    let full_class = if class.is_empty() { "card".to_string() } else { format!("card {}", class) };
    if let Some(href) = href {
        view! {
            <Style id="ui-card">{include_str!("./card.css")}</Style>
            <a href=href class=full_class>{children()}</a>
        }.into_any()
    } else {
        view! {
            <Style id="ui-card">{include_str!("./card.css")}</Style>
            <div class=full_class>{children()}</div>
        }.into_any()
    }
}
