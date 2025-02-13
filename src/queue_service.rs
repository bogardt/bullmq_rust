use async_trait::async_trait;
use redis::{Commands, RedisResult};
use std::sync::Arc;
use serde_json;
use chrono::Utc;
use crate::config_service::ConfigService;
use crate::job_model::JobData;
use crate::QueueServiceTrait;

/// Service responsible for managing a Redis queue.
pub struct QueueService {
    conn: redis::Connection
}

impl QueueService {
    pub fn new(conn: redis::Connection) -> Self {
        Self { conn }
    }

    pub async fn connect() -> redis::Connection {
        let config = ConfigService::new();
        let redis_client = Arc::new(tokio::sync::Mutex::new(config.get_client().unwrap()));
        let conn = redis_client.lock().await.get_connection().unwrap();
        conn
    }
}

#[async_trait]
impl QueueServiceTrait for QueueService {
    /// Adds a job to the specified queue.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to add the job to.
    /// * `job` - The job data to add to the queue.
    ///
    /// # Returns
    ///
    /// A `RedisResult` indicating the success or failure of the operation.
    async fn add_job(&mut self, queue_name: &str, job: JobData) -> RedisResult<()> {
        let job_json = serde_json::to_string(&job).unwrap();
        let score = Utc::now().timestamp() + job.delay.unwrap_or(0);
        let _: () = self.conn.zadd(queue_name, job_json, score)?;
        Ok(())
    }

    /// Retrieves the next job from the specified queue.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to retrieve the job from.
    ///
    /// # Returns
    ///
    /// A `RedisResult` containing an optional job JSON string.
    async fn get_next_job(&mut self, queue_name: &str) -> RedisResult<Option<String>> {
        let job: Option<(String, i64)> = self.conn.zpopmin(queue_name, 1)?;
        Ok(job.map(|(job, _)| job))
    }

    /// Counts the number of jobs in the specified queue.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to count the jobs in.
    ///
    /// # Returns
    ///
    /// A `RedisResult` containing the number of jobs in the queue.
    async fn count_jobs(&mut self, queue_name: &str) -> RedisResult<u64> {
        let count: u64 = self.conn.zcard(queue_name)?;
        Ok(count)
    }

    /// Moves a job to the failed queue.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue the job belongs to.
    /// * `job` - The job data to move to the failed queue.
    ///
    /// # Returns
    ///
    /// A `RedisResult` indicating the success or failure of the operation.
    async fn move_to_failed(&mut self, queue_name: &str, job: JobData) -> RedisResult<()> {
        let failed_queue_name = format!("{}:failed", queue_name);
        self.add_job(&failed_queue_name, job).await
    }

    /// Logs the status of a job.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue the job belongs to.
    /// * `job` - A reference to the job data.
    /// * `status` - The status message to log.
    ///
    /// # Returns
    ///
    /// A `RedisResult` indicating the success or failure of the operation.
    async fn log_job_status(&mut self, queue_name: &str, job: &JobData, status: &str) -> RedisResult<()> {
        let log_queue_name = format!("{}:log", queue_name);
        let log_entry = format!("{} - {}: {}", Utc::now().to_rfc3339(), status, job.message);
        let _: () = self.conn.lpush(log_queue_name, log_entry)?;
        Ok(())
    }

    /// Updates the progress of a job.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue the job belongs to.
    /// * `job_id` - The ID of the job.
    /// * `progress` - The progress value to update.
    ///
    /// # Returns
    ///
    /// A `RedisResult` indicating the success or failure of the operation.
    async fn update_job_progress(&mut self, queue_name: &str, job_id: &str, progress: u32) -> RedisResult<()> {
        let job_key = format!("{}:{}", queue_name, job_id);
        let _: () = self.conn.hset(job_key, "progress", progress)?;
        Ok(())
    }

    /// Retrieves the progress of a job.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue the job belongs to.
    /// * `job_id` - The ID of the job.
    ///
    /// # Returns
    ///
    /// A `RedisResult` containing the progress value of the job.
    async fn get_job_progress(&mut self, queue_name: &str, job_id: &str) -> RedisResult<u32> {
        let job_key = format!("{}:{}", queue_name, job_id);
        let progress: u32 = self.conn.hget(job_key, "progress")?;
        Ok(progress)
    }
}
