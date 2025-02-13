use bullmq_rust::config_service::ConfigService;
use bullmq_rust::queue_service::QueueServcie;
use bullmq_rust::QueueServiceTrait;
use bullmq_rust::queue_trigger_service::QueueTriggerService;
use redis::RedisResult;
use std::sync::Arc;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> RedisResult<()> {
    // Initialize the configuration service
    let config = ConfigService::new();
    let redis_client = Arc::new(tokio::sync::Mutex::new(config.get_client()?));

    // Create a new queue service instance
    let redis_service =
        Arc::new(QueueServcie::new(redis_client)) as Arc<dyn QueueServiceTrait>;

    let queue_name = "my_queue";

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
