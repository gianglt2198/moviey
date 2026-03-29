use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

pub mod handler;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/", get(handler::get_movies))
        .route("/search", get(handler::search_movies))
        .route("/{movie_id}", get(handler::get_movie_detail))
        .route("/upload", post(handler::upload_movie))
        .with_state(pool)
}
