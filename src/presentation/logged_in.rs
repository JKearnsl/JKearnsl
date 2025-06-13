use leptos::prelude::{use_context, Get, RwSignal, Set};

#[derive(Copy, Clone)]
pub struct LoggedInContext(pub RwSignal<bool>);

pub fn is_logged_in() -> bool {
    let signal = use_context::<LoggedInContext>().unwrap().0;
    signal.get()
}

pub fn set_logged_in(value: bool) {
    let signal = use_context::<LoggedInContext>().unwrap().0;
    signal.set(value);
}
