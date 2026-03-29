use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "movie_status", rename_all = "lowercase")]
pub enum MovieStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// Domain model - represents the movies table
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Movie {
    pub id: Uuid,
    pub title: String,
    pub original_path: String,
    pub hls_path: Option<String>,
    pub thumbnail_path: Option<String>,
    pub status: MovieStatus,
    pub genre: Option<String>,
    pub director: Option<String>,
    pub release_year: Option<i32>,
    pub rating: Option<rust_decimal::Decimal>,
    pub description: Option<String>,
    pub duration_seconds: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
