use leptos::prelude::*;
use leptos_meta::Style;

#[component]
pub fn FormField(
    #[prop(into)] label: String,
    #[prop(optional, into)] label_for: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <Style id="ui-form-field">{include_str!("./form_field.css")}</Style>
        <div class="form-field">
            <label for=label_for>{label}</label>
            {children()}
        </div>
    }
}
