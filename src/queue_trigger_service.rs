use crate::job_model::JobData;
use crate::queue_service::QueueService;
use crate::QueueServiceTrait;
use serde_json;
use tokio::task;

/// Service responsible for triggering actions based on queue messages.
pub struct QueueTriggerService {
    queue_name: String,
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum RetryMethod {
    Reconnect,
    AskRedirect,
    MovedRedirect,
    WaitAndRetry,
    RetryImmediately,
}

impl QueueTriggerService {
    /// Creates a new `QueueTriggerService`.
    ///
    /// # Arguments
    ///
    /// * `queue_name` - The name of the queue to monitor.
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
    ///
    /// # Arguments
    ///
    /// * `refresh_time_milli` - The time in milliseconds to wait before checking the queue again.
    pub async fn start(&self, refresh_time_milli: u64) {
        let con_manager = QueueService::connect().await;
        let mut queue_service = QueueService::new(con_manager);

        let queue_name = self.queue_name.clone();
        task::spawn(async move {
            loop {
                match queue_service.get_next_job(&queue_name).await {
                    Ok(Some(job_json)) => {
                        if job_json.len() > 0 {

                            let job: JobData = serde_json::from_str(&job_json[0]).unwrap();
                            let timestamp = &job_json[1];
    
                            println!(
                                "queue:\t\t{}\ntimestamp:\t{}\nid:\t\t{}\nmessage:\t{}\n",
                                queue_name, job.timestamp, job.id, job.message
                            );
                        }
                    }
                    Ok(None) => {
                        println!("No jobs found. Retrying...");
                    }
                    Err(e) => {
                        if e.is_timeout() || e.is_connection_dropped() || e.is_connection_refusal()
                        {
                            eprintln!("Error fetching job: {}. Reconnecting...", e);
                        } else {
                            eprintln!("Error fetching job: {}. Retrying...", e);
                        }
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(refresh_time_milli)).await;
            }
        });
    }
}
