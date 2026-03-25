use axum::{
    Router,
    response::Result,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, services::ServeDir};

mod handlers;
mod middlewares;
mod models;
mod runner;

use handlers::*;

const DATABASE_URL: &str = "postgres://moviey:password@localhost:5433/moviey";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await?;

    // Run migrations (if you have any)
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Database connected and migrations applied.");

    let input_dir = "./uploads";
    let output_dir = "./stream_output";

    let watcher_pool = pool.clone();
    let output_base = output_dir.to_string();

    tokio::spawn(async move {
        let runner =
            runner::Runner::new(Arc::new(watcher_pool), input_dir.to_string(), output_base);
        runner.start().await;
    });

    let app = Router::new()
        .route("/api/movies", get(get_movies))
        .nest_service("/streams", ServeDir::new(output_dir))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    // 4. START THE SERVER
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 API Live at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
