use leptos::prelude::*;

#[component]
pub fn FormField(
    #[prop(into)] label: String,
    #[prop(optional, into)] label_for: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="form-field">
            <label for=label_for>{label}</label>
            {children()}
        </div>
    }
}
