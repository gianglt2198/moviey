use chrono::Utc;
use redis::aio::ConnectionManager;
use redis::{AsyncCommands, Commands};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::keys::{CacheKeys, CacheTTL};
use crate::dtos::recommendation_dto::RecommendationResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedRecommendations {
    pub recommendations: Vec<RecommendationResponse>,
    pub generated_at: String,
    pub expires_at: String,
    pub hit_count: i32,
}

pub struct RecommendationCache {
    redis: ConnectionManager,
}

impl RecommendationCache {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    /// Get cached recommendations (with hit tracking)
    pub async fn get_recommendations(
        &mut self,
        profile_id: Uuid,
    ) -> Result<Option<CachedRecommendations>, redis::RedisError> {
        let key = CacheKeys::recommendations(profile_id);

        // Try to get from cache
        let cached: Option<String> = self.redis.get(&key).await?;

        if let Some(json_str) = cached {
            // Increment hit counter
            let _: () = self
                .redis
                .incr(format!("{}:hits", key), 1)
                .await
                .ok()
                .unwrap_or_default();

            let mut rec: CachedRecommendations = serde_json::from_str(&json_str).map_err(|_| {
                redis::RedisError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Failed to deserialize",
                ))
            })?;

            rec.hit_count += 1;
            return Ok(Some(rec));
        }

        Ok(None)
    }

    /// Set recommendations in cache with TTL
    pub async fn set_recommendations(
        &mut self,
        profile_id: Uuid,
        recommendations: Vec<RecommendationResponse>,
    ) -> Result<(), redis::RedisError> {
        let key = CacheKeys::recommendations(profile_id);
        let ttl = CacheTTL::recommendations();

        let cached = CachedRecommendations {
            recommendations,
            generated_at: Utc::now().to_rfc3339(),
            expires_at: (Utc::now() + chrono::Duration::from_std(ttl).unwrap()).to_rfc3339(),
            hit_count: 0,
        };

        let json = serde_json::to_string(&cached).map_err(|e| {
            redis::RedisError::from(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Serialization failed",
            ))
        })?;

        let _: () = self.redis.set_ex(&key, json, ttl.as_secs()).await?;

        Ok(())
    }

    /// Check if recommendations are cached and valid
    pub async fn is_cached_and_valid(
        &mut self,
        profile_id: Uuid,
    ) -> Result<bool, redis::RedisError> {
        let key = CacheKeys::recommendations(profile_id);
        let exists: bool = self.redis.exists(&key).await?;
        Ok(exists)
    }

    /// Get cache statistics
    pub async fn get_cache_stats(&mut self) -> Result<CacheStats, redis::RedisError> {
        let stats_key = CacheKeys::cache_metrics();

        let hits: i64 = self.redis.hget(&stats_key, "hits").await.unwrap_or(0);
        let misses: i64 = self.redis.hget(&stats_key, "misses").await.unwrap_or(0);
        let invalidations: i64 = self
            .redis
            .hget(&stats_key, "invalidations")
            .await
            .unwrap_or(0);

        let hit_rate = if hits + misses == 0 {
            0.0
        } else {
            (hits as f64 / (hits + misses) as f64) * 100.0
        };

        Ok(CacheStats {
            hits,
            misses,
            invalidations,
            hit_rate,
            total_requests: hits + misses,
        })
    }

    /// Increment cache metrics
    pub async fn increment_hit(&mut self) -> Result<(), redis::RedisError> {
        let stats_key = CacheKeys::cache_metrics();
        let _: () = self.redis.hincr(stats_key, "hits", 1).await?;
        Ok(())
    }

    pub async fn increment_miss(&mut self) -> Result<(), redis::RedisError> {
        let stats_key = CacheKeys::cache_metrics();
        let _: () = self.redis.hincr(stats_key, "misses", 1).await?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: i64,
    pub misses: i64,
    pub invalidations: i64,
    pub hit_rate: f64,
    pub total_requests: i64,
}

/// Movie similarity cache
pub struct MovieSimilarityCache {
    redis: ConnectionManager,
}

impl MovieSimilarityCache {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    pub async fn get(
        &mut self,
        movie_id: Uuid,
    ) -> Result<Option<Vec<(Uuid, f64)>>, redis::RedisError> {
        let key = CacheKeys::movie_similarity(movie_id);
        let json: Option<String> = self.redis.get(&key).await?;

        if let Some(json_str) = json {
            let similar: Vec<(Uuid, f64)> = serde_json::from_str(&json_str).map_err(|_| {
                redis::RedisError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Deserialization failed",
                ))
            })?;
            return Ok(Some(similar));
        }

        Ok(None)
    }

    pub async fn set(
        &mut self,
        movie_id: Uuid,
        similar_movies: Vec<(Uuid, f64)>,
    ) -> Result<(), redis::RedisError> {
        let key = CacheKeys::movie_similarity(movie_id);
        let ttl = CacheTTL::movie_similarity();

        let json = serde_json::to_string(&similar_movies).map_err(|_| {
            redis::RedisError::from(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Serialization failed",
            ))
        })?;

        let _: () = self.redis.set_ex(&key, json, ttl.as_secs()).await?;

        Ok(())
    }
}

/// User preferences cache
pub struct UserPreferencesCache {
    redis: ConnectionManager,
}

impl UserPreferencesCache {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    pub async fn get(
        &mut self,
        profile_id: Uuid,
    ) -> Result<Option<(Vec<(String, f64)>, Vec<(String, f64)>)>, redis::RedisError> {
        let key = CacheKeys::user_preferences(profile_id);
        let json: Option<String> = self.redis.get(&key).await?;

        if let Some(json_str) = json {
            let prefs: (Vec<(String, f64)>, Vec<(String, f64)>) = serde_json::from_str(&json_str)
                .map_err(|_| {
                redis::RedisError::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Deserialization failed",
                ))
            })?;
            return Ok(Some(prefs));
        }

        Ok(None)
    }

    pub async fn set(
        &mut self,
        profile_id: Uuid,
        preferences: (Vec<(String, f64)>, Vec<(String, f64)>),
    ) -> Result<(), redis::RedisError> {
        let key = CacheKeys::user_preferences(profile_id);
        let ttl = CacheTTL::user_preferences();

        let json = serde_json::to_string(&preferences).map_err(|e| {
            redis::RedisError::from(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Serialization failed",
            ))
        })?;

        let _: () = self.redis.set_ex(&key, json, ttl.as_secs()).await?;

        Ok(())
    }
}
