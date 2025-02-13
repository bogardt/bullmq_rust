# BullMQ Wrapper in Rust - Advanced Job Management with Redis

## 📌 Description
This library is a modular **BullMQ-inspired** wrapper in **Rust**, allowing **queue, job, and worker management** via **Redis**. It provides advanced features such as **job prioritization, delays, retries, and queue management**.

## 📂 Project Architecture
```
/src
  ├── config_service.rs     # Centralized Redis configuration management
  ├── queue_service.rs      # Queue and job management
  ├── worker_service.rs     # Workers for job execution
  ├── job_model.rs          # Job model with advanced options
  ├── log_service.rs        # Logging service for job events
  ├── lib.rs                # Library module declarations
  ├── main.rs               # Application entry point
/tests
  ├── config_service_tests.rs # Tests for ConfigService
  ├── mocks                 # Mock services for testing
```

## 🚀 Installation
Ensure you have **Rust** installed and **Redis** running locally or in the cloud.

```sh
cargo build
```

## 🛠️ Configuration
Create a `.env` file with Redis parameters:
```env
REDIS_URL=redis://127.0.0.1:6379
```

## 📖 Usage Example
1. start redis
```
docker compose up -d
```

2. Create a queue trigger service :
```
cargo run --bin queue_trigger 
```

3. Push message to queue :
```
cargo run --bin push_message 
```



### 1️⃣ Add a Job
```rust
use bullmq_rust::queue_service::QueueService;
use bullmq_rust::job_model::JobData;
use bullmq_rust::config_service::ConfigService;
use chrono::Utc;

#[tokio::main]
async fn main() {
    let config = ConfigService::new();
    let mut queue_service = QueueService::new(config.get_client().unwrap());

    let job = JobData {
        id: "1".to_string(),
        message: "Hello, Rust!".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        priority: Some(1),
        delay: Some(5),
        retries: Some(3),
        expires_in: None,
        progress: Some(0),
    };

    queue_service.add_job("testQueue", job).await.unwrap();
}
```

### 2️⃣ Start a Worker to Process Jobs
```rust
use std::sync::Arc;
use bullmq_rust::worker_service::WorkerService;
use bullmq_rust::queue_service::QueueService;
use bullmq_rust::config_service::ConfigService;

#[tokio::main]
async fn main() {
    let config = ConfigService::new();
    let queue_service = Arc::new(QueueService::new(config.get_client().unwrap()));
    let worker = WorkerService::new("testQueue".to_string(), Arc::clone(&queue_service));
    worker.start().await;
}
```

### 3️⃣ Retry Failed Jobs
```rust
use std::sync::Arc;
use bullmq_rust::worker_service::WorkerService;
use bullmq_rust::queue_service::QueueService;
use bullmq_rust::config_service::ConfigService;

#[tokio::main]
async fn main() {
    let config = ConfigService::new();
    let queue_service = Arc::new(QueueService::new(config.get_client().unwrap()));
    let worker = WorkerService::new("testQueue".to_string(), Arc::clone(&queue_service));
    worker.retry_failed_jobs().await;
}
```


## 📜 Detailed Documentation

### ConfigService
Manages Redis configuration.

#### Methods:
- `new() -> Self`: Creates a new `ConfigService` instance.
- `get_client(&self) -> RedisResult<Client>`: Returns a Redis client.

### QueueService
Manages queues and jobs in Redis.

#### Methods:
- `new(conn: redis::Connection) -> Self`: Creates a new `QueueService` instance.
- `add_job(&mut self, queue_name: &str, job: JobData) -> RedisResult<()>`: Adds a job to the specified queue.
- `get_next_job(&mut self, queue_name: &str) -> RedisResult<Option<String>>`: Retrieves the next job from the specified queue.
- `count_jobs(&mut self, queue_name: &str) -> RedisResult<u64>`: Counts the number of jobs in the specified queue.
- `move_to_failed(&mut self, queue_name: &str, job: JobData) -> RedisResult<()>`: Moves a job to the failed queue.
- `log_job_status(&mut self, queue_name: &str, job: &JobData, status: &str) -> RedisResult<()>`: Logs the status of a job.
- `update_job_progress(&mut self, queue_name: &str, job_id: &str, progress: u32) -> RedisResult<()>`: Updates the progress of a job.
- `get_job_progress(&mut self, queue_name: &str, job_id: &str) -> RedisResult<u32>`: Retrieves the progress of a job.

### WorkerService
Manages workers that process jobs from a queue.

#### Methods:
- `new(queue_name: String, queue_service: Arc<QueueService>) -> Self`: Creates a new `WorkerService` instance.
- `start(&self)`: Starts the worker to process jobs from the queue.
- `retry_failed_jobs(&self)`: Retries failed jobs from the failed queue.

### LogService
Logs job events to Redis.

#### Methods:
- `new(client: Arc<Mutex<Client>>) -> Self`: Creates a new `LogService` instance.
- `log(&self, queue_name: &str, message: &str) -> RedisResult<()>`: Logs a message to the specified queue's log.

### JobData
Represents the data of a job.

#### Fields:
- `id: String`: The unique identifier of the job.
- `message: String`: The message of the job.
- `timestamp: String`: The timestamp when the job was created.
- `priority: Option<i32>`: The priority of the job.
- `delay: Option<i64>`: The delay before the job can be processed.
- `retries: Option<u32>`: The number of retries allowed for the job.
- `expires_in: Option<i64>`: The expiration time of the job.
- `progress: Option<u32>`: The progress of the job.

## 🐳 Docker Setup
### Dockerfile
```dockerfile
FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN apt-get update && apt-get install -y libssl-dev pkg-config
RUN cargo build --release

CMD ["cargo", "run"]
```

### .env
```env
REDIS_URL=redis://localhost:6379
```

## 📜 License
This project is licensed under the MIT License.
