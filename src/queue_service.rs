use redis::{Client, Commands, RedisResult};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json;
use chrono::Utc;
use crate::config_service::ConfigService;
use crate::job_model::JobData;

pub struct QueueService {
    client: Arc<Mutex<Client>>,
}

impl QueueService {
    pub fn new(config: &ConfigService) -> RedisResult<Self> {
        let client = config.get_client()?;
        Ok(Self {
            client: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn add_job(&self, queue_name: &str, job: JobData) -> RedisResult<()> {
        let mut conn = self.client.lock().await.get_connection()?;
        let job_json = serde_json::to_string(&job).unwrap();
        let score = Utc::now().timestamp() + job.delay.unwrap_or(0);
        let _: () = conn.zadd(queue_name, job_json, score)?;
        Ok(())
    }

    pub async fn get_next_job(&self, queue_name: &str) -> RedisResult<Option<String>> {
        let mut conn = self.client.lock().await.get_connection()?;
        let job: Option<(String, i64)> = conn.zpopmin(queue_name, 1)?;
        Ok(job.map(|(job, _)| job))
    }

    pub async fn count_jobs(&self, queue_name: &str) -> RedisResult<u64> {
        let mut conn = self.client.lock().await.get_connection()?;
        let count: u64 = conn.zcard(queue_name)?;
        Ok(count)
    }

    pub async fn move_to_failed(&self, queue_name: &str, job: JobData) -> RedisResult<()> {
        let failed_queue_name = format!("{}:failed", queue_name);
        self.add_job(&failed_queue_name, job).await
    }

    pub async fn log_job_status(&self, queue_name: &str, job: &JobData, status: &str) -> RedisResult<()> {
        let log_queue_name = format!("{}:log", queue_name);
        let log_entry = format!("{} - {}: {}", Utc::now().to_rfc3339(), status, job.message);
        let mut conn = self.client.lock().await.get_connection()?;
        let _: () = conn.lpush(log_queue_name, log_entry)?;
        Ok(())
    }

    pub async fn update_job_progress(&self, queue_name: &str, job_id: &str, progress: u32) -> RedisResult<()> {
        let mut conn = self.client.lock().await.get_connection()?;
        let job_key = format!("{}:{}", queue_name, job_id);
        let _: () = conn.hset(job_key, "progress", progress)?;
        Ok(())
    }

    pub async fn get_job_progress(&self, queue_name: &str, job_id: &str) -> RedisResult<u32> {
        let mut conn = self.client.lock().await.get_connection()?;
        let job_key = format!("{}:{}", queue_name, job_id);
        let progress: u32 = conn.hget(job_key, "progress")?;
        Ok(progress)
    }
}
