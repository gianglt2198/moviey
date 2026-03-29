use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

pub mod handler;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(handler::get_watch_histories))
        .route("/{history_id}", get(handler::get_watch_history))
        .route("/save", post(handler::save_watch_progress))
        .route(
            "/save-enhanced",
            post(handler::save_enhanced_watch_progress),
        )
        .with_state(pool)
}
