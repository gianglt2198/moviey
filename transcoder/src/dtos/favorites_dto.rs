use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ============ REQUEST DTOs ============

#[derive(Debug, Deserialize, ToSchema)]
pub struct ToggleFavoriteRequest {
    pub movie_id: Uuid,
}

// ============ RESPONSE DTOs ============

#[derive(Debug, Serialize, ToSchema)]
pub struct FavoritesListResponse {
    pub total_count: i32,
    pub favorites: Vec<FavoriteMovieResponse>,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct FavoriteMovieResponse {
    pub id: Uuid,
    pub title: String,
    pub stream_url: String,
    pub status: String,
    pub duration: String,
    pub thumbnail_url: String,
    pub genre: Option<String>,
    pub director: Option<String>,
    pub release_year: Option<i32>,
    pub rating: Option<f64>,
}
