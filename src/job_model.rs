use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JobData {
    /// The message of the job.
    pub message: String,
    /// The timestamp when the job was created.
    pub timestamp: String,
    /// The priority of the job.
    pub priority: Option<i32>,
    /// The delay before the job can be processed.
    pub delay: Option<i64>,
    /// The number of retries allowed for the job.
    pub retries: Option<u32>,
    /// The expiration time of the job.
    pub expires_in: Option<i64>,
    /// The progress of the job.
    pub progress: Option<u32>,
}
