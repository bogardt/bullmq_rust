use std::sync::{Arc, Mutex};
use std::env;
use bullmq_rust::job_model::JobData;
use bullmq_rust::queue_service::QueueService;
use bullmq_rust::QueueServiceTrait;
use chrono::Utc;
use redis::RedisResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DataModel {
    message_type: String,
    message_content: String,
    message_enum: i64
}

#[tokio::main]
async fn main() -> RedisResult<()> {
    let args: Vec<String> = env::args().collect();
    let mut queue_name = "my_queue".to_string();
    let mut message_type = "TEST".to_string();
    let mut message_content = "ContentTEST".to_string();
    let mut message_enum = 3;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--queue_name" | "-q" => {
                if i + 1 < args.len() {
                    queue_name = args[i + 1].clone();
                    i += 1;
                } else {
                    print_usage();
                    return Ok(());
                }
            }
            "--message_type" | "-t" => {
                if i + 1 < args.len() {
                    message_type = args[i + 1].clone();
                    i += 1;
                } else {
                    print_usage();
                    return Ok(());
                }
            }
            "--message_content" | "-c" => {
                if i + 1 < args.len() {
                    message_content = args[i + 1].clone();
                    i += 1;
                } else {
                    print_usage();
                    return Ok(());
                }
            }
            "--message_enum" | "-e" => {
                if i + 1 < args.len() {
                    message_enum = args[i + 1].parse().unwrap_or(3);
                    i += 1;
                } else {
                    print_usage();
                    return Ok(());
                }
            }
            "--help" | "-h" => {
                print_usage();
                return Ok(());
            }
            _ => {}
        }
        i += 1;
    }

    // Create a new queue service instance
    let conn = QueueService::connect().await;
    let redis_service = Arc::new(Mutex::new(QueueService::new(conn)));

    let data_model = DataModel {
        message_type,
        message_content,
        message_enum,
    };

    // Create a job and add it to the queue
    let job_id = "job_1".to_string();
    let job = JobData {
        id: job_id.clone(),
        message: serde_json::to_string(&data_model).unwrap(),
        timestamp: Utc::now().to_rfc3339(),
        priority: Some(1),
        delay: Some(0),
        retries: Some(3),
        expires_in: Some(Utc::now().timestamp() + 60),
        progress: Some(0),
    };

    if let Err(e) = redis_service.lock().unwrap().add_job(&queue_name, job).await {
        eprintln!("Failed to add {} to {}: {}", job_id, queue_name, e);
    } else {
        eprintln!("Succeeded to add {} to {}", job_id, queue_name);
    }

    Ok(())
}

fn print_usage() {
    println!("Usage: push_message [options]");
    println!("Options:");
    println!("  --queue_name, -q <queue_name>         The name of the queue (default: my_queue)");
    println!("  --message_type, -t <message_type>     The type of the message (default: TEST)");
    println!("  --message_content, -c <message_content> The content of the message (default: ContentTEST)");
    println!("  --message_enum, -e <message_enum>     The enum value of the message (default: 3)");
    println!("  --help, -h                            Print this help message");
}
