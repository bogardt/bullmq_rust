use bullmq_rust::job_model::JobData;
use redis::RedisResult;
use bullmq_rust::QueueServiceTrait;
use async_trait::async_trait;
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
    #[derive(Clone, DerefMut)]
    pub QueueService {}

    #[async_trait]
    impl QueueServiceTrait for QueueService {
        async fn add_job(&mut self,  queue_name: &str, job: JobData) -> RedisResult<()>;
        async fn get_next_job(&mut self, queue_name: &str) -> RedisResult<Option<String>>;
        async fn count_jobs(&mut self,  queue_name: &str) -> RedisResult<u64>;
        async fn update_job_progress(&mut self,  queue_name: &str, job_id: &str, progress: u32) -> RedisResult<()>;
        async fn get_job_progress(&mut self,  queue_name: &str, job_id: &str) -> RedisResult<u32>;
        async fn move_to_failed(&mut self,  queue_name: &str, job: JobData) -> RedisResult<()>;
        async fn log_job_status(&mut self,  queue_name: &str, job: &JobData, status: &str) -> RedisResult<()>;
    }
}
