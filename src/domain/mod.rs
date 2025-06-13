use cfg_if::cfg_if;

cfg_if! { if #[cfg(feature = "ssr")] {
    pub mod models;
    pub mod services;
    pub mod id_generator;
}}
