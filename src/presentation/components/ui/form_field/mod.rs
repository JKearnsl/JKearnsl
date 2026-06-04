use leptos::prelude::*;

#[component]
pub fn FormField(
    #[prop(into)] label: String,
    #[prop(optional, into)] label_for: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2">
            <label
                class="font-mono text-[11px] tracking-[.16em] uppercase text-muted select-none"
                for=label_for
            >
                {label}
            </label>
            {children()}
        </div>
    }
}
