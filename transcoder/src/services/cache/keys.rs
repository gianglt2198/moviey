use std::time::Duration;
use uuid::Uuid;

pub struct CacheKeys;

impl CacheKeys {
    /// Recommendation cache key
    pub fn recommendations(profile_id: Uuid) -> String {
        format!("rec:user:{}:recommendations", profile_id)
    }

    /// Recommendation metadata (generation time, expiry)
    pub fn recommendations_metadata(profile_id: Uuid) -> String {
        format!("rec:user:{}:metadata", profile_id)
    }

    /// Movie similarity cache
    pub fn movie_similarity(movie_id: Uuid) -> String {
        format!("rec:movie:{}:similar", movie_id)
    }

    /// User preferences cache
    pub fn user_preferences(profile_id: Uuid) -> String {
        format!("rec:user:{}:preferences", profile_id)
    }

    /// Cache warming status
    pub fn warming_status(profile_id: Uuid) -> String {
        format!("rec:warming:{}:status", profile_id)
    }

    /// Active users set (for cache warming)
    pub fn active_users_set() -> String {
        "rec:active_users".to_string()
    }

    /// Cache hit/miss metrics
    pub fn cache_metrics() -> String {
        "rec:metrics:cache".to_string()
    }
}

pub struct CacheTTL;

impl CacheTTL {
    /// 24 hours for personalized recommendations
    pub fn recommendations() -> Duration {
        Duration::from_secs(86400)
    }

    /// 6 hours for movie similarity (less frequently updated)
    pub fn movie_similarity() -> Duration {
        Duration::from_secs(21600)
    }

    /// 3 hours for user preferences
    pub fn user_preferences() -> Duration {
        Duration::from_secs(10800)
    }

    /// 15 minutes for warming status
    pub fn warming_status() -> Duration {
        Duration::from_secs(900)
    }

    /// 30 days for active user tracking
    pub fn active_users() -> Duration {
        Duration::from_secs(2592000)
    }
}
