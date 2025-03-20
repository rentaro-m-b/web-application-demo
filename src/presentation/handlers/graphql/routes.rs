use axum::{Router, routing::get, response::IntoResponse};
use sqlx::PgPool;
use juniper::http::playground::playground_source;

pub fn routes(_db_pool: PgPool) -> Router {
    Router::new()
        .route("/", get(graphql_playground))
}

async fn graphql_playground() -> impl IntoResponse {
    axum::response::Html(playground_html())
}

fn playground_html() -> String {
    playground_source("/api/graphql", None)
}
