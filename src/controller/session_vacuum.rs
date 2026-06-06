use std::sync::Arc;
use std::time::Duration;
use crate::application::common::interactor::Interactor;
use crate::interactor_factory::InteractorFactory;

pub async fn run(ioc: Arc<dyn InteractorFactory>, interval: Duration) {
    let mut ticker = tokio::time::interval(interval);
    loop {
        ticker.tick().await;
        match ioc.vacuum_sessions().execute(()).await {
            Ok(n) if n > 0 => log::info!("session vacuum: removed {} expired sessions", n),
            Ok(_) => {}
            Err(e) => log::error!("session vacuum: {}", e),
        }
    }
}
