use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

pub mod handler;
pub mod validators;

pub fn router(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/register", post(handler::register))
        .route("/login", post(handler::login))
        .route("/profile", get(handler::get_user_profile))
        .route("/profile/create", post(handler::create_profile))
        .with_state(pool)
}
