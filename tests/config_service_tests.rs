use bullmq_rust::config_service::ConfigService;

#[tokio::test]
async fn test_config_service_creation() {
    let config_service = ConfigService::new();
    assert_eq!(config_service.redis_url, "redis://127.0.0.1:6379");
}

#[tokio::test]
async fn test_get_client() {
    let config_service = ConfigService::new();
    let client = config_service.get_client().unwrap();
    assert!(client.get_connection().is_ok());
}
