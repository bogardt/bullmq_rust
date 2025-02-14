use std::sync::Arc;
use bullmq_rust::queue_trigger_service::QueueTriggerService;
use redis::RedisResult;
use std::env;

#[tokio::main]
async fn main() -> RedisResult<()> {
    let args: Vec<String> = env::args().collect();
    let mut queue_name = None;
    let mut refresh_time_milli = 1000;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--queue_name" | "-q" => {
                if i + 1 < args.len() {
                    queue_name = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    print_usage();
                    return Ok(());
                }
            }
            "--refresh" | "-r" => {
                if i + 1 < args.len() {
                    refresh_time_milli = args[i + 1].parse().unwrap_or(1000);
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

    if let Some(queue_name) = queue_name {
        start_queue_trigger(&queue_name, refresh_time_milli).await
    } else {
        print_usage();
        Ok(())
    }
}

fn print_usage() {
    println!("Usage: queue_trigger --queue_name <queue_name> [--refresh <time>] [--help]");
    println!("Options:");
    println!("  --queue_name, -q <queue_name>  The name of the queue to monitor (mandatory)");
    println!("  --refresh, -r <time>           The refresh time in milliseconds (default: 1000)");
    println!("  --help, -h                     Print this help message");
}

async fn start_queue_trigger(queue_name: &str, refresh_time_milli: u64) -> RedisResult<()> {
    // Create queue trigger service
    let queue_trigger = Arc::new(QueueTriggerService::new(queue_name.to_string()));
    
        // Start queue trigger to monitor and process jobs
    tokio::spawn(async move {
        queue_trigger.start(refresh_time_milli).await;
    });

    // Keep the main function alive
    loop {}
}
