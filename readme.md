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
  ├── main.rs               # Application entry point
```

## 🚀 Installation
Ensure you have **Rust** installed and **Redis** running locally or in the cloud.

```sh
cargo build
```

Add the following dependencies to your `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
dotenv = "0.15"
redis = "0.22"
serde = { version = "1.0", features = ["derive"] }
chrono = "0.4"
```

## 🛠️ Configuration
Create a `.env` file with Redis parameters:
```env
REDIS_URL=redis://127.0.0.1:6379
```

## 📖 Usage

### 1️⃣ Add a Job
```rust
use queue_service::QueueService;
use job_model::JobData;
use chrono::Utc;

#[tokio::main]
async fn main() {
    let queue_service = QueueService::new("testQueue").unwrap();
    
    let job = JobData {
        message: "Hello, Rust!".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        priority: Some(1),
        delay: Some(5),
        retries: Some(3),
        progress: Some(0),
    };
    
    queue_service.add_job(job).await.unwrap();
}
```

### 2️⃣ Start a Worker to Process Jobs
```rust
use worker_service::WorkerService;

#[tokio::main]
async fn main() {
    let worker = WorkerService::new("testQueue");
    worker.start().await;
}
```

### 3️⃣ Track Job Progress
```rust
use queue_service::QueueService;

#[tokio::main]
async fn main() {
    let queue_service = QueueService::new("testQueue").unwrap();
    
    let progress = queue_service.get_job_progress("testQueue", "job_id").await.unwrap();
    println!("Job progress: {}", progress);
}
```

## 🔍 Features
✅ **Job addition and execution**
✅ **Priority and delay management**
✅ **Automatic retries on failure**
✅ **Job count tracking**
✅ **Job progress tracking**
✅ **Simple integration with Redis**

## 📜 License
MIT

---

💡 **Contributions and feedback are welcome!** 🚀

