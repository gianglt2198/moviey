use std::{sync::Arc, time::Duration};

use chrono::Utc;
use sqlx::PgPool;
use tokio::spawn;
use tokio_schedule::{Job, every};

use super::jobs::BatchJobs;
use crate::config::redis::RedisPool;

// TODO: swtich to asynq or similar library for better job management and monitoring
pub struct BatchJobRunner {
    pool: Arc<PgPool>,
    redis: Arc<RedisPool>,
}

impl BatchJobRunner {
    pub fn new(pool: Arc<PgPool>, redis: Arc<RedisPool>) -> Self {
        Self { pool, redis }
    }

    pub async fn start(&self) {
        // Daily 2 AM UTC - User embeddings
        let user_embeddings_job = {
            let pool = self.pool.clone();
            every(1)
                .days()
                .at(02, 00, 00)
                .in_timezone(&Utc)
                .perform(move || {
                    println!("🔄 Starting user embeddings calculation...");
                    let pool = pool.clone();
                    async move {
                        let _ = BatchJobs::calculate_user_embeddings(pool).await;
                    }
                })
        };
        spawn(user_embeddings_job);

        // Daily 2:30 AM UTC - Movie similarities
        let movie_similarities_job = {
            let pool = self.pool.clone();
            every(1)
                .days()
                .at(02, 30, 00)
                .in_timezone(&Utc)
                .perform(move || {
                    println!("🔄 Starting movie similarities calculation...");
                    let pool = pool.clone();
                    async move {
                        let _ = BatchJobs::precalculate_movie_similarities(pool).await;
                    }
                })
        };
        spawn(movie_similarities_job);

        // Run cache warming daily at 3 AM UTC
        let cache_warming_job = {
            let pool = self.pool.clone();
            let redis = self.redis.clone();
            every(1)
                .days()
                .at(03, 00, 00)
                .in_timezone(&Utc)
                .perform(move || {
                    let pool = pool.clone();
                    let redis = redis.clone();
                    async move {
                        let _ = BatchJobs::warm_recommendation_cache(pool, redis).await;
                    }
                })
        };
        spawn(cache_warming_job);

        // Weekly on Sunday 3 AM UTC - Recalculate user segments
        let recalculate_segments_job = {
            let pool = self.pool.clone();
            every(1)
                .weeks()
                .at(03, 00, 00)
                .in_timezone(&Utc)
                .perform(move || {
                    println!("🔄 Starting user segments recalculation...");
                    let pool = pool.clone();
                    async move {
                        let _ = BatchJobs::recalculate_user_segments(pool).await;
                    }
                })
        };
        spawn(recalculate_segments_job);

        // Daily 4 AM UTC - Data quality checks
        let data_quality_checks_job = {
            let pool = self.pool.clone();
            every(1)
                .days()
                .at(04, 00, 00)
                .in_timezone(&Utc)
                .perform(move || {
                    println!("🔄 Starting data quality checks...");
                    let pool = pool.clone();
                    async move {
                        let _ = BatchJobs::run_data_quality_checks(pool).await;
                    }
                })
        };
        spawn(data_quality_checks_job);

        // Keep running
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }
}
