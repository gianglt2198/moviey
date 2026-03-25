use axum::{
    Router,
    response::Result,
    routing::{get, post},
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::{env, net::SocketAddr};
use tower_http::{cors::CorsLayer, services::ServeDir};

mod handlers;
mod middlewares;
mod models;
mod runner;

use handlers::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://moviey:password@localhost:5433/moviey".to_string());
    let upload_dir = env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".to_string());
    let output_dir = env::var("OUTPUT_DIR").unwrap_or_else(|_| "./stream_output".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
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
        // Public routes
        .route("/api/movies", get(get_movies))
        .route("/api/movies/upload", post(upload_movie))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        // Protected routes
        .route("/api/user/profile", get(get_user_profile))
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
