use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, sqlx::Type)]
#[sqlx(type_name = "movie_status", rename_all = "lowercase")]
pub enum MovieStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieDto {
    pub id: Uuid,
    pub title: String,
    pub stream_url: String, // This will point to our static file server
    pub status: MovieStatus,
    pub duration: String,
    pub thumbnail_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieDetailDto {
    pub id: Uuid,
    pub title: String,
    pub stream_url: String,
    pub status: MovieStatus,
    pub duration: String,
    pub thumbnail_url: String,
    pub genre: Option<String>,
    pub director: Option<String>,
    pub release_year: Option<i32>,
    pub rating: Option<f64>,
    pub description: Option<String>,
}

// Watch Progress DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct WatchProgressDto {
    pub movie_id: Uuid,
    pub position_seconds: i32,
    pub completed: bool,
}

// Search Query
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub genre: Option<String>,
    pub sort: Option<String>, // recent, rating, title
}

// Favorite Request
#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteRequest {
    pub movie_id: Uuid,
}
