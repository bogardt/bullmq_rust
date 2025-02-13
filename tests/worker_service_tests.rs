// use bullmq_rust::job_model::JobData;
// use bullmq_rust::worker_service::WorkerService;
// use chrono::Utc;
// use std::sync::Arc;
// mod mocks;
// use mocks::MockQueueService;
// use bullmq_rust::QueueServiceTrait;

// #[tokio::test]
// async fn test_worker_service_start() {
//     let mock_queue_service = Arc::new(MockQueueService::new());
//     let worker_service = WorkerService::new("testQueue".to_string(), mock_queue_service.clone() as Arc<dyn QueueServiceTrait>);

//     worker_service.start().await;

//     // Simulate job processing
//     let job = JobData {
//         message: "test".to_string(),
//         timestamp: Utc::now().to_rfc3339(),
//         priority: Some(1),
//         delay: Some(5),
//         retries: Some(3),
//         expires_in: None,
//         progress: Some(0),
//     };

//     let progress = mock_queue_service
//         .get_job_progress("testQueue", "test")
//         .await
//         .unwrap();
//     assert_ne!(progress, 0);
// }

// #[tokio::test]
// async fn test_retry_failed_jobs() {
//     let mock_queue_service = Arc::new(MockQueueService::new());
//     let worker_service = WorkerService::new("testQueue".to_string(), mock_queue_service.clone() as Arc<dyn QueueServiceTrait>);

//     worker_service.retry_failed_jobs().await;

//     // Simulate failed job processing
//     let job = JobData {
//         message: "test".to_string(),
//         timestamp: Utc::now().to_rfc3339(),
//         priority: Some(1),
//         delay: Some(5),
//         retries: Some(3),
//         expires_in: None,
//         progress: Some(0),
//     };

//     mock_queue_service
//         .move_to_failed("testQueue", job.clone())
//         .await
//         .unwrap();
//     tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

//     let result = mock_queue_service.get_next_job("testQueue").await.unwrap();
//     assert_eq!(result, Some("{\"message\":\"test\"}".to_string()));
// }

// #[tokio::test]
// async fn test_worker_service_handle_expired_job() {
//     let mock_queue_service = Arc::new(MockQueueService::new());
//     let worker_service = WorkerService::new("testQueue".to_string(), mock_queue_service.clone());

//     let job = JobData {
//         message: "Expired Job".to_string(),
//         timestamp: Utc::now().to_rfc3339(),
//         priority: Some(1),
//         delay: Some(0),
//         retries: Some(3),
//         expires_in: Some(Utc::now().timestamp() - 10),
//         progress: Some(0),
//     };

//     let mock_queue_service = tokio::sync::Mutex::new(MockQueueService::new());

//     // Handle expired job
//     let mut guard = mock_queue_service.lock().await;
//     guard.expect_get_next_job()
//         .with(mockall::predicate::eq("testQueue"))
//         .times(1)
//         .returning(move |_| Ok(Some(serde_json::to_string(&job).unwrap())));

//     worker_service.start().await;
// }
