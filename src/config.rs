use std::thread;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CredentialsConfig {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tls {
    pub cert: String,
    pub key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub tls: Option<Tls>,
    pub credentials: CredentialsConfig
}

impl Config {
    pub fn from_env() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("PORT").unwrap_or_else(|_| 8080.to_string()).parse().unwrap();
        
        let workers = match thread::available_parallelism() {
            Ok(parallelism) => usize::from(parallelism),
            Err(_) => 1,
        };
        
        let tsl_cert = std::env::var("TLS_CERT").ok();
        let tsl_key = std::env::var("TLS_KEY").ok();
        let tls = match (tsl_cert, tsl_key) {
            (Some(cert), Some(key)) => Some(Tls { cert, key }),
            _ => None
        };

        let credentials = CredentialsConfig {
            username: std::env::var("USERNAME").unwrap_or_else(|_| "admin".to_string()),
            password: std::env::var("PASSWORD").unwrap_or_else(|_| "admin".to_string())
        };
        
        Self {
            host,
            port,
            workers,
            tls,
            credentials
        }
    }
}
