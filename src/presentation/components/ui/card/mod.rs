use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] href: Option<String>,
    children: Children,
) -> impl IntoView {
    let full_class = if class.is_empty() { "card".to_string() } else { format!("card {}", class) };
    if let Some(href) = href {
        view! {
            <a href=href class=full_class>{children()}</a>
        }.into_any()
    } else {
        view! {
            <div class=full_class>{children()}</div>
        }.into_any()
    }
}
