use axum::Router;
use sqlx::PgPool;

use super::handlers;

pub fn create_routes(db_pool: PgPool) -> Router {
    Router::new()
        .nest("/api/rest", handlers::rest::routes::routes(db_pool.clone()))
        .nest("/api/graphql", handlers::graphql::routes::routes(db_pool))
}
