use std::fs::File;
use std::io::BufReader;
use std::net::TcpListener;
use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use sqlx::sqlite::SqlitePoolOptions;
use crate::adapters::argon2_password_hasher::Argon2PasswordHasher;
use crate::adapters::auth::token::TokenProcessor;
use crate::adapters::database::initial::initial_models;
use crate::application::common::hasher::Hasher;
use crate::ioc::IoC;
use crate::presentation::interactor_factory::InteractorFactory;

mod adapters;
mod application;
mod presentation;
mod domain;
mod config;
mod ioc;

pub struct CredentialsProvider {
    pub username: String,
    pub hashed_password: String,
}


#[actix_web::main]
async fn main() {
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

    let credentials_provider = CredentialsProvider {
        username: config.credentials.username,
        hashed_password: Argon2PasswordHasher::new().hash(&config.credentials.password).await,
    };
    let ioc = Arc::new(IoC::new(db_pool, credentials_provider));

    let token_processor = web::Data::new(TokenProcessor::new());

    let app_builder = move || {
        let ioc_arc: Arc<dyn InteractorFactory> = ioc.clone();
        let ioc_data: web::Data<dyn InteractorFactory> = web::Data::from(ioc_arc);

        App::new()
            .service(web::scope("/api")
                .configure(presentation::rest::user::router)
                .configure(presentation::rest::session::router)
            )
            .app_data(token_processor.clone())
            .app_data(ioc_data)
            .default_service(web::route().to(presentation::rest::exception::not_found))
            .wrap(Logger::default())
    };

    let tcp_listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).unwrap();

    let mut server = HttpServer::new(app_builder);
    if let Some(tls) = config.tls {
        rustls::crypto::aws_lc_rs::default_provider().install_default().unwrap();

        let mut key_file = BufReader::new(File::open(tls.key).map_err(
            |error| {
                log::error!("Failed to open key music: {}", error.to_string());
                std::process::exit(1);
            }
        ).unwrap());

        let mut certs_file = BufReader::new(File::open(tls.cert).map_err(
            |error| {
                log::error!("Failed to open certificate music: {}", error.to_string());
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
