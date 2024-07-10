use std::net::SocketAddr;

use tracing_subscriber::EnvFilter;
use ttr::{
    config::{self},
    routes::create_routes,
};

#[tokio::main]
async fn main() {
    config::init();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from("debug"))
        .init();

    let app = create_routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("Server connected and listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
