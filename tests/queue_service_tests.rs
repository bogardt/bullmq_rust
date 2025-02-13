use bullmq_rust::job_model::JobData;
use bullmq_rust::QueueServiceTrait;
use mockall::predicate::*;
use chrono::Utc;
mod mocks;
use mocks::mocks::MockQueueService;

#[tokio::test]
async fn test_add_and_get_job() {
    let mut mock_queue_service = MockQueueService::new();

    let job = JobData {
        id: "test_job".to_string(),
        message: "Test Job".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        priority: Some(1),
        delay: Some(0),
        retries: Some(3),
        expires_in: None,
        progress: Some(0),
    };

    // Successful case
    mock_queue_service
        .expect_add_job()
        .with(eq("testQueue"), eq(job.clone()))
        .times(1)
        .returning(|_, _| Ok(()));
    mock_queue_service
        .expect_get_next_job()
        .with(eq("testQueue"))
        .times(1)
        .returning({
            let job = job.clone();
            move |_| Ok(Some(serde_json::to_string(&job).unwrap()))
        });

    mock_queue_service
        .add_job("testQueue", job.clone())
        .await
        .unwrap();
    let fetched_job = mock_queue_service.get_next_job("testQueue").await.unwrap();
    assert!(fetched_job.is_some());
    let fetched_job: JobData = serde_json::from_str(&fetched_job.unwrap()).unwrap();
    assert_eq!(fetched_job.message, job.message);

    // Failing case
    mock_queue_service
        .expect_add_job()
        .with(eq("testQueue"), eq(job.clone()))
        .times(1)
        .returning(|_, _| {
            Err(redis::RedisError::from((
                redis::ErrorKind::IoError,
                "Failed to add job",
            )))
        });
    mock_queue_service
        .expect_get_next_job()
        .with(eq("testQueue"))
        .times(1)
        .returning(|_| {
            Err(redis::RedisError::from((
                redis::ErrorKind::IoError,
                "Failed to get job",
            )))
        });

    assert!(mock_queue_service
        .add_job("testQueue", job.clone())
        .await
        .is_err());
    assert!(mock_queue_service.get_next_job("testQueue").await.is_err());
}

#[tokio::test]
async fn test_count_jobs() {
    let mut mock_queue_service = MockQueueService::new();

    // Successful case
    mock_queue_service
        .expect_count_jobs()
        .with(eq("testQueue"))
        .times(1)
        .returning(|_| Ok(1));

    let count = mock_queue_service.count_jobs("testQueue").await.unwrap();
    assert_eq!(count, 1);

    // Failing case
    mock_queue_service
        .expect_count_jobs()
        .with(eq("testQueue"))
        .times(1)
        .returning(|_| {
            Err(redis::RedisError::from((
                redis::ErrorKind::IoError,
                "Failed to count jobs",
            )))
        });

    assert!(mock_queue_service.count_jobs("testQueue").await.is_err());
}

#[tokio::test]
async fn test_update_and_get_job_progress() {
    let mut mock_queue_service = MockQueueService::new();
    let job_id = "test_job";
    let queue_name = "testQueue";

    // Successful case
    mock_queue_service
        .expect_update_job_progress()
        .with(eq(queue_name), eq(job_id), eq(50))
        .times(1)
        .returning(|_, _, _| Ok(()));
    mock_queue_service
        .expect_get_job_progress()
        .with(eq(queue_name), eq(job_id))
        .times(1)
        .returning(|_, _| Ok(50));

    mock_queue_service
        .update_job_progress(queue_name, job_id, 50)
        .await
        .unwrap();
    let progress = mock_queue_service
        .get_job_progress(queue_name, job_id)
        .await
        .unwrap();
    assert_eq!(progress, 50);

    // Failing case
    mock_queue_service
        .expect_update_job_progress()
        .with(eq(queue_name), eq(job_id), eq(50))
        .times(1)
        .returning(|_, _, _| {
            Err(redis::RedisError::from((
                redis::ErrorKind::IoError,
                "Failed to update job progress",
            )))
        });
    mock_queue_service
        .expect_get_job_progress()
        .with(eq(queue_name), eq(job_id))
        .times(1)
        .returning(|_, _| {
            Err(redis::RedisError::from((
                redis::ErrorKind::IoError,
                "Failed to get job progress",
            )))
        });

    assert!(mock_queue_service
        .update_job_progress(queue_name, job_id, 50)
        .await
        .is_err());
    assert!(mock_queue_service
        .get_job_progress(queue_name, job_id)
        .await
        .is_err());
}

