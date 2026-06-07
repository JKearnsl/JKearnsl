mod config;

#[cfg(feature = "ssr")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "ssr")]
async fn serve_pkg(
    req: actix_web::HttpRequest,
    filename: actix_web::web::Path<String>,
    opts: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::HttpResponse {
    use actix_files::NamedFile;
    use actix_web::http::header::{self, HeaderValue};

    let site_root = opts.site_root.as_ref();
    let name = filename.into_inner();
    if name.contains("..") || name.contains('/') {
        return actix_web::HttpResponse::NotFound().finish();
    }
    let base = format!("{site_root}/pkg/{name}");
    let ct = match name.rsplit_once('.').map(|(_, e)| e) {
        Some("wasm") => "application/wasm",
        Some("js")   => "text/javascript; charset=utf-8",
        Some("css")  => "text/css; charset=utf-8",
        _            => "application/octet-stream",
    };
    let accept = req
        .headers()
        .get(header::ACCEPT_ENCODING)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if accept.contains("br") {
        if let Ok(f) = NamedFile::open_async(format!("{base}.br")).await {
            let mut res = f.use_etag(true).use_last_modified(true).into_response(&req);
            res.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static(ct));
            res.headers_mut().insert(header::CONTENT_ENCODING, HeaderValue::from_static("br"));
            res.headers_mut().insert(header::VARY, HeaderValue::from_static("Accept-Encoding"));
            return res;
        }
    }
    if accept.contains("gzip") {
        if let Ok(f) = NamedFile::open_async(format!("{base}.gz")).await {
            let mut res = f.use_etag(true).use_last_modified(true).into_response(&req);
            res.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static(ct));
            res.headers_mut().insert(header::CONTENT_ENCODING, HeaderValue::from_static("gzip"));
            res.headers_mut().insert(header::VARY, HeaderValue::from_static("Accept-Encoding"));
            return res;
        }
    }

    NamedFile::open_async(&base)
        .await
        .map(|f| f.use_etag(true).use_last_modified(true).into_response(&req))
        .unwrap_or_else(|_| actix_web::HttpResponse::NotFound().finish())
}

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
    use jkearnsl::adapters;

    use leptos::config::get_configuration;
    use leptos_actix::{generate_route_list, LeptosRoutes};
    use jkearnsl::controller::web::app::App;

    use adapters::database::pool::create_pool;
    use adapters::auth::token::TokenProcessor;
    use adapters::database::session::SqliteSessionGateway;
    use adapters::database::user::SqliteUserGateway;
    use jkearnsl::controller::default_user;
    use jkearnsl::controller::session_vacuum;
    use jkearnsl::ioc::IoC;

    let config = config::Config::from_env();
    pretty_env_logger::init_custom_env("LOG_LEVEL");

    let db_pool = create_pool(config.workers).await;
    let ioc = Arc::new(IoC::new(db_pool.clone()));

    default_user::run(&*ioc).await.expect("seeding default user");

    tokio::spawn(session_vacuum::run(
        ioc.clone(),
        Duration::from_secs(60 * 60),
    ));

    let token_processor = web::Data::new(TokenProcessor::new(
        SqliteSessionGateway::new(db_pool.clone()),
        SqliteUserGateway::new(db_pool.clone()),
    ));
    let conf = get_configuration(None).expect("leptos configuration required");
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
                web::resource("/pkg/{filename}")
                    .route(web::get().to(serve_pkg))
            )
            .service(
                Files::new("/assets", site_root)
                    .use_etag(true)
                    .use_last_modified(true),
            )
            .app_data(web::Data::new(leptos_options.to_owned()))
            .app_data(ioc_data)
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
                move || jkearnsl::controller::web::index::shell(leptos_options.clone())
            })
    };

    let tcp_listener = TcpListener::bind(site_addr).expect("bind TCP listener");

    let mut server = HttpServer::new(app_builder)
        .workers(config.workers)
        .backlog(2048);

    if let Some(tls) = config.tls {
        rustls::crypto::aws_lc_rs::default_provider().install_default().expect("install TLS crypto provider");

        let mut key_file = BufReader::new(File::open(&tls.key).unwrap_or_else(|e| {
            log::error!("opening TLS key file: {}", e);
            std::process::exit(1);
        }));
        let mut certs_file = BufReader::new(File::open(&tls.cert).unwrap_or_else(|e| {
            log::error!("opening TLS cert file: {}", e);
            std::process::exit(1);
        }));

        let tls_certs = rustls_pemfile::certs(&mut certs_file)
            .collect::<Result<Vec<_>, _>>()
            .expect("parse TLS certificates");
        let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
            .next()
            .expect("no TLS private key found")
            .expect("parse TLS private key");

        let mut tls_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
            .expect("configure TLS");
        tls_config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

        server = server.listen_rustls_0_23(tcp_listener, tls_config).expect("bind TLS listener");
    } else {
        server = server.listen(tcp_listener).expect("bind listener");
    }

    server.run().await.expect("server failed");
}
