use async_trait::async_trait;
use job_model::JobData;
use redis::RedisResult;

/// Module for managing Redis configuration.
pub mod config_service;
/// Module for managing queues and jobs.
pub mod queue_service;
/// Module for defining the job data model.
pub mod job_model;
/// Module for logging job events.
pub mod log_service;
/// Module for queue trigger service
pub mod queue_trigger_service;


#[async_trait]
pub trait QueueServiceTrait: Send + Sync {
    async fn add_job(&mut self, queue_name: &str, job: JobData) -> RedisResult<()>;
    async fn get_next_job(&mut self, queue_name: &str) -> RedisResult<Option<String>>;
    async fn count_jobs(&mut self, queue_name: &str) -> RedisResult<u64>;
    async fn update_job_progress(&mut self, queue_name: &str, job_id: &str, progress: u32) -> RedisResult<()>;
    async fn get_job_progress(&mut self, queue_name: &str, job_id: &str) -> RedisResult<u32>;
    async fn move_to_failed(&mut self, queue_name: &str, job: JobData) -> RedisResult<()>;
    async fn log_job_status(&mut self, queue_name: &str, job: &JobData, status: &str) -> RedisResult<()>;
}
