use std::time::Duration;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use std::str::FromStr;

pub type DbPool = sqlx::SqlitePool;

pub async fn create_pool(workers: usize) -> DbPool {
    let connect_options = SqliteConnectOptions::from_str("sqlite://database?mode=rwc")
        .unwrap_or_else(|e| {
            log::error!("Invalid database URL: {}", e);
            std::process::exit(1);
        })
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(30))
        .pragma("cache_size", "-65536")
        .pragma("mmap_size", "268435456")
        .pragma("temp_store", "memory");

    let pool_size = (workers * 2).max(10) as u32;
    let pool = SqlitePoolOptions::new()
        .max_connections(pool_size)
        .min_connections(workers.min(4) as u32)
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(600))
        .connect_with(connect_options)
        .await
        .unwrap_or_else(|e| {
            log::error!("Failed to connect to database: {}", e);
            std::process::exit(1);
        });

    sqlx::migrate!().run(&pool).await.unwrap_or_else(|e| {
        log::error!("Failed to run migrations: {}", e);
        std::process::exit(1);
    });

    pool
}
