use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// ============ REQUEST DTOs ============

#[derive(Debug, Deserialize, ToSchema)]
pub struct SaveWatchProgressRequest {
    pub movie_id: Uuid,
    pub position_seconds: i32,
    pub completed: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EnhancedWatchProgressRequest {
    pub movie_id: Uuid,
    pub watch_duration_seconds: i32,
    pub total_movie_duration_seconds: i32,
    pub watch_quality: String,
    pub interrupted_count: i32,
    pub playback_speed: f64,
    pub device_type: String,
    pub completed: bool,
}

// ============ RESPONSE DTOs ============

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct WatchHistoryResponse {
    pub id: Uuid,
    pub movie_id: Uuid,
    pub last_position_seconds: i32,
    pub completed: bool,
    pub watched_at: String,
}

#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct WatchHistoryDetailResponse {
    pub id: Uuid,
    pub movie_id: Uuid,
    pub last_position_seconds: i32,
    pub completed: bool,
    pub watch_duration_seconds: Option<i32>,
    pub total_movie_duration_seconds: Option<i32>,
    pub completion_percentage: Option<f64>,
    pub watch_quality: Option<String>,
    pub interrupted_count: Option<i32>,
    pub playback_speed: Option<f64>,
    pub device_type: Option<String>,
    pub completion_reason: Option<String>,
    pub flagged_for_review: Option<bool>,
    pub watched_at: String,
}
