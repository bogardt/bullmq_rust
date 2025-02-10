use dotenv::dotenv;
use std::env;
use redis::{Client, RedisResult};

#[derive(Clone)]
pub struct ConfigService {
    pub redis_url: String,
}

impl ConfigService {
    pub fn new() -> Self {
        dotenv().ok();
        let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        Self { redis_url }
    }

    pub fn get_client(&self) -> RedisResult<Client> {
        Client::open(self.redis_url.as_str())
    }
}
