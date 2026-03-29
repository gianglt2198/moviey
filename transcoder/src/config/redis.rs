use redis::Client;
use redis::aio::ConnectionManager;
use std::time::Duration;

pub struct RedisConfig {
    pub host: String,
    pub port: u16,
    pub db: i64,
    pub max_pool_size: usize,
    pub connection_timeout: Duration,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            host: std::env::var("REDIS_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("REDIS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(6379),
            db: std::env::var("REDIS_DB")
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(0),
            max_pool_size: 20,
            connection_timeout: Duration::from_secs(5),
        }
    }
}

pub struct RedisPool {
    pub manager: ConnectionManager,
}

impl RedisPool {
    pub async fn new(config: RedisConfig) -> Result<Self, redis::RedisError> {
        let redis_url = format!("redis://{}:{}/{}", config.host, config.port, config.db);

        let client = Client::open(redis_url)?;
        let manager = ConnectionManager::new(client).await?;

        println!("✅ Redis connected: {}:{}", config.host, config.port);

        Ok(Self { manager })
    }

    pub fn get_connection(&self) -> ConnectionManager {
        self.manager.clone()
    }
}