#[tokio::test]
async fn test_move_to_failed() {
    let mut mock_queue_service = MockQueueService::new();
    let job = JobData {
        id: "Failed Job".to_string(),
        message: "Failed Job".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        priority: Some(1),
        delay: Some(0),
        retries: Some(3),
        expires_in: None,
        progress: Some(0),
    };

    // Successful case
    mock_queue_service
        .expect_move_to_failed()
        .with(eq("testQueue"), eq(job.clone()))
        .times(1)
        .returning(|_, _| Ok(()));

    mock_queue_service
        .move_to_failed("testQueue", job.clone())
        .await
        .unwrap();

    // Failing case
    mock_queue_service
        .expect_move_to_failed()
        .with(eq("testQueue"), eq(job.clone()))
        .times(1)
        .returning(|_, _| {
            Err(redis::RedisError::from((
                redis::ErrorKind::IoError,
                "Failed to move job to failed",
            )))
        });

    assert!(mock_queue_service
        .move_to_failed("testQueue", job.clone())
        .await
        .is_err());
}

#[tokio::test]
async fn test_log_job_status() {
    let mut mock_queue_service = MockQueueService::new();
    let job = JobData {
        id: "Log Job".to_string(),
        message: "Log Job".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        priority: Some(1),
        delay: Some(0),
        retries: Some(3),
        expires_in: None,
        progress: Some(0),
    };

    // Successful case
    mock_queue_service
        .expect_log_job_status()
        .with(eq("testQueue"), eq(job.clone()), eq("completed"))
        .times(1)
        .returning(|_, _, _| Ok(()));

    mock_queue_service
        .log_job_status("testQueue", &job, "completed")
        .await
        .unwrap();

    // Failing case
    mock_queue_service
        .expect_log_job_status()
        .with(eq("testQueue"), eq(job.clone()), eq("completed"))
        .times(1)
        .returning(|_, _, _| {
            Err(redis::RedisError::from((
                redis::ErrorKind::IoError,
                "Failed to log job status",
            )))
        });

    assert!(mock_queue_service
        .log_job_status("testQueue", &job, "completed")
        .await
        .is_err());
}

#[tokio::test]
async fn test_add_job() {
    let mut mock_queue_service = MockQueueService::new();

    let job = JobData {
        id: "test".to_string(),
        message: "test".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        priority: Some(1),
        delay: Some(5),
        retries: Some(3),
        expires_in: None,
        progress: Some(0),
    };

    // Définir l'attente pour la méthode add_job
    mock_queue_service
        .expect_add_job()
        .with(eq("testQueue"), eq(job.clone()))
        .times(1)
        .returning(|_, _| Ok(()));

    let result = mock_queue_service.add_job("testQueue", job).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_next_job() {
    let mut mock_queue_service = MockQueueService::new();
    let timestamp = Utc::now().to_rfc3339();
    let job = JobData {
        id: "test".to_string(),
        message: "test".to_string(),
        timestamp: timestamp.clone(),
        priority: Some(1),
        delay: Some(5),
        retries: Some(3),
        expires_in: None,
        progress: Some(0),
    };

    // Définir l'attente pour la méthode get_next_job
    mock_queue_service
        .expect_get_next_job()
        .with(eq("testQueue"))
        .times(1)
        .returning(move |_| Ok(Some(serde_json::to_string(&job).unwrap())));

    let result = mock_queue_service.get_next_job("testQueue").await;
    let attempt_string = format!("\"id\":\"test\",\"message\":\"test\",\"timestamp\":\"{}\",\"priority\":1,\"delay\":5,\"retries\":3,\"expires_in\":null,\"progress\":0", timestamp.clone());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Some("{".to_owned() + &attempt_string + "}"));
}
