use leptos::prelude::{ChildrenFn, Show, StoredValue, Suspense, WithValue};
use leptos::{component, view, IntoView};
use leptos::prelude::ElementChild;

// #[component]
// pub fn NavPanel() -> impl IntoView {
//     view! {
//         <hr/>
//             <nav>
//                 <ul>
//                     <li><a href="/">Home</a></li>
//                     <li><a href="/projects">Projects</a></li>
//                     <li><a href="/contact">Contact</a></li>
//                     { logout_button_view() }
//                 </ul>
//             </nav>
//         <hr/>
//         <main>
//             {child}
//         </main>
//     }
// }

pub fn logged_in<I, O>(children: I) -> O
where
    I: Fn() + IntoView + 'static,
    O: FnOnce() -> dyn IntoView + 'static,
{
    move || {
        view! {
            <hr/>
            <h1>"Logged in check..."</h1>
            <hr/>
            {children()}
   
        }
    }
}