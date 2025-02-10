use redis::RedisResult;
use std::sync::Arc;
use chrono::Utc;
use bullmq_rust::config_service::ConfigService;
use bullmq_rust::queue_service::QueueService;
use bullmq_rust::worker_service::WorkerService;
use bullmq_rust::job_model::JobData;

/// The main entry point of the application.
///
/// This function initializes the configuration, queue service, and worker service.
/// It adds a job to the queue and starts the worker to process jobs.
#[tokio::main]
async fn main() -> RedisResult<()> {
    let config = ConfigService::new();
    let queue_service = Arc::new(QueueService::new(&config)?);
    
    let job = JobData {
        message: "Hello, Rust!".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        priority: Some(1),
        delay: Some(5),
        retries: Some(3),
        expires_in: Some(Utc::now().timestamp() + 60),
        progress: Some(0),
    };
    
    queue_service.add_job("testQueue", job).await?;
    
    let worker = WorkerService::new("testQueue".to_string(), Arc::clone(&queue_service));
    worker.start().await;
    worker.retry_failed_jobs().await;

    Ok(())
}
