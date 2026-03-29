use std::sync::Arc;

use axum::{Router, routing::get};
use sqlx::PgPool;

pub mod handler;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route(
            "/completion-by-genre",
            get(handler::get_completion_by_genre),
        )
        .route("/watch-patterns", get(handler::get_watch_patterns))
        .route("/data-quality", get(handler::get_data_quality))
        .route("/user/segment", get(handler::get_user_segment))
        .with_state(pool)
}
