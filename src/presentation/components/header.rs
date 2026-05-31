use leptos::prelude::*;
use leptos_meta::Style;
use leptos_router::hooks::use_location;

#[component]
pub fn Header(theme: RwSignal<String>) -> impl IntoView {
    let toggle_theme = move |_| {
        theme.update(|t| {
            *t = if t == "light" { "dark".to_string() } else { "light".to_string() };
        });
    };

    let location = use_location();
    let path = move || location.pathname.get();

    let nav_links = [
        ("/", "архив"),
        ("/music", "музыка"),
        ("/gallery", "галерея"),
        ("/about", "автор"),
    ];

    view! {
        <Style id="comp-header">{include_str!("./header.css")}</Style>
        <header class="site-header">
            <div class="wrap header-inner">
                <a href="/" class="logo-link">
                    <span class="logo-badge">
                        <span class="logo-ring"/>
                        "jk"
                    </span>
                    <span class="logo-text">
                        "JKearnsl "
                        <span class="logo-sub">"// blog"</span>
                    </span>
                </a>

                <nav class="site-nav">
                    {nav_links.into_iter().map(|(href, label)| {
                        let path = path.clone();
                        view! {
                            <a
                                href={href}
                                class="nav-link"
                                class:active=move || path() == href
                            >{label}</a>
                        }
                    }).collect_view()}
                </nav>

                <div class="header-actions">
                    <span class="type-mono search-hint">
                        <span class="kbd">"⌘ K"</span>
                        " поиск"
                    </span>
                    <button class="theme-toggle" on:click=toggle_theme aria-label="toggle theme">
                        <div class="knob"/>
                    </button>
                </div>
            </div>
        </header>
    }
}
