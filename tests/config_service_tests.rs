use bullmq_rust::config_service::ConfigService;

/// Test the creation of ConfigService and validate its redis_url field.
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
