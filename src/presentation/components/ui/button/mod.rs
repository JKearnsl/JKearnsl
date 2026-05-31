use leptos::prelude::*;
use leptos_meta::Style;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Variant {
    #[default]
    Accent,
    Ghost,
}

impl Variant {
    fn as_str(self) -> &'static str {
        match self {
            Variant::Accent => "btn accent",
            Variant::Ghost => "btn ghost",
        }
    }
}

/// Renders as `<a>` when `href` is provided, otherwise as `<button>`.
#[component]
pub fn Button(
    #[prop(default = Variant::Accent)] variant: Variant,
    #[prop(optional, into)] class: String,
    // --- <button> props ---
    #[prop(default = false)] submit: bool,
    #[prop(optional, into)] pending: Signal<bool>,
    // --- <a> props ---
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] target: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base = variant.as_str();
    let full_class = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    if let Some(href) = href {
        view! {
            <Style id="ui-button">{include_str!("./button.css")}</Style>
            <a href=href target=target class=full_class>
                {children()}
            </a>
        }
        .into_any()
    } else {
        let btn_type = if submit { "submit" } else { "button" };
        view! {
            <Style id="ui-button">{include_str!("./button.css")}</Style>
            <button
                type=btn_type
                class=full_class
                prop:disabled=move || pending.get()
            >
                {children()}
            </button>
        }
        .into_any()
    }
}
