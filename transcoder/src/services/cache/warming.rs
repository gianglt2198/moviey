use redis::AsyncCommands;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use super::{keys::CacheKeys, store::CacheManager};
use crate::dtos::recommendation_dto::RecommendationResponse;
use crate::services::hybrid_recommender::HybridRecommender;

#[derive(Debug, Clone)]
pub struct CacheWarmingMetrics {
    pub successful: i32,
    pub failed: i32,
}

impl CacheWarmingMetrics {
    fn new() -> Self {
        Self {
            successful: 0,
            failed: 0,
        }
    }
}

pub struct CacheWarmer {
    redis: ConnectionManager,
    manager: CacheManager,
    pool: Arc<PgPool>,
}

impl CacheWarmer {
    pub fn new(redis: ConnectionManager, pool: Arc<PgPool>) -> Self {
        Self {
            redis: redis.clone(),
            pool,
            manager: CacheManager::new(redis),
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

    pub async fn warm_active_users(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let active_users = self.get_active_users().await?;
        let total = active_users.len();

        let recommender = HybridRecommender::default();
        let mut metrics = CacheWarmingMetrics::new();

        for (i, profile_id) in active_users.into_iter().enumerate() {
            println!("Warming cache for user {}/{}: {}", i + 1, total, profile_id);
            match self.warm_user(profile_id.clone(), &recommender).await {
                Ok(_recs) => {
                    metrics.successful += 1;
                }
                Err(e) => {
                    eprintln!("Failed to warm cache for user {}: {}", profile_id, e);
                    metrics.failed += 1;
                }
            }
        }

        self.store_warming_metrics(metrics).await.ok();

        Ok(())
    }

    pub async fn warm_user(
        &mut self,
        profile_id: Uuid,
        recommender: &HybridRecommender,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Generate recommendations
        let recommendations = recommender
            .generate_recommendation(self.pool.as_ref(), profile_id, 10, 0.3)
            .await?;

        // Convert to response format
        let responses: Vec<RecommendationResponse> = recommendations
            .iter()
            .enumerate()
            .map(|(idx, (movie_id, score, reason))| RecommendationResponse {
                movie_id: *movie_id,
                title: format!("Movie {}", idx),
                score: *score,
                reason: reason.clone(),
                rank: (idx + 1) as i32,
                thumbnail_url: format!("http://localhost:3000/streams/{}/thumbnail.jpg", movie_id),
                genre: None,
            })
            .collect();

        // Use generic cache to set recommendations
        self.manager
            .set_recommendations(profile_id, responses)
            .await?;

        // Mark user as warmed
        let _: () = self
            .redis
            .set_ex(CacheKeys::warming_status(profile_id), "completed", 300)
            .await?;

        Ok(())
    }

    async fn store_warming_metrics(
        &mut self,
        metrics: CacheWarmingMetrics,
    ) -> Result<(), redis::RedisError> {
        let key = "rec:metrics:warming";
        let _: () = self
            .redis
            .hset_multiple(
                key,
                &[
                    ("last_run", chrono::Utc::now().to_rfc3339().as_str()),
                    ("successful", metrics.successful.to_string().as_str()),
                    ("failed", metrics.failed.to_string().as_str()),
                    (
                        "total",
                        (metrics.successful + metrics.failed).to_string().as_str(),
                    ),
                ],
            )
            .await?;
        let _: () = self.redis.expire(key, 86400).await?;
        Ok(())
    }
}
