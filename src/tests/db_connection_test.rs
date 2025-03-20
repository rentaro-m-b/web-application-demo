#[tokio::test]
async fn test_db_connection() {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = infrastructure::db::connect().await;
    assert!(pool.is_ok());
}
