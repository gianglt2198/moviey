use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

pub mod handler;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(handler::get_favorites))
        .route("/toggle", post(handler::toggle_favorite))
        .with_state(pool)
}
