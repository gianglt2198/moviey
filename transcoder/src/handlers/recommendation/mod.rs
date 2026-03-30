use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::config::redis::RedisPool;

pub mod handler;

pub fn router(pool: Arc<PgPool>, redis: Arc<RedisPool>) -> Router {
    Router::new()
        .route(
            "/generate/{user_id}",
            post(handler::generate_recommendations),
        )
        .route("/{user_id}", get(handler::get_recommendations))
        .route("/similar/{movie_id}", get(handler::get_similar_movies))
        .route("/feedback", post(handler::save_recommendation_feedback))
        .route("/cache/performance", get(handler::get_cache_performance))
        .with_state((pool, redis))
}
