use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_meta::MetaTags;
use crate::controller::web::app::App;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ru">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script>
                    r#"(function(){try{var t=localStorage.getItem('jk-theme');if(t)document.documentElement.setAttribute('data-theme',t);}catch(e){}})();"#
                </script>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options.clone()/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}
