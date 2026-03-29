use tracing::info;
use tracing_subscriber;

pub fn init_logging() {
    let level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    tracing_subscriber::fmt()
        .with_max_level(level.parse().unwrap_or(tracing::Level::INFO))
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();

    info!("🚀 Logging initialized at level: {}", level);
}
