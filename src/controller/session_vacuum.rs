use std::time::Duration;
use crate::adapters::database::{pool::DbPool, session::SqliteSessionGateway};
use crate::application::{
    common::interactor::Interactor,
    session::vacuum::VacuumSessions,
};

const SESSION_TTL_SECS: i64 = 60 * 60 * 24 * 7; // 7 days

pub async fn run(db: DbPool, interval: Duration) {
    let gateway = SqliteSessionGateway::new(db);
    let mut ticker = tokio::time::interval(interval);
    loop {
        ticker.tick().await;
        let interactor = VacuumSessions { session_vacuum: &gateway, max_age_secs: SESSION_TTL_SECS };
        match interactor.execute(()).await {
            Ok(n) if n > 0 => log::info!("session vacuum: removed {} expired sessions", n),
            _ => {}
        }
    }
}
