use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    pub mod database;
    pub mod auth;
    pub mod argon2_password_hasher;
}}
