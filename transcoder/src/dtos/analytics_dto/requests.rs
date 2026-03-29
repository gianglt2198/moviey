use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
// ============ REQUEST DTOs ============

// ============ RESPONSE DTOs ============

#[derive(Debug, Serialize, ToSchema)]
pub struct CompletionRateByGenre {
    pub genre: String,
    pub total_watches: i64,
    pub avg_completion_rate: f64,
    pub median_completion: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WatchTimePattern {
    pub time_period: String,
    pub watch_count: i64,
    pub avg_completion: f64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DataQualityReport {
    pub total_records: i64,
    pub valid_records: i64,
    pub flagged_records: i64,
    pub validation_percentage: f64,
    pub top_issues: Vec<(String, i64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserSegment {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub segment_type: String,
    pub segment_score: f64,
    pub last_calculated_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub metadata: serde_json::Value,
}
