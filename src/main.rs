use leptos::html::InnerHtml;
use leptos::tachys::html::class::IntoClass;
use leptos::web_sys::Comment;


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
    use sqlx::sqlite::SqlitePoolOptions;
    use jkearnsl::{adapters, ioc};
    use jkearnsl::adapters::basic_credentials_provider::BasicCredentialsProvider;

    use leptos::prelude::*;
    use leptos::config::get_configuration;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;

    use jkearnsl::presentation::app::App;

    use adapters::argon2_password_hasher::Argon2PasswordHasher;
    use adapters::auth::token::TokenProcessor;
    use adapters::database::initial::initial_models;
    use jkearnsl::application::common::hasher::Hasher;
    use jkearnsl::interactor_factory::InteractorFactory;
    use jkearnsl::ioc::IoC;


    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let config = config::Config::from_env();
    pretty_env_logger::init_custom_env("LOG_LEVEL");

    // Initial
    let db_pool = SqlitePoolOptions::new()
        .max_connections(100) // todo: test it
        .connect(&"sqlite://database?mode=rwc".to_string()).await.map_err(
        |error| {
            log::error!("Failed to connect to database: {}", error.to_string());
            std::process::exit(1);
        }
    ).unwrap();

    initial_models(&db_pool).await.map_err(
        |error| {
            log::error!("Failed to initial models: {}", error.to_string());
            std::process::exit(1);
        }
    ).unwrap();

    let credentials_provider = BasicCredentialsProvider::new(
        config.credentials.username,
        Argon2PasswordHasher::new().hash(&config.credentials.password).await,
    );
    let ioc = Arc::new(IoC::new(db_pool, credentials_provider));
    let token_processor = web::Data::new(TokenProcessor::new());

    let conf = get_configuration(None).unwrap();
    let app_builder = move || {
        let ioc_arc: Arc<dyn InteractorFactory> = ioc.clone();
        let ioc_data: web::Data<dyn InteractorFactory> = web::Data::from(ioc_arc);

        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root.as_ref();

        ActixApp::new()
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            .service(Files::new("/assets", site_root))
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(token_processor.clone())
            .app_data(ioc_data)
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(
                actix_web::middleware::TrailingSlash::Trim,
            ))
            .leptos_routes(routes,{
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <meta http-equiv="X-UA-Compatible" content="ie=edge"/>
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

    let tcp_listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).unwrap();

    let mut server = HttpServer::new(app_builder);
    if let Some(tls) = config.tls {
        rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();

        let mut key_file = BufReader::new(File::open(tls.key).map_err(
            |error| {
                log::error!("Failed to open key: {}", error.to_string());
                std::process::exit(1);
            }
        ).unwrap());

        let mut certs_file = BufReader::new(File::open(tls.cert).map_err(
            |error| {
                log::error!("Failed to open certificate: {}", error.to_string());
                std::process::exit(1);
            }
        ).unwrap());

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
