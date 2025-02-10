use dotenv::dotenv;
use std::env;
use redis::{Client, RedisResult};

/// Service responsible for managing Redis configuration.
#[derive(Clone)]
pub struct ConfigService {
    pub redis_url: String,
}

impl ConfigService {
    /// Creates a new `ConfigService`.
    ///
    /// This function loads the environment variables and initializes the Redis URL.
    ///
    /// # Returns
    ///
    /// A new instance of `ConfigService`.
    pub fn new() -> Self {
        dotenv().ok();
        let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        Self { redis_url }
    }

    /// Gets a Redis client.
    ///
    /// # Returns
    ///
    /// A `RedisResult` containing the Redis client.
    pub fn get_client(&self) -> RedisResult<Client> {
        Client::open(self.redis_url.as_str())
    }
}
