use bullmq_rust::config_service::ConfigService;
use bullmq_rust::queue_service::QueueService;
use bullmq_rust::job_model::JobData;
use chrono::Utc;

#[tokio::test]
async fn test_add_and_get_job() {
    let config_service = ConfigService::new();
    let queue_service = QueueService::new(&config_service).unwrap();
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
    let fetched_job = queue_service.get_next_job("testQueue").await.unwrap();
    assert!(fetched_job.is_some());
    let fetched_job: JobData = serde_json::from_str(&fetched_job.unwrap()).unwrap();
    assert_eq!(fetched_job.message, job.message);
}

#[tokio::test]
async fn test_count_jobs() {
    let config_service = ConfigService::new();
    let queue_service = QueueService::new(&config_service).unwrap();
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
    let count = queue_service.count_jobs("testQueue").await.unwrap();
    assert_eq!(count, 1);
}
