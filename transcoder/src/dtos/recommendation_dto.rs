use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ============ REQUEST DTOs ============

#[derive(Debug, Deserialize, ToSchema)]
pub struct GenerateRecommendationsRequest {
    pub limit: Option<i32>,
    pub diversity_factor: Option<f64>, // 0.0-1.0
    pub include_trending: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RecommendationFeedbackRequest {
    pub movie_id: Uuid,
    pub action: String, // "click", "watch", "not_interested", "dislike"
    pub watch_duration_seconds: Option<i32>,
}

// ============ RESPONSE DTOs ============

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct RecommendationResponse {
    pub movie_id: Uuid,
    pub title: String,
    pub score: f64,
    pub reason: String,
    pub rank: i32,
    pub thumbnail_url: String,
    pub genre: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RecommendationsListResponse {
    pub recommendations: Vec<RecommendationResponse>,
    pub total: i32,
    pub generated_at: String,
    pub expires_at: String,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct SimilarMovieResponse {
    pub movie_id: Uuid,
    pub title: String,
    pub similarity_score: f64,
    pub reason: String,
    pub thumbnail_url: String,
}
