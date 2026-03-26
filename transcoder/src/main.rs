use axum::{
    Json, Router,
    response::Result,
    routing::{get, post},
};
use dotenv::dotenv;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use std::{sync::Arc, time::Duration};
use tower_http::{cors::CorsLayer, services::ServeDir};

mod handlers;
mod logging;
mod middlewares;
mod models;
mod runner;

use handlers::*;

pub async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    logging::init_logging();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://moviey:password@localhost:5433/moviey".to_string());
    let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let output_dir = env::var("OUTPUT_DIR").unwrap_or_else(|_| "./stream_output".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(20) // Increased for production
        .min_connections(5) // Minimum idle connections
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&database_url)
        .await?;

    // Run migrations (if you have any)
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Database connected and migrations applied.");

    let watcher_pool = pool.clone();
    let output_base = output_dir.to_string();

    tokio::spawn(async move {
        let runner =
            runner::Runner::new(Arc::new(watcher_pool), upload_dir.to_string(), output_base);
        runner.start().await;
    });

    let app = Router::new()
        // Health check route
        .route("/health", get(health_check))
        // Public routes
        .route("/api/movies", get(get_movies))
        .route("/api/movies/search", get(search_movies))
        .route("/api/movies/{movie_id}", get(get_movie_detail))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        // Protected routes
        .route("/api/user/profile", get(get_user_profile))
        .route("/api/watch-history", get(get_watch_history))
        .route("/api/watch-progress", post(save_watch_progress))
        .route("/api/favorites", get(get_favorites))
        .route("/api/favorites/toggle", post(toggle_favorite))
        // Admin routes
        .route("/api/movies/upload", post(upload_movie))
        // User-specific routes
        .nest_service("/streams", ServeDir::new(output_dir))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // 4. START THE SERVER
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 API Live at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
