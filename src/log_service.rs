use redis::{Client, Commands, RedisResult};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Utc;

/// Service responsible for logging job events to Redis.
pub struct LogService {
    client: Arc<Mutex<Client>>,
}

impl LogService {
    /// Creates a new `LogService`.
    ///
    /// # Arguments
    ///
    /// * `client` - An `Arc` wrapped `Mutex` containing the Redis client.
    ///
    /// # Returns
    ///
    /// A new instance of `LogService`.
    pub fn new(client: Arc<Mutex<Client>>) -> Self {
        Self { client }
    }

    /// Logs a message to the specified queue's log.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to log the message to.
    /// * `message` - The message to log.
    ///
    /// # Returns
    ///
    /// A `RedisResult` indicating the success or failure of the operation.
    pub async fn log(&self, queue_name: &str, message: &str) -> RedisResult<()> {
        let log_queue_name = format!("{}:log", queue_name);
        let log_entry = format!("{} - {}", Utc::now().to_rfc3339(), message);
        let mut conn = self.client.lock().await.get_connection()?;
        let _: () = conn.lpush(log_queue_name, log_entry)?;
        Ok(())
    }
}
