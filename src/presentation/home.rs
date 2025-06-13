use leptos::{component, IntoView, view};
use leptos_meta::Title;
use leptos::prelude::{use_context, ElementChild, Get, GlobalOnAttributes, InnerHtmlAttribute, ReadSignal, RenderHtml, Set, Update, WriteSignal};
use crate::presentation::app::NavBar;
use crate::presentation::logged_in::{is_logged_in, set_logged_in};

#[component]
pub fn HomePage() -> impl IntoView {
    
    view! {
        <Title text="home" />
        <main>
            <section>
                <h3>About</h3>
                <p>
                    Welcome to my website, I am glad to see you!
                    <br/>Here you can find my notes, projects and contact information
                </p>
        <div>
  <blockquote cite="https://www.huxley.net/bnw/four.html">
    <p>Words can be like X-rays, if you use them properly theyll go through anything. You read and youre pierced.</p>
  </blockquote>
  <p>Aldous Huxley, <cite>Brave New World</cite></p>
</div>

                // <button onclick=move || { set_logged_in(!is_logged_in)}>
                //     {move || if is_logged_in { "Log Out" } else { "Log In" }}
                // </button>
            </section>
            <section>
                <h3>Notes</h3>
                <div class="notes"></div>
            </section>
        </main>
    }
}