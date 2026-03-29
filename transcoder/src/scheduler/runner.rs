use std::{sync::Arc, time::Duration};

use chrono::Utc;
use sqlx::PgPool;
use tokio::spawn;
use tokio_schedule::{Job, every};

use crate::{config::redis::RedisPool, scheduler::*};

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
        let user_embeddings_job =
            every(1)
                .days()
                .at(02, 00, 00)
                .in_timezone(&Utc)
                .perform(|| async {
                    println!("🔄 Starting user embeddings calculation...");
                    // self.calculate_user_embeddings().await
                });
        spawn(user_embeddings_job);

        // Daily 2:30 AM UTC - Movie similarities
        let movie_similarities_job =
            every(1)
                .days()
                .at(02, 30, 00)
                .in_timezone(&Utc)
                .perform(|| async {
                    println!("🔄 Starting movie similarity calculation...");
                    // self.precalculate_similarities().await
                });
        spawn(movie_similarities_job);

        // Run cache warming daily at 3 AM UTC
        let cache_warming_job = {
            let jobs = Jobs::new(self.pool.clone(), self.redis.clone());
            every(1)
                .days()
                .at(03, 00, 00)
                .in_timezone(&Utc)
                .perform(|| async {
                    println!("🔄 Starting daily cache warming...");
                })
        };
        spawn(cache_warming_job);

        // Keep running
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }
}
