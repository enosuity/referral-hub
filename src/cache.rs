use redis::{Client, AsyncCommands};
use crate::error::AppError;

#[derive(Clone)]
pub struct CacheRepository {
    client: Client,
}

impl CacheRepository {
    pub fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = Client::open(redis_url)
            .map_err(|e| AppError::CacheError(e.to_string()))?;
        Ok(Self { client })
    }

    pub async fn set_key(&self, key: &str, value: &str, expiry_secs: Option<u64>) -> Result<(), AppError> {
        let mut conn = self.client
            .get_async_connection()
            .await
            .map_err(|e| AppError::CacheError(e.to_string()))?;

        match expiry_secs {
            Some(secs) => {
                conn.set_ex(key, value, secs as usize)
                    .await
                    .map_err(|e| AppError::CacheError(e.to_string()))
            }
            None => {
                conn.set(key, value)
                    .await
                    .map_err(|e| AppError::CacheError(e.to_string()))
            }
        }
    }

    pub async fn get_key(&self, key: &str) -> Result<Option<String>, AppError> {
        let mut conn = self.client
            .get_async_connection()
            .await
            .map_err(|e| AppError::CacheError(e.to_string()))?;

        conn.get(key)
            .await
            .map_err(|e| AppError::CacheError(e.to_string()))
    }
} 