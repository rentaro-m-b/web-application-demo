use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn routes(_db_pool: PgPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "REST API is running."
}
