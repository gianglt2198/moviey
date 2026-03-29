use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, ToSchema)]
pub enum InteractionType {
    #[serde(rename = "watched")]
    Watched,
    #[serde(rename = "abandoned")]
    Abandoned,
    #[serde(rename = "sampled")]
    Sampled,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct EnhancedWatchHistoryEntry {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub movie_id: Uuid,
    pub watch_duration_seconds: i32,
    pub total_movie_duration_seconds: i32,
    pub completion_percentage: f64,
    pub watch_quality: String,
    pub interrupted_count: i32,
    pub playback_speed: f64,
    pub device_type: String,
    pub completion_reason: String,
    pub flagged_for_review: bool,
    pub watched_at: DateTime<Utc>,
    pub last_session_resumed_at: Option<DateTime<Utc>>,
}

// ============ DATA VALIDATION ============

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct WatchProgressRequest {
    pub movie_id: Uuid,
    pub watch_duration_seconds: i32,
    pub total_movie_duration_seconds: i32,
    pub watch_quality: String,
    pub interrupted_count: i32,
    pub playback_speed: f64,
    pub device_type: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityFlag {
    pub id: Uuid,
    pub watch_history_id: Uuid,
    pub flag_type: String,
    pub flag_severity: String,
    pub description: String,
    pub resolved: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct QualityFlagResponse {
    pub total_flags: i64,
    pub critical_count: i64,
    pub error_count: i64,
    pub warning_count: i64,
    pub resolution_rate: f64,
}

// ============ USER BEHAVIOR ============

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserBehaviorSnapshot {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub snapshot_date: NaiveDate,
    pub movies_watched_period: i32,
    pub total_watch_time_minutes: i32,
    pub avg_completion_rate: f64,
    pub preferred_genre: Option<String>,
    pub preferred_time_of_day: Option<String>,
    pub device_most_used: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct BehaviorAnalyticsResponse {
    pub current_snapshot: UserBehaviorSnapshot,
    pub previous_snapshot: Option<UserBehaviorSnapshot>,
    pub trend: String, // "improving", "declining", "stable"
    pub recommendation: String,
}

// ============ USER SEGMENTS ============

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, ToSchema)]
pub enum UserSegmentType {
    #[serde(rename = "binge_watcher")]
    BingeWatcher,
    #[serde(rename = "casual_viewer")]
    CasualViewer,
    #[serde(rename = "explorer")]
    Explorer,
    #[serde(rename = "weekend_warrior")]
    WeekendWarrior,
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "general")]
    General,
}

impl UserSegmentType {
    pub fn as_str(&self) -> &str {
        match self {
            UserSegmentType::BingeWatcher => "BINGE_WATCHER",
            UserSegmentType::CasualViewer => "CASUAL_VIEWER",
            UserSegmentType::Explorer => "EXPLORER",
            UserSegmentType::WeekendWarrior => "WEEKEND_WARRIOR",
            UserSegmentType::Inactive => "INACTIVE",
            UserSegmentType::General => "GENERAL",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "BINGE_WATCHER" => UserSegmentType::BingeWatcher,
            "CASUAL_VIEWER" => UserSegmentType::CasualViewer,
            "EXPLORER" => UserSegmentType::Explorer,
            "WEEKEND_WARRIOR" => UserSegmentType::WeekendWarrior,
            "INACTIVE" => UserSegmentType::Inactive,
            _ => UserSegmentType::General,
        }
    }
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserSegmentResponse {
    pub segment_type: String,
    pub segment_score: f64,
    pub description: String,
    pub characteristics: Vec<String>,
    pub recommendations: Vec<String>,
}

// ============ ANALYTICS QUERIES ============

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
