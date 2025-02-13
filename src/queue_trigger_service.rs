use crate::job_model::JobData;
use crate::queue_service::QueueService;
use crate::QueueServiceTrait;
use serde_json;
use tokio::task;

/// Service responsible for triggering actions based on queue messages.
pub struct QueueTriggerService {
    queue_name: String,
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
    pub fn new(queue_name: String) -> Self {
        Self { queue_name }
    }

    /// Starts the trigger to monitor the queue for messages.
    ///
    /// This function spawns a new asynchronous task that continuously fetches
    /// and processes messages from the queue.
    pub async fn start(
        &self,
        refresh_time_milli: u64,
    ) {
        // Create a new connection
        let conn = QueueService::connect().await;
    
        // Create a new queue service instance
        let mut queue_service = QueueService::new(conn);

        let queue_name = self.queue_name.clone();
        task::spawn(async move {
            loop {
                if let Ok(Some(job_json)) = queue_service.get_next_job(&queue_name).await {
                    let job: JobData = serde_json::from_str(&job_json).unwrap();
                    println!(
                        "queue:\t\t{}\ntimestamp:\t{}\nmessage:\t{}",
                        queue_name, job.timestamp, job.message
                    );
                } else {
                    // println!("No jobs available, sleeping...");
                    tokio::time::sleep(tokio::time::Duration::from_millis(refresh_time_milli))
                        .await;
                }
            }
        });
    }
}
