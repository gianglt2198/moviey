use redis::aio::ConnectionManager;
use uuid::Uuid;

use crate::services::cache::keys::CacheKeys;

use super::{
    entries::{CacheDefault, CachedMovieSimilarity, CachedRecommendations, CachedUserPreferences},
    errors::CacheError,
    generic_cache::{CacheEntry, GenericCache},
    keys::CacheTTL,
};

pub struct CacheManager {
    pub redis: ConnectionManager,
}

impl CacheManager {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    pub async fn get_recommendations(
        &self,
        profile_id: Uuid,
    ) -> Result<Option<CachedRecommendations>, CacheError> {
        let key = CacheKeys::recommendations(profile_id);
        let mut cache = GenericCache::new(self.redis.clone());
        cache.get(&key).await
    }

    pub async fn set_recommendations(
        &self,
        profile_id: Uuid,
        recommendations: Vec<crate::dtos::RecommendationResponse>,
    ) -> Result<(), CacheError> {
        let val = CachedRecommendations {
            profile_id,
            recommendations,
            generated_at: chrono::Utc::now().to_rfc3339(),
            expires_at: (chrono::Utc::now()
                + chrono::Duration::from_std(CacheTTL::recommendations()).unwrap())
            .to_rfc3339(),
        };

        let mut cache: GenericCache<CachedRecommendations> = GenericCache::new(self.redis.clone());
        cache
            .set(&val.key(), val, CacheTTL::recommendations())
            .await
    }

    // Movie Similarity
    pub async fn _get_movie_similarity(
        &self,
        movie_id: Uuid,
    ) -> Result<Option<CachedMovieSimilarity>, CacheError> {
        let key = CacheKeys::movie_similarity(movie_id);
        let mut cache: GenericCache<CachedMovieSimilarity> = GenericCache::new(self.redis.clone());
        cache.get(&key).await
    }

    pub async fn _set_movie_similarity(
        &self,
        movie_id: Uuid,
        similar_movies: Vec<(Uuid, f64)>,
    ) -> Result<(), CacheError> {
        let val = CachedMovieSimilarity {
            movie_id,
            similar_movies,
        };

        let mut cache = GenericCache::new(self.redis.clone());
        cache
            .set(&val.key(), val, CacheTTL::movie_similarity())
            .await
    }

    // User Preferences
    pub async fn _get_user_preferences(
        &self,
        profile_id: Uuid,
    ) -> Result<Option<CachedUserPreferences>, CacheError> {
        let key = CacheKeys::user_preferences(profile_id);
        let mut cache = GenericCache::new(self.redis.clone());
        cache.get(&key).await
    }

    pub async fn _set_user_preferences(
        &self,
        profile_id: Uuid,
        genre_preferences: Vec<(String, f64)>,
        director_preferences: Vec<(String, f64)>,
    ) -> Result<(), CacheError> {
        let val = CachedUserPreferences {
            profile_id,
            genre_preferences,
            director_preferences,
        };
        let mut cache = GenericCache::new(self.redis.clone());
        cache
            .set(&val.key(), val, CacheTTL::user_preferences())
            .await
    }

    // Batch operations
    pub async fn _invalidate_profile_cache(&self, profile_id: Uuid) -> Result<(), CacheError> {
        let mut cache = GenericCache::<CacheDefault>::new(self.redis.clone());

        let keys = vec![
            CacheKeys::recommendations(profile_id),
            CacheKeys::user_preferences(profile_id),
        ];

        for key in keys {
            cache.delete(&key).await.ok();
        }

        Ok(())
    }
}
