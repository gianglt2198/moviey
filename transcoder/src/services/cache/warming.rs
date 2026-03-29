use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::keys::CacheKeys;
use super::store::RecommendationCache;
use crate::services::hybrid_recommender::HybridRecommender;

pub struct CacheWarmer {
    redis: ConnectionManager,
    pool: Arc<PgPool>,
    rec_cache: RecommendationCache,
}

impl CacheWarmer {
    pub fn new(redis: ConnectionManager, pool: Arc<PgPool>) -> Self {
        let rec_cache = RecommendationCache::new(redis.clone());
        Self {
            redis,
            pool,
            rec_cache,
        }
    }

    /// Get active users (watched something in last 30 days)
    pub async fn get_active_users(&self) -> Result<Vec<Uuid>, sqlx::Error> {
        sqlx::query_scalar::<_, Uuid>(
            r#"SELECT DISTINCT profile_id 
               FROM watch_history 
               WHERE watched_at > NOW() - INTERVAL '30 days'
               ORDER BY COUNT(*) DESC
               LIMIT 1000"#,
        )
        .fetch_all(self.pool.as_ref())
        .await
    }

    /// Warm cache for active users (pre-calculate recommendations)
    pub async fn warm_active_users(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔥 Starting cache warming for active users...");

        let active_users = self.get_active_users().await?;
        let total = active_users.len();

        println!("📊 Found {} active users", total);

        let recommender = HybridRecommender::default();

        for (idx, profile_id) in active_users.iter().enumerate() {
            match self.warm_user(profile_id, &recommender).await {
                Ok(_) => {
                    if (idx + 1) % 100 == 0 {
                        println!("✅ Warmed cache for {}/{} users", idx + 1, total);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error warming cache for user {}: {:?}", profile_id, e);
                }
            }
        }

        println!("✨ Cache warming complete!");
        Ok(())
    }

    /// Warm cache for a single user
    pub async fn warm_user(
        &mut self,
        profile_id: &Uuid,
        recommender: &HybridRecommender,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Generate recommendations
        let recommendations = recommender
            .generate_recommendation(self.pool.as_ref(), *profile_id, 10, 0.3)
            .await?;

        // Convert to response format
        let responses: Vec<_> = recommendations
            .iter()
            .enumerate()
            .map(|(idx, (movie_id, score, reason))| {
                // In production, fetch actual movie details
                crate::dtos::recommendation_dto::RecommendationResponse {
                    movie_id: *movie_id,
                    title: format!("Movie {}", idx),
                    score: *score,
                    reason: reason.clone(),
                    rank: (idx + 1) as i32,
                    thumbnail_url: format!(
                        "http://localhost:3000/streams/{}/thumbnail.jpg",
                        movie_id
                    ),
                    genre: None,
                }
            })
            .collect();

        // Cache them
        self.rec_cache
            .set_recommendations(*profile_id, responses)
            .await?;

        // Mark user as warmed
        let _: () = self
            .redis
            .set_ex(CacheKeys::warming_status(*profile_id), "completed", 300)
            .await?;

        Ok(())
    }

    /// Lazy warm cache (warm on-demand for new users)
    pub async fn warm_on_demand(
        &mut self,
        profile_id: Uuid,
        recommender: &HybridRecommender,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Check if already warming
        let warming: Option<String> = self
            .redis
            .get(CacheKeys::warming_status(profile_id))
            .await?;

        if warming.is_some() {
            // Already warming, skip
            return Ok(());
        }

        // Mark as warming
        let _: () = self
            .redis
            .set_ex(CacheKeys::warming_status(profile_id), "in_progress", 60)
            .await?;

        // Warm it
        self.warm_user(&profile_id, recommender).await?;

        Ok(())
    }
}
