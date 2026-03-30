use axum::{Json, Router, middleware, response::Result, routing::get};
use dotenv::dotenv;
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use std::{sync::Arc, time::Duration};
use tower_http::{cors::CorsLayer, services::ServeDir};
use utoipa::{Modify, OpenApi,     openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},  
};
use utoipa_swagger_ui::SwaggerUi;

mod domains;
mod dtos;

mod handlers;
mod logging;
mod middlewares;
mod models;
mod runner;
mod services;
mod scheduler;
mod config;

use handlers::*;

use crate::config::redis::{RedisConfig, RedisPool};
use crate::middlewares::security_headers::add_security_headers;

#[derive(OpenApi)]
#[openapi(
    paths(  
        handlers::movie::handler::get_movies,  
        handlers::movie::handler::search_movies,  
        handlers::movie::handler::get_movie_detail,  
        handlers::movie::handler::upload_movie,  
        handlers::user::handler::register,  
        handlers::user::handler::login,  
        handlers::user::handler::get_user_profile,  
        handlers::user::handler::create_profile,  
        handlers::watch_history::handler::save_watch_progress,  
        handlers::watch_history::handler::save_enhanced_watch_progress,  
        handlers::watch_history::handler::get_watch_histories,  
        handlers::watch_history::handler::get_watch_history,  
        handlers::favorites::handler::toggle_favorite,  
        handlers::favorites::handler::get_favorites,  
        handlers::analytics::handler::get_completion_by_genre,  
        handlers::analytics::handler::get_watch_patterns,  
        handlers::analytics::handler::get_data_quality,  
        handlers::analytics::handler::get_user_segment,  
    ),  
    components(  
        schemas(  
            dtos::MovieResponse,  
            dtos::MovieDetailResponse,  
            // dtos::SearchMovieQuery,  
            dtos::RegisterRequest,  
            dtos::LoginRequest,  
            dtos::AuthResponse,  
            dtos::CreateProfileRequest,  
            dtos::ProfileResponse,  
            dtos::SaveWatchProgressRequest,  
            dtos::EnhancedWatchProgressRequest,  
            dtos::WatchHistoryResponse,  
            dtos::WatchHistoryDetailResponse,  
            dtos::ToggleFavoriteRequest,  
            dtos::FavoritesListResponse,  
            dtos::FavoriteMovieResponse,  
            dtos::CompletionRateByGenre,  
            dtos::WatchTimePattern,  
            dtos::DataQualityReport,  
            dtos::UserSegment,  
            models::Claims,  
        )  
    ),  
    modifiers(&SecurityAddon),  
    tags(  
        (name = "Movies", description = "Movie listing and search endpoints"),  
        (name = "Authentication", description = "User registration and login"),  
        (name = "User", description = "User profile management"),  
        (name = "Watch History", description = "Track and retrieve watch history"),  
        (name = "Favorites", description = "Manage favorite movies"),  
        (name = "Analytics", description = "Retrieve analytics and user insights"),  
    ),  
    info(  
        title = "Movie Streaming API",  
        description = "A comprehensive API for movie streaming with analytics",  
        version = "1.0.0",  
        contact(  
            name = "API Support",  
            email = "support@example.com"  
        ),  
    )  
)]
struct ApiDoc;
/// Security scheme addon for JWT authentication  
struct SecurityAddon;

impl Modify for SecurityAddon {  
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {  
        if let Some(components) = &mut openapi.components {  
            components.add_security_scheme(  
                "bearer_auth",  
                SecurityScheme::ApiKey(ApiKey::Header(  
                    ApiKeyValue::new("Authorization")
                )),
            )  
        }  
    }  
}  

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

       // Initialize Redis  
    let redis_config = RedisConfig::default();  
    let redis = RedisPool::new(redis_config).await?;  
    let redis = Arc::new(redis);  


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

    let pool = Arc::new(pool);
    let watcher_pool = pool.clone();
    let output_base = output_dir.to_string();

    tokio::spawn(async move {
        let runner = runner::Runner::new(watcher_pool, upload_dir.to_string(), output_base);
        runner.start().await;
    });

    let scheduler_pool = pool.clone();  
    let scheduler_redis = redis.clone();
    tokio::spawn(async move {  
        let runner = scheduler::BatchJobRunner::new(scheduler_pool, scheduler_redis);  
        runner.start().await;  
    });

    let app = Router::new()
        // Swagger UI route
        .merge(SwaggerUi::new("/api-docs/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))  
        // Health check route
        .route("/health", get(health_check))
        // Modular routers with nesting
        .nest("/api/movies", movie_router(pool.clone()))
        .nest("/api/auth", user_router(pool.clone()))
        .nest("/api/watch-history", watch_history_router(pool.clone(), redis.clone()))
        .nest("/api/favorites", favorites_router(pool.clone()))
        .nest("/api/analytics", analytics_router(pool.clone()))
        .nest("/api/recommendations", recommendation_router(pool.clone(), redis.clone()))
        .nest("/api/jobs", jobs_router(pool.clone())) 
        // User-specific routes
        .nest_service("/streams", ServeDir::new(output_dir))
        .layer(middleware::from_fn(add_security_headers))
        .layer(CorsLayer::permissive());

    // 4. START THE SERVER
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 API Live at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
