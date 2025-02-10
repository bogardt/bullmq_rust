use redis::{Client, Commands, RedisResult};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;

pub struct LogService {
    client: Arc<Mutex<Client>>,
}

impl LogService {
    pub fn new(client: Arc<Mutex<Client>>) -> Self {
        Self { client }
    }

    pub async fn log(&self, queue_name: &str, message: &str) -> RedisResult<()> {
        let log_queue_name = format!("{}:log", queue_name);
        let log_entry = format!("{} - {}", Utc::now().to_rfc3339(), message);
        let mut conn = self.client.lock().await.get_connection()?;
        let _: () = conn.lpush(log_queue_name, log_entry)?;
        Ok(())
    }
}
