use bullmq_rust::config_service::ConfigService;
use bullmq_rust::job_model::JobData;
use bullmq_rust::queue_service::QueueServcie;
use bullmq_rust::QueueServiceTrait;
use bullmq_rust::queue_trigger_service::QueueTriggerService;
use chrono::Utc;
use redis::RedisResult;
use std::sync::Arc;
use tokio::time::sleep;
use std::time::Duration;

/// This function initializes the configuration, queue service, and worker service.
/// It adds a job to the queue and starts the worker to process jobs.
#[tokio::main]
async fn main() -> RedisResult<()> {
    // Initialize the configuration service
    let config = ConfigService::new();
    let redis_client = Arc::new(tokio::sync::Mutex::new(config.get_client()?));

    // Create a new queue service instance
    let redis_service =
        Arc::new(QueueServcie::new(redis_client)) as Arc<dyn QueueServiceTrait>;

    let queue_name = "my_queue";

    // Create a job and add it to the queue
    let job_id = "job_1".to_string();
    let job = JobData {
        id: job_id.clone(),
        message: "Hello, world!".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        priority: Some(1),
        delay: Some(0),
        retries: Some(3),
        expires_in: Some(Utc::now().timestamp() + 60),
        progress: Some(0),
    };

    if let Err(e) = redis_service.add_job(queue_name, job).await {
        eprintln!("Failed to add {} to {}: {}", job_id, queue_name, e);
    } else {
        eprintln!("Succeeded to add {} to {}", job_id, queue_name);
    }

    // Create queue trigger service
    let queue_trigger = QueueTriggerService::new(queue_name.to_string(), redis_service.clone());

    // Start queue trigger to monitor and process jobs
    tokio::spawn(async move {
        queue_trigger.start().await;
    });

    // Keep the main function alive
    sleep(Duration::from_secs(60)).await;

    Ok(())
}
