use leptos::prelude::*;
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
        <header class="sticky top-0 z-40 bg-cream/88 backdrop-blur-[14px] border-b border-[var(--line)]">
            <div class="wrap flex justify-between items-center py-[18px]">
                <a href="/" class="flex items-center gap-3">
                    <span class="size-[34px] rounded-full bg-terracotta text-cream inline-flex items-center justify-center font-mono text-[14px] font-bold relative shrink-0">
                        <span class="absolute inset-[-4px] rounded-full border border-ink rotate-45 pointer-events-none"/>
                        "jk"
                    </span>
                    <span class="font-display text-[18px] font-semibold tracking-[-0.01em]">
                        "JKearnsl "
                        <span class="font-mono font-normal text-muted text-[12px] ml-[6px]">"// blog"</span>
                    </span>
                </a>

                <nav class="flex items-center gap-[6px]">
                    {nav_links.into_iter().map(|(href, label)| {
                        let path = path.clone();
                        view! {
                            <a
                                href={href}
                                class="py-[10px] px-[16px] rounded-full font-mono text-[12px] tracking-[.06em] uppercase transition-colors duration-200 hover:bg-ink/10"
                                class:bg-ink=move || path() == href
                                class:text-cream=move || path() == href
                            >{label}</a>
                        }
                    }).collect_view()}
                </nav>

                <div class="flex items-center gap-[14px]">
                    <span class="type-mono text-muted flex items-center gap-[7px]">
                        <span class="font-mono text-[11px] py-[2px] px-[6px] border border-[var(--line)] rounded-[5px] bg-paper">"⌘ K"</span>
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
