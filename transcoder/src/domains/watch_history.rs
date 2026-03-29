use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use uuid::Uuid;

/// Domain model - represents the watch_history table
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct WatchHistory {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub movie_id: Uuid,
    pub last_position_seconds: i32,
    pub completed: bool,
    pub watched_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Rich metadata columns
    pub watch_duration_seconds: Option<i32>,
    pub total_movie_duration_seconds: Option<i32>,
    pub completion_percentage: Option<Decimal>,
    pub watch_quality: Option<String>,
    pub interrupted_count: Option<i32>,
    pub last_session_resumed_at: Option<DateTime<Utc>>,
    pub playback_speed: Option<Decimal>,
    pub device_type: Option<String>,
    pub completion_reason: Option<String>,
    pub flagged_for_review: Option<bool>,
}
