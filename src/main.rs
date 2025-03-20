mod domain;
mod infrastructure;
mod presentation;

use infrastructure::db;
use presentation::routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_tracing();

    let app = init_router().await;

    
    let listener = init_listener("0.0.0.0:8080").await;

    tracing::info!("Server running at http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}

fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
}

async fn init_router() -> axum::Router {
    let db_pool = db::connect()
        .await
        .expect("Failed to connect to DB");

    routes::create_routes(db_pool)
}

async fn init_listener(address: &str) -> tokio::net::TcpListener {
    let socket_addr: std::net::SocketAddr = address
        .parse()
        .expect("Failed to parse server address");
    
    tokio::net::TcpListener::bind(socket_addr)
        .await
        .expect("Failed to bind listener")
}

