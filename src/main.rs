mod config;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() {
    use std::fs::File;
    use std::io::BufReader;
    use std::net::TcpListener;
    use std::sync::Arc;
    use actix_files::Files;
    use actix_web::{App as ActixApp, HttpServer, web};
    use actix_web::middleware::{Compress, NormalizePath};
    use actix_web::middleware::Logger;
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use jkearnsl::adapters;

    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;

    use jkearnsl::presentation::app::App;

    use adapters::database::initial::initial_models;
    use adapters::database::user_verifier::SqliteUserVerifier;
    use adapters::auth::token::TokenProcessor;
    use jkearnsl::ioc::IoC;

    let config = config::Config::from_env();
    pretty_env_logger::init_custom_env("LOG_LEVEL");

    let connect_options = SqliteConnectOptions::from_str("sqlite://database?mode=rwc")
        .unwrap_or_else(|e| {
            log::error!("Invalid database URL: {}", e);
            std::process::exit(1);
        })
        .foreign_keys(true);

    let db_pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(connect_options)
        .await
        .unwrap_or_else(|e| {
            log::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        });

    initial_models(&db_pool).await.unwrap_or_else(|e| {
        log::error!("Failed to initialize models: {}", e);
        std::process::exit(1);
    });

    let user_verifier = web::Data::new(SqliteUserVerifier::new(db_pool.clone()));
    let token_processor = web::Data::new(TokenProcessor::new());
    let ioc = Arc::new(IoC::new(db_pool));
    let conf = get_configuration(None).unwrap();
    let site_addr = conf.leptos_options.site_addr;

    let app_builder = move || {
        let ioc_arc: Arc<dyn jkearnsl::interactor_factory::InteractorFactory> = ioc.clone();
        let ioc_data: web::Data<dyn jkearnsl::interactor_factory::InteractorFactory> = web::Data::from(ioc_arc);

        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.as_ref();

        ActixApp::new()
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root))
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(ioc_data)
            .app_data(user_verifier.clone())
            .app_data(token_processor.clone())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(actix_web::middleware::TrailingSlash::Trim))
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="ru">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <script>
                                    r#"(function(){try{var t=localStorage.getItem('jk-theme');if(t)document.documentElement.setAttribute('data-theme',t);}catch(e){}})();"#
                                </script>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
    };

    let tcp_listener = TcpListener::bind(site_addr).unwrap();

    let mut server = HttpServer::new(app_builder);

    if let Some(tls) = config.tls {
        rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();

        let mut key_file = BufReader::new(File::open(&tls.key).unwrap_or_else(|e| {
            log::error!("Failed to open TLS key: {}", e);
            std::process::exit(1);
        }));
        let mut certs_file = BufReader::new(File::open(&tls.cert).unwrap_or_else(|e| {
            log::error!("Failed to open TLS cert: {}", e);
            std::process::exit(1);
        }));

        let tls_certs = rustls_pemfile::certs(&mut certs_file)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
            .next()
            .unwrap()
            .unwrap();

        let tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
            .unwrap();

        server = server.listen_rustls_0_23(tcp_listener, tls_config).unwrap();
    } else {
        server = server.listen(tcp_listener).unwrap();
    }

    server.workers(config.workers).run().await.unwrap();
}
