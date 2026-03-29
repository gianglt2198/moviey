use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

// ============ REQUEST DTOs ============

#[derive(Debug, Deserialize, IntoParams)]
pub struct SearchMovieQuery {
    pub q: Option<String>,
    pub genre: Option<String>,
    pub sort: Option<String>, // recent, rating, title
}

// ============ RESPONSE DTOs ============

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct MovieResponse {
    pub id: Uuid,
    pub title: String,
    pub stream_url: String,
    pub status: String,
    pub duration: String,
    pub thumbnail_url: String,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct MovieDetailResponse {
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
    pub description: Option<String>,
}
