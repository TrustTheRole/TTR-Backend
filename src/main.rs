use std::{net::SocketAddr, sync::Arc};

use tracing_subscriber::EnvFilter;
use ttr::{
    config::{self},
    db,
    rabbitmq,
    routes::create_routes,
};

#[tokio::main]
async fn main() {
    config::init();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from("debug"))
        .init();

    let pool = Arc::new(db::establish_connection());
    let app = create_routes(pool.clone());

    tokio::spawn(async move{
        let result = rabbitmq::connect_to_rabbitmq(pool);
        if let Err(e) = result {
            tracing::error!("Error connecting to RabbitMQ: {:?}", e);
        }
    });


    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("Server connected and listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
