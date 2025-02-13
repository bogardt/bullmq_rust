use bullmq_rust::job_model::JobData;
use redis::RedisResult;
use bullmq_rust::QueueServiceTrait;
use async_trait::async_trait;
use mockall::*;

// #[async_trait]
// pub trait RedisClient {
//     fn get_connection(&self) -> RedisResult<()>;
// }

// mock! {
//     pub ConfigService {
//         fn get_client(&self) -> RedisResult<Box<dyn RedisClient>>;
//     }
// }

// mock! {
//     pub RedisClient {
//         fn get_connection(&self) -> RedisResult<()>;
//     }
// }

// #[automock]
// #[async_trait]
// pub trait QueueServiceTrait {
//     fn new(client: Arc<tokio::sync::Mutex<redis::Client>>) -> Self;
//     async fn add_job(&self, queue_name: &str, job: JobData) -> RedisResult<()>;
//     async fn get_next_job(&self, queue_name: &str) -> RedisResult<Option<String>>;
//     async fn count_jobs(&self, queue_name: &str) -> RedisResult<u64>;
//     async fn update_job_progress(&self, queue_name: &str, job_id: &str, progress: u32) -> RedisResult<()>;
//     async fn get_job_progress(&self, queue_name: &str, job_id: &str) -> RedisResult<u32>;
//     async fn move_to_failed(&self, queue_name: &str, job: JobData) -> RedisResult<()>;
//     async fn log_job_status(&self, queue_name: &str, job: &JobData, status: &str) -> RedisResult<()>;
// }





mock! {
    #[derive(Clone, DerefMut)]
    pub QueueService {}

    #[async_trait]
    impl QueueServiceTrait for QueueService {
        async fn add_job(&self, queue_name: &str, job: JobData) -> RedisResult<()>;
        async fn get_next_job(&self, queue_name: &str) -> RedisResult<Option<String>>;
        async fn count_jobs(&self, queue_name: &str) -> RedisResult<u64>;
        async fn update_job_progress(&self, queue_name: &str, job_id: &str, progress: u32) -> RedisResult<()>;
        async fn get_job_progress(&self, queue_name: &str, job_id: &str) -> RedisResult<u32>;
        async fn move_to_failed(&self, queue_name: &str, job: JobData) -> RedisResult<()>;
        async fn log_job_status(&self, queue_name: &str, job: &JobData, status: &str) -> RedisResult<()>;
    }
}







// pub struct RedisQueueService {
//     client: Arc<Mutex<redis::Client>>,
// }

// #[async_trait]
// impl QueueServiceTrait for RedisQueueService {
//     fn new(client: Arc<tokio::sync::Mutex<redis::Client>>) -> Self {
//         Self { client }
//     }

//     async fn add_job(&self, _queue_name: &str, _job: JobData) -> RedisResult<()> {
//         Ok(())
//     }

//     async fn get_next_job(&self, _queue_name: &str) -> RedisResult<Option<String>> {
//         Ok(Some("{\"message\":\"test\"}".to_string()))
//     }

//     async fn count_jobs(&self, _queue_name: &str) -> RedisResult<u64> {
//         Ok(1)
//     }

//     async fn update_job_progress(&self, _queue_name: &str, _job_id: &str, _progress: u32) -> RedisResult<()> {
//         Ok(())
//     }

//     async fn get_job_progress(&self, _queue_name: &str, _job_id: &str) -> RedisResult<u32> {
//         Ok(100)
//     }

//     async fn move_to_failed(&self, _queue_name: &str, _job: JobData) -> RedisResult<()> {
//         Ok(())
//     }

//     async fn log_job_status(&self, _queue_name: &str, _job: &JobData, _status: &str) -> RedisResult<()> {
//         Ok(())
//     }
// }
