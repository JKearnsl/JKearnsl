use leptos::prelude::*;
use leptos_meta::Style;

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
        <Style id="ui-tags-input">{include_str!("./tags_input.css")}</Style>
        <div class="tags-input">
            <For
                each=move || value.get()
                key=|tag| tag.clone()
                children=move |tag| {
                    let t = tag.clone();
                    view! {
                        <span class="tags-input-tag">
                            {tag}
                            <button
                                type="button"
                                class="tags-input-remove"
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
                class="tags-input-field"
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
