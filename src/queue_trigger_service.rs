use std::sync::Arc;
use tokio::task;
use serde_json;
use crate::QueueServiceTrait;
use crate::job_model::JobData;

/// Service responsible for triggering actions based on queue messages.
pub struct QueueTriggerService {
    queue_name: String,
    queue_service: Arc<dyn QueueServiceTrait>,
}

impl QueueTriggerService {
    /// Creates a new `QueueTriggerService`.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to monitor.
    /// * `queue_service` - An `Arc` wrapped `QueueService` instance.
    ///
    /// # Returns
    ///
    /// A new instance of `QueueTriggerService`.
    pub fn new(queue_name: String, queue_service: Arc<dyn QueueServiceTrait>) -> Self {
        Self { queue_name, queue_service }
    }

    /// Starts the trigger to monitor the queue for messages.
    ///
    /// This function spawns a new asynchronous task that continuously fetches
    /// and processes messages from the queue.
    pub async fn start(&self) {
        let queue_name = self.queue_name.clone();
        let queue_service = Arc::clone(&self.queue_service);

        task::spawn(async move {
            loop {
                if let Ok(Some(job_json)) = queue_service.get_next_job(&queue_name).await {
                    let job: JobData = serde_json::from_str(&job_json).unwrap();
                    println!("Triggered job: {} at {}", job.message, job.timestamp);

                    // Process the job (this is where you would add your custom logic)
                    // For now, we just print the job message
                    println!("Processing job: {}", job.message);

                    // Delete the job after processing
                    println!("Deleting job: {}", job.message);
                } else {
                    println!("No jobs available, sleeping...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        });
    }
}
