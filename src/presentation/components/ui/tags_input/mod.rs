use leptos::prelude::*;

#[component]
pub fn TagsInput(
    value: RwSignal<Vec<String>>,
    #[prop(optional, into)] placeholder: Option<String>,
) -> impl IntoView {
    let draft = RwSignal::new(String::new());

    let add_tag = move |text: String| {
        let tag = text.trim().to_string();
        if !tag.is_empty() {
            value.update(|v| {
                if !v.contains(&tag) {
                    v.push(tag);
                }
            });
        }
        draft.set(String::new());
    };

    let on_input = move |ev: leptos::ev::Event| {
        let val = event_target_value(&ev);
        if val.ends_with(',') {
            add_tag(val.trim_end_matches(',').to_string());
        } else {
            draft.set(val);
        }
    };

    let on_keydown = move |_ev: leptos::ev::KeyboardEvent| {
        #[cfg(feature = "hydrate")]
        match _ev.key().as_str() {
            "Enter" | "," => {
                _ev.prevent_default();
                add_tag(draft.get_untracked());
            }
            "Backspace" if draft.get_untracked().is_empty() => {
                value.update(|v| { v.pop(); });
            }
            _ => {}
        }
    };

    view! {
        <div class="flex flex-wrap gap-[6px] items-center bg-cream border border-[var(--line)] \
            rounded-[var(--radius-sm)] py-2 px-3 min-h-[50px] cursor-text \
            transition-[border-color,box-shadow] duration-200 \
            focus-within:border-terracotta \
            focus-within:shadow-[0_0_0_3px_color-mix(in_oklab,var(--terracotta)_12%,transparent)]">
            <For
                each=move || value.get()
                key=|tag| tag.clone()
                children=move |tag| {
                    let t = tag.clone();
                    view! {
                        <span class="inline-flex items-center gap-[4px] font-mono text-[11px] \
                            tracking-[.08em] uppercase bg-ink text-cream rounded-full \
                            py-[4px] pl-[10px] pr-[4px] select-none">
                            {tag}
                            <button
                                type="button"
                                class="flex items-center justify-center w-[18px] h-[18px] \
                                    rounded-full border-0 \
                                    bg-[color-mix(in_oklab,var(--cream)_18%,transparent)] \
                                    text-cream text-[13px] leading-none cursor-pointer p-0 \
                                    transition-colors duration-150 \
                                    hover:bg-[color-mix(in_oklab,var(--cream)_35%,transparent)]"
                                on:click=move |_| {
                                    let t = t.clone();
                                    value.update(|v| v.retain(|x| x != &t));
                                }
                            >
                                "×"
                            </button>
                        </span>
                    }
                }
            />
            <input
                type="text"
                class="flex-1 min-w-[120px] bg-transparent border-none outline-none \
                    font-sans text-[15px] text-ink py-[3px] \
                    placeholder:text-muted placeholder:opacity-50"
                placeholder=move || {
                    if value.get().is_empty() {
                        placeholder.clone().unwrap_or_default()
                    } else {
                        String::new()
                    }
                }
                prop:value=move || draft.get()
                on:input=on_input
                on:keydown=on_keydown
            />
        </div>
    }
}
