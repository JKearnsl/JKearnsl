use leptos::prelude::*;

const BASE: &str = "relative [transition:transform_.35s_cubic-bezier(.2,.7,.2,1)] cursor-pointer hover:-translate-y-[6px]";

#[component]
pub fn Card(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] href: Option<String>,
    children: Children,
) -> impl IntoView {
    let full_class = if class.is_empty() {
        BASE.to_string()
    } else {
        format!("{BASE} {}", class)
    };
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
