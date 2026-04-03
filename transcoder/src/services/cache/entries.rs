use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::generic_cache::CacheEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheDefault;

impl CacheEntry for CacheDefault {
    fn key(&self) -> String {
        "rec:default".to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRecommendations {
    pub profile_id: Uuid,
    pub recommendations: Vec<crate::dtos::RecommendationResponse>,
    pub generated_at: String,
    pub expires_at: String,
}

impl CacheEntry for CachedRecommendations {
    fn key(&self) -> String {
        format!("rec:user:{}:recommendations", self.profile_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedMovieSimilarity {
    pub movie_id: Uuid,
    pub similar_movies: Vec<(Uuid, f64)>,
}

impl CacheEntry for CachedMovieSimilarity {
    fn key(&self) -> String {
        format!("rec:movie:{}:similar", self.movie_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedUserPreferences {
    pub profile_id: Uuid,
    pub genre_preferences: Vec<(String, f64)>,
    pub director_preferences: Vec<(String, f64)>,
}

impl CacheEntry for CachedUserPreferences {
    fn key(&self) -> String {
        format!("rec:user:{}:preferences", self.profile_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: i64,
    pub misses: i64,
    pub invalidations: i64,
    pub hit_rate: f64,
    pub total_requests: i64,
}

impl CacheEntry for CacheStats {
    fn key(&self) -> String {
        "rec:metrics:cache".to_string()
    }
}
