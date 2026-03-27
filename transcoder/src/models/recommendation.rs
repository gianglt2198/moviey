use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateMovieRequest {
    pub rating: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecommendationDto {
    pub id: Uuid,
    pub title: String,
    pub stream_url: String,
    pub genre: Option<String>,
    pub director: Option<String>,
    pub rating: Option<f32>,
    pub thumbnail_url: String,
    pub recommendation_score: f32,
    pub recommendation_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRatingStats {
    pub total_rated: i32,
    pub average_rating: f32,
    pub most_rated_genre: Option<String>,
}
