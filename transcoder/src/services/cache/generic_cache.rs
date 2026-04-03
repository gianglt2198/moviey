use redis::{AsyncCommands, MSetOptions, aio::ConnectionManager};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData, time::Duration};

use super::errors::CacheError;

pub trait CacheEntry: Serialize + for<'de> Deserialize<'de> + Debug + Clone + Send + Sync {
    fn key(&self) -> String;
}

pub struct GenericCache<T: CacheEntry> {
    redis: ConnectionManager,
    _phantom: PhantomData<T>,
}

impl<T: CacheEntry> GenericCache<T> {
    pub fn new(redis: ConnectionManager) -> Self {
        Self {
            redis,
            _phantom: PhantomData,
        }
    }

    pub async fn get(&mut self, key: &str) -> Result<Option<T>, CacheError> {
        let json: Option<String> = self
            .redis
            .get(key)
            .await
            .map_err(|e| CacheError::Redis(e.to_string()))?;

        match json {
            Some(json_str) => {
                let _: () = self
                    .redis
                    .incr(format!("{}:hits", key), 1)
                    .await
                    .ok()
                    .unwrap_or_default();

                let data = serde_json::from_str::<T>(&json_str)
                    .map_err(|e| CacheError::Serialization(e.to_string()))?;

                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    pub async fn set(
        &mut self,
        key: &str,
        val: T,
        ttl_seconds: Duration,
    ) -> Result<(), CacheError> {
        let json =
            serde_json::to_string(&val).map_err(|e| CacheError::Serialization(e.to_string()))?;

        let _: () = self
            .redis
            .set_ex(&key, json, ttl_seconds.as_secs())
            .await
            .map_err(|e| CacheError::Redis(e.to_string()))?;

        Ok(())
    }

    pub async fn _exists(&mut self, key: &str) -> Result<bool, CacheError> {
        self.redis
            .exists(key)
            .await
            .map_err(|e| CacheError::Redis(e.to_string()))
    }

    pub async fn delete(&mut self, key: &str) -> Result<(), CacheError> {
        self.redis
            .del(key)
            .await
            .map_err(|e| CacheError::Redis(e.to_string()))
    }

    pub async fn _get_many(&mut self, keys: &[&str]) -> Result<Vec<Option<T>>, CacheError> {
        let values: Vec<Option<String>> = self
            .redis
            .mget(keys)
            .await
            .map_err(|e| CacheError::Redis(e.to_string()))?;

        values
            .into_iter()
            .map(|opt_json| {
                opt_json
                    .map(|json_str| {
                        serde_json::from_str::<T>(&json_str)
                            .map_err(|e| CacheError::Redis(e.to_string()))
                    })
                    .transpose()
            })
            .collect()
    }

    pub async fn set_many(
        &mut self,
        items: Vec<(&str, T)>,
        ttl: Duration,
    ) -> Result<(), CacheError> {
        let mut values: Vec<(&str, String)> = Vec::new();

        for (k, v) in items {
            let json =
                serde_json::to_string(&v).map_err(|e| CacheError::Serialization(e.to_string()))?;
            values.push((k, json));
        }

        let _: () = self
            .redis
            .mset_ex(
                &values,
                MSetOptions::default().with_expiration(redis::SetExpiry::EX(ttl.as_secs())),
            )
            .await
            .map_err(|e| CacheError::Redis(e.to_string()))?;

        Ok(())
    }

    /// Increment counter for metrics  
    pub async fn increment_counter(
        &mut self,
        key: &str,
        increment: i64,
    ) -> Result<i64, CacheError> {
        self.redis
            .incr(key, increment)
            .await
            .map_err(|e| CacheError::Redis(e.to_string()))
    }
}
