use crate::config::redis::RedisPool;
use crate::services::cache::warming::CacheWarmer;
use sqlx::PgPool;
use std::sync::Arc;

// #[derive(Clone)]
// pub struct Jobs {
//     pool: Arc<PgPool>,
//     redis: Arc<RedisPool>,
// }

// impl Jobs {
//     pub fn new(pool: Arc<PgPool>, redis: Arc<RedisPool>) -> Self {
//         Self { pool, redis }
//     }

//     pub async fn cache_warming_job(&self) {
//         println!("⏰ Cache warming job started");

//         let mut warmer = CacheWarmer::new(self.redis.get_connection(), self.pool.clone());

//         match warmer.warm_active_users().await {
//             Ok(_) => println!("✅ Cache warming completed successfully"),
//             Err(e) => eprintln!("❌ Cache warming failed: {:?}", e),
//         }
//     }
// }

pub async fn cache_warming_job(pool: Arc<PgPool>, redis: Arc<RedisPool>) {
    println!("⏰ Cache warming job started");

    let mut warmer = CacheWarmer::new(redis.get_connection(), pool.clone());

    match warmer.warm_active_users().await {
        Ok(_) => println!("✅ Cache warming completed successfully"),
        Err(e) => eprintln!("❌ Cache warming failed: {:?}", e),
    }
}
