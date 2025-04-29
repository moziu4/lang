use redis::RedisError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),

    #[error("JSON deserialization error: {0}")]
    Json(#[from] SerdeError),
}