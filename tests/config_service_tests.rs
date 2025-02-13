use bullmq_rust::config_service::ConfigService;
use mocks::MockRedisClient;
mod mocks;

#[tokio::test]
async fn test_config_service_creation() {
    let config_service = ConfigService {
        redis_url: "redis://redis:6379".to_string(),
    };
    assert_eq!(config_service.redis_url, "redis://redis:6379");

    let config_service = ConfigService {
        redis_url: "".to_string(),
    };
    assert_ne!(config_service.redis_url, "redis://redis:6379");
}

// #[tokio::test]
// async fn test_get_client() {
//     let config_service = ConfigService {
//         redis_url: "redis://redis:6379".to_string(),
//     };

//     let mut mock_redis_client = MockRedisClient::new();

//     // Successful case
//     mock_redis_client
//         .expect_get_connection()
//         .times(1)
//         .returning(|| Ok(()));

//     let client = config_service.get_client().unwrap();
//     assert!(client.get_connection().is_ok());

//     // Failing case
//     mock_redis_client
//         .expect_get_connection()
//         .times(1)
//         .returning(|| {
//             Err(redis::RedisError::from((
//                 redis::ErrorKind::IoError,
//                 "Failed to get connection",
//             )))
//         });

//     let client = config_service.get_client().unwrap();
//     assert!(client.get_connection().is_err());
// }
