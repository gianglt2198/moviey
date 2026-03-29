use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub movie_id: Uuid,
    pub score: f64,
    pub reason: String,
    pub collab_score: f64,
    pub content_score: f64,
    pub generated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MovieSimilarity {
    pub id: Uuid,
    pub movie_a_id: Uuid,
    pub movie_b_id: Uuid,
    pub similarity_score: f64,
    pub similarity_type: String, // "genre", "director", "content"
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserPreference {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub preferred_genres: Vec<String>,
    pub preferred_directors: Vec<String>,
    pub watch_frequency: i32,
    pub avg_completion_rate: Decimal,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct UserVector {
    pub profile_id: Uuid,
    pub genre_preferences: Vec<(String, f64)>,
    pub director_preferences: Vec<(String, f64)>,
    pub temporal_pattern: String,
}

#[derive(Debug, Clone)]
pub struct MovieVector {
    pub movie_id: Uuid,
    pub genre: String,
    pub director: String,
    pub rating: f64,
    pub tags: Vec<String>,
    pub release_year: i32,
}
