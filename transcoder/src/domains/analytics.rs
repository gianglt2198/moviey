use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde_json::Value as JsonValue;
use uuid::Uuid;

/// Domain model - represents the user_behavior_snapshot table
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserBehaviorSnapshot {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub snapshot_date: NaiveDate,
    pub movies_watched_period: i32,
    pub total_watch_time_minutes: i32,
    pub avg_completion_rate: Decimal,
    pub preferred_genre: Option<String>,
    pub preferred_time_of_day: Option<String>,
    pub device_most_used: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Domain model - represents the data_quality_flags table
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DataQualityFlag {
    pub id: Uuid,
    pub watch_history_id: Uuid,
    pub flag_type: String,
    pub flag_severity: String,
    pub description: Option<String>,
    pub resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Domain model - represents the user_segments table
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserSegment {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub segment_type: String,
    pub segment_score: Decimal,
    pub last_calculated_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub metadata: JsonValue,
    pub created_at: DateTime<Utc>,
}
