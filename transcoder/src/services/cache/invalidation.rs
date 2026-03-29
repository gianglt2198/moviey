use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use uuid::Uuid;

use super::keys::CacheKeys;

pub struct CacheInvalidation {
    redis: ConnectionManager,
}

impl CacheInvalidation {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    /// Invalidate user's recommendations when they watch a new movie
    pub async fn on_watch_event(
        &mut self,
        profile_id: Uuid,
        movie_id: Uuid,
    ) -> Result<(), redis::RedisError> {
        let rec_key = CacheKeys::recommendations(profile_id);
        let pref_key = CacheKeys::user_preferences(profile_id);

        // Delete cached recommendations
        let _: () = self.redis.del(&rec_key).await?;

        // Invalidate user preferences (they've changed)
        let _: () = self.redis.del(&pref_key).await?;

        // Log invalidation
        let _: () = self
            .redis
            .hincr(CacheKeys::cache_metrics(), "invalidations", 1)
            .await?;

        println!(
            "🔄 Invalidated cache for user {} (watched movie {})",
            profile_id, movie_id
        );

        Ok(())
    }

    /// Invalidate similar movies cache when a movie is updated
    pub async fn on_movie_update(&mut self, movie_id: Uuid) -> Result<(), redis::RedisError> {
        let key = CacheKeys::movie_similarity(movie_id);
        let _: () = self.redis.del(&key).await?;

        println!("🔄 Invalidated similarity cache for movie {}", movie_id);

        Ok(())
    }

    /// Batch invalidate for multiple users (when system updates)
    pub async fn invalidate_users(
        &mut self,
        profile_ids: Vec<Uuid>,
    ) -> Result<(), redis::RedisError> {
        for profile_id in profile_ids {
            let rec_key = CacheKeys::recommendations(profile_id);
            let pref_key = CacheKeys::user_preferences(profile_id);

            let _: () = self.redis.del(&[rec_key, pref_key]).await?;
        }

        Ok(())
    }

    /// Clear all recommendation caches (system-wide)
    pub async fn clear_all_recommendations(&mut self) -> Result<(), redis::RedisError> {
        // Use SCAN to find all recommendation keys (non-blocking)
        let mut cursor = 0;
        let pattern = "rec:user:*:recommendations";

        loop {
            let (new_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut self.redis.clone())
                .await?;

            for key in keys {
                let _: () = self.redis.del(&key).await?;
            }

            cursor = new_cursor;
            if cursor == 0 {
                break;
            }
        }

        println!("🗑️ Cleared all recommendation caches");

        Ok(())
    }
}
