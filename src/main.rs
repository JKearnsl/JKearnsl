mod config;

#[cfg(feature = "ssr")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() {
    use std::fs::File;
    use std::io::BufReader;
    use std::net::TcpListener;
    use std::sync::Arc;
    use std::time::Duration;
    use actix_files::Files;
    use actix_web::{App as ActixApp, HttpServer, web};
    use actix_web::dev::Service;
    use actix_web::http::header::{self, HeaderValue};
    use actix_web::middleware::{Compress, DefaultHeaders, NormalizePath};
    use actix_web::middleware::Logger;
    use sqlx::sqlite::{
        SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous,
    };
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
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(30))
        // 64 MB page cache (negative = kibibytes)
        .pragma("cache_size", "-65536")
        // 256 MB memory-mapped I/O — reduces syscalls on reads
        .pragma("mmap_size", "268435456")
        .pragma("temp_store", "memory");

    let pool_size = (config.workers * 2).max(10) as u32;
    let db_pool = SqlitePoolOptions::new()
        .max_connections(pool_size)
        .min_connections(config.workers.min(4) as u32)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
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
    let has_tls = config.tls.is_some();

    let app_builder = move || {
        let ioc_arc: Arc<dyn jkearnsl::interactor_factory::InteractorFactory> = ioc.clone();
        let ioc_data: web::Data<dyn jkearnsl::interactor_factory::InteractorFactory> =
            web::Data::from(ioc_arc);

        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.as_ref();

        let mut security_headers = DefaultHeaders::new()
            .add(("X-Content-Type-Options", "nosniff"))
            .add(("X-Frame-Options", "SAMEORIGIN"))
            .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
            .add(("Permissions-Policy", "camera=(), microphone=(), geolocation=()"));
        if has_tls {
            security_headers = security_headers
                .add(("Strict-Transport-Security", "max-age=63072000; includeSubDomains; preload"));
        }

        ActixApp::new()
            .service(
                Files::new("/pkg", format!("{site_root}/pkg"))
                    .use_etag(true)
                    .use_last_modified(true),
            )
            .service(
                Files::new("/assets", site_root)
                    .use_etag(true)
                    .use_last_modified(true),
            )
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(ioc_data)
            .app_data(user_verifier.clone())
            .app_data(token_processor.clone())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(actix_web::middleware::TrailingSlash::Trim))
            .wrap(security_headers)
            // Cache-Control: /pkg files are content-hashed by leptos -> immutable forever
            .wrap_fn(|req, srv| {
                let path = req.path().to_owned();
                let fut = srv.call(req);
                async move {
                    let mut res = fut.await?;
                    let headers = res.headers_mut();
                    if path.starts_with("/pkg/") {
                        headers.insert(
                            header::CACHE_CONTROL,
                            HeaderValue::from_static("public, max-age=31536000, immutable"),
                        );
                    } else if path.starts_with("/assets/") {
                        headers.insert(
                            header::CACHE_CONTROL,
                            HeaderValue::from_static("public, max-age=86400, must-revalidate"),
                        );
                    }
                    Ok(res)
                }
            })
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

    let mut server = HttpServer::new(app_builder)
        .workers(config.workers)
        .keep_alive(Duration::from_secs(75))
        .client_request_timeout(Duration::from_secs(10))
        .backlog(2048);

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

        let mut tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
            .unwrap();
        // Явный ALPN: браузеры увидят h2 и договорятся на HTTP/2 по TLS
        tls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

        server = server.listen_rustls_0_23(tcp_listener, tls_config).unwrap();
    } else {
        server = server.listen(tcp_listener).unwrap();
    }

    server.run().await.unwrap();
}
