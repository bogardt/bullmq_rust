use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JobData {
    pub message: String,
    pub timestamp: String,
    pub priority: Option<i32>,
    pub delay: Option<i64>,
    pub retries: Option<u32>,
    pub expires_in: Option<i64>,
    pub progress: Option<u32>,
}
