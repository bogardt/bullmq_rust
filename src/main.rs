use bullmq_rust::config_service::ConfigService;
use bullmq_rust::job_model::JobData;
use bullmq_rust::queue_service::RedisQueueService;
use bullmq_rust::worker_service::WorkerService;
use bullmq_rust::QueueServiceTrait;
use chrono::Utc;
use redis::RedisResult;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// This function initializes the configuration, queue service, and worker service.
/// It adds a job to the queue and starts the worker to process jobs.
#[tokio::main]
async fn main() -> RedisResult<()> {
    // Initialize the configuration service
    let config = ConfigService::new();
    let redis_client = Arc::new(tokio::sync::Mutex::new(config.get_client()?));

    // Create a new queue service instance
    let redis_service =
        Arc::new(RedisQueueService::new(redis_client)) as Arc<dyn QueueServiceTrait>;

    let queues = vec!["queue1", "queue2", "queue3"];

    // Create jobs and add them to the queues
    let mut job_ids = vec![];
    for i in 0..10 {
        let queue_name = queues[i % 3];
        let job_id = format!("job_{}", i);
        let job = JobData {
            message: job_id.clone(),
            timestamp: Utc::now().to_rfc3339(),
            priority: Some(i as i32 % 5),
            delay: Some(i as i64 % 10),
            retries: Some(3),
            expires_in: Some(Utc::now().timestamp() + 60),
            progress: Some(0),
        };

        if let Err(e) = redis_service.add_job(queue_name, job).await {
            eprintln!("Failed to add job to {}: {}", queue_name, e);
        } else {
            eprintln!("Succeeded to add job to {}", queue_name);
            job_ids.push((queue_name.to_string(), job_id));
        }
    }

    // Create worker services for each queue
    let mut workers = vec![];
    for queue_name in &queues {
        let worker = WorkerService::new(queue_name.to_string(), redis_service.clone());
        workers.push(worker);
    }

    // Start workers to process jobs
    for worker in workers {
        tokio::spawn(async move {
            worker.start().await;
            worker.retry_failed_jobs().await;
        });
    }

    // Simulate tracking job progress
    sleep(Duration::from_secs(10)).await;
    for (queue_name, job_id) in &job_ids {
        match &redis_service.get_job_progress(queue_name, job_id).await {
            Ok(progress) => println!("Job progress in {}: {}", queue_name, progress),
            Err(e) => eprintln!(
                "Failed to get job progress in {}: {} - {:?}",
                queue_name, job_id, e
            ), // Debug statement
        }
    }

    Ok(())
}
