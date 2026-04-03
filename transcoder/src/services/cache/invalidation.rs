use redis::AsyncCommands;
use redis::aio::ConnectionManager;
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

        println!(
            "🔄 Invalidated cache for user {} (watched movie {})",
            profile_id, movie_id
        );

        Ok(())
    }
}
