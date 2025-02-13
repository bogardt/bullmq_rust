use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use serde_json;
use chrono::Utc;
use crate::QueueServiceTrait;
use crate::job_model::JobData;

/// Service responsible for managing workers that process jobs from a queue.
pub struct WorkerService {
    queue_name: String,
    queue_service: Arc<dyn QueueServiceTrait>,
}

impl WorkerService {
    /// Creates a new `WorkerService`.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to process jobs from.
    /// * `queue_service` - An `Arc` wrapped `QueueService` instance.
    ///
    /// # Returns
    ///
    /// A new instance of `WorkerService`.
    pub fn new(queue_name: String, queue_service: Arc<dyn QueueServiceTrait>) -> Self {
        Self { queue_name, queue_service }
    }

    /// Starts the worker to process jobs from the queue.
    ///
    /// This function spawns a new asynchronous task that continuously fetches
    /// and processes jobs from the queue. It updates job progress and handles
    /// retries for failed jobs.
    pub async fn start(&self) {
        let queue_name = self.queue_name.clone();
        let queue_service = Arc::clone(&self.queue_service);
        let progress_mutex = Arc::new(Mutex::new(()));

        task::spawn(async move {
            loop {
                if let Ok(Some(job_json)) = queue_service.get_next_job(&queue_name).await {
                    let job: JobData = serde_json::from_str(&job_json).unwrap();
                    println!("Processing job: {} at {}", job.message, job.timestamp);

                    // Simulate job processing with real-time progress updates
                    for progress in (0..=100).step_by(10) {
                        {
                            let _lock = progress_mutex.lock().await;
                            queue_service.update_job_progress(&queue_name, &job.message, progress).await.ok();
                        }
                        println!("Job {} progress: {}%", job.message, progress);
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }

                    if let Some(expiration) = job.expires_in {
                        if Utc::now().timestamp() > expiration {
                            println!("Skipping expired job: {}", job.message);
                            continue;
                        }
                    }

                    if let Some(retries) = job.retries {
                        if retries > 0 {
                            println!("Retrying job {} - Remaining attempts: {}", job.message, retries - 1);
                            let new_job = JobData {
                                retries: Some(retries - 1),
                                ..job.clone()
                            };
                            queue_service.add_job(&queue_name, new_job).await.ok();
                        }
                    }

                    // Finalize job progress
                    queue_service.update_job_progress(&queue_name, &job.message.clone(), 100).await.ok();
                } else {
                    println!("No jobs available, sleeping...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        });
    }

    /// Retries failed jobs from the failed queue.
    ///
    /// This function spawns a new asynchronous task that continuously fetches
    /// and retries jobs from the failed queue.
    pub async fn retry_failed_jobs(&self) {
        let queue_name = format!("{}:failed", self.queue_name.clone());
        let queue_service = Arc::clone(&self.queue_service);
        let original_queue_name = self.queue_name.clone();

        task::spawn(async move {
            loop {
                if let Ok(Some(job_json)) = queue_service.get_next_job(&queue_name).await {
                    let job: JobData = serde_json::from_str(&job_json).unwrap();
                    println!("Retrying failed job: {} at {}", job.message, job.timestamp);
                    
                    queue_service.add_job(&original_queue_name, job).await.ok();
                } else {
                    println!("No failed jobs available, sleeping...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                }
            }
        });
    }
}
