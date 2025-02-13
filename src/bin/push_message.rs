use bullmq_rust::job_model::JobData;
use bullmq_rust::queue_service::QueueService;
use bullmq_rust::QueueServiceTrait;
use chrono::Utc;
use redis::RedisResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DataModel {
    message_type: String,
    message_content: String,
    message_enum: i64
}


#[tokio::main]
async fn main() -> RedisResult<()> {
    // Create a new queue service instance
    let conn = QueueService::connect().await;
    let mut redis_service = QueueService::new(conn);

    let queue_name = "my_queue";
    
    let data_model = DataModel {
        message_type: "TEST".to_string(),
        message_content: "ContentTEST".to_string(),
        message_enum: 3
    };

    // Create a job and add it to the queue
    let job_id = "job_1".to_string();
    let job = JobData {
        id: job_id.clone(),
        message: serde_json::to_string(&data_model).unwrap(),
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

    Ok(())
}
