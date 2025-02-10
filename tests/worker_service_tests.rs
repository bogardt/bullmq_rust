use bullmq_rust::config_service::ConfigService;
use bullmq_rust::queue_service::QueueService;
use bullmq_rust::worker_service::WorkerService;
use bullmq_rust::job_model::JobData;
use chrono::Utc;
use std::sync::Arc;

#[tokio::test]
async fn test_worker_service_start() {
    let config_service = ConfigService::new();
    let queue_service = Arc::new(QueueService::new(&config_service).unwrap());
    let worker_service = WorkerService::new("testQueue".to_string(), Arc::clone(&queue_service));

    let job = JobData {
        message: "Test Job".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        priority: Some(1),
        delay: Some(0),
        retries: Some(3),
        expires_in: None,
        progress: Some(0),
    };

    queue_service.add_job("testQueue", job.clone()).await.unwrap();
    worker_service.start().await;
    let fetched_job = queue_service.get_next_job("testQueue").await.unwrap();
    assert!(fetched_job.is_none());
}
