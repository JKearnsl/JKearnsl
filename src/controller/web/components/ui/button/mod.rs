use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Variant {
    #[default]
    Accent,
    Ghost,
}

impl Variant {
    fn classes(self) -> &'static str {
        match self {
            Variant::Accent => {
                "inline-flex items-center gap-[10px] py-[14px] px-[22px] rounded-full \
                font-mono text-[12px] tracking-[.08em] uppercase \
                bg-terracotta text-cream border border-terracotta \
                [transition:transform_.2s_ease,background_.2s_ease] hover:-translate-y-[2px]"
            }
            Variant::Ghost => {
                "inline-flex items-center gap-[10px] py-[14px] px-[22px] rounded-full \
                font-mono text-[12px] tracking-[.08em] uppercase \
                bg-transparent text-ink border border-ink \
                [transition:transform_.2s_ease,background_.2s_ease,color_.2s_ease] \
                hover:-translate-y-[2px] hover:bg-ink hover:text-cream"
            }
        }
    }
}

#[component]
pub fn Button(
    #[prop(default = Variant::Accent)] variant: Variant,
    #[prop(optional, into)] class: String,
    #[prop(default = false)] submit: bool,
    #[prop(optional, into)] pending: Signal<bool>,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] target: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    let base = variant.classes();
    let full_class = if class.is_empty() {
        base.to_string()
    } else {
        format!("{} {}", base, class)
    };

    if let Some(href) = href {
        view! {
            <a href=href target=target class=full_class>
                {children()}
            </a>
        }
        .into_any()
    } else {
        let btn_type = if submit { "submit" } else { "button" };
        view! {
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
