use bullmq_rust::job_model::JobData;
use redis::RedisResult;
use mockall::*;

mock! {
    pub ConfigService {
        pub fn get_client(&self) -> RedisResult<redis::Client>;
    }
}

mock! {
    #[derive(Clone)]
    pub RedisClient {
        pub fn get_connection(&self) -> RedisResult<()>;
    }
}

mock! {
    pub QueueService {
        /// Adds a job to the specified queue.
        ///
        /// # Arguments
        ///
        /// * `queue_name` - The name of the queue to add the job to.
        /// * `job` - The job data to add to the queue.
        ///
        /// # Returns
        ///
        /// A `RedisResult` indicating the success or failure of the operation.
        pub async fn add_job(&mut self, queue_name: &str, job: JobData) -> RedisResult<()>;

        /// Retrieves the next job from the specified queue.
        ///
        /// # Arguments
        ///
        /// * `queue_name` - The name of the queue to retrieve the job from.
        ///
        /// # Returns
        ///
        /// A `RedisResult` containing an optional job JSON string.
        pub async fn get_next_job(&mut self, queue_name: &str) -> RedisResult<Option<Vec<String>>>;

        /// Counts the number of jobs in the specified queue.
        ///
        /// # Arguments
        ///
        /// * `queue_name` - The name of the queue to count the jobs in.
        ///
        /// # Returns
        ///
        /// A `RedisResult` containing the number of jobs in the queue.
        pub async fn count_jobs(&mut self, queue_name: &str) -> RedisResult<u64>;

        /// Updates the progress of a job.
        ///
        /// # Arguments
        ///
        /// * `queue_name` - The name of the queue the job belongs to.
        /// * `job_id` - The ID of the job.
        /// * `progress` - The progress value to update.
        ///
        /// # Returns
        ///
        /// A `RedisResult` indicating the success or failure of the operation.
        pub async fn update_job_progress(&mut self, queue_name: &str, job_id: &str, progress: u32) -> RedisResult<()>;

        /// Retrieves the progress of a job.
        ///
        /// # Arguments
        ///
        /// * `queue_name` - The name of the queue the job belongs to.
        /// * `job_id` - The ID of the job.
        ///
        /// # Returns
        ///
        /// A `RedisResult` containing the progress value of the job.
        pub async fn get_job_progress(&mut self, queue_name: &str, job_id: &str) -> RedisResult<u32>;

        /// Moves a job to the failed queue.
        ///
        /// # Arguments
        ///
        /// * `queue_name` - The name of the queue the job belongs to.
        /// * `job` - The job data to move to the failed queue.
        ///
        /// # Returns
        ///
        /// A `RedisResult` indicating the success or failure of the operation.
        pub async fn move_to_failed(&mut self, queue_name: &str, job: JobData) -> RedisResult<()>;

        /// Logs the status of a job.
        ///
        /// # Arguments
        ///
        /// * `queue_name` - The name of the queue the job belongs to.
        /// * `job` - A reference to the job data.
        /// * `status` - The status message to log.
        ///
        /// # Returns
        ///
        /// A `RedisResult` indicating the success or failure of the operation.
        pub async fn log_job_status(&mut self, queue_name: &str, job: &JobData, status: &str) -> RedisResult<()>;
    }
}
