use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    pub mod auth;
    pub mod argon2_password_hasher;
    pub mod database;
    pub mod basic_credentials_provider;
}}
