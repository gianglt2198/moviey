use axum::{Router, routing::get};
use sqlx::PgPool;
use std::sync::Arc;

pub mod handler;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/executions", get(handler::get_job_executions))
        .route("/executions/:job_name", get(handler::get_job_history))
        .route("/statistics", get(handler::get_job_statistics))
        .route("/health", get(handler::get_job_health))
        .with_state(pool)
}
