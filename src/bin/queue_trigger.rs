use bullmq_rust::queue_trigger_service::QueueTriggerService;
use redis::RedisResult;

#[tokio::main]
async fn main() -> RedisResult<()> {
    let queue_name = "my_queue";

    // Create queue trigger service
    let queue_trigger = QueueTriggerService::new(queue_name.to_string());

    // Start queue trigger to monitor and process jobs
    tokio::spawn(async move {
        queue_trigger.start(200).await;
    });

    // Keep the main function alive
    // sleep(Duration::from_secs(60)).await;

    while true {}

    Ok(())
}
