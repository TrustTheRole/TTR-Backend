use std::{net::SocketAddr, sync::Arc};

use log::{debug, error};
use env_logger;
use ttr::{
    config::{self},
    db,
    rabbitmq,
    routes::create_routes,
};

#[tokio::main]
async fn main() {
    config::init();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    let pool = Arc::new(db::establish_connection());
    let app = create_routes(pool.clone());

    tokio::spawn(async move{
        let result = rabbitmq::connect_to_rabbitmq(pool);
        if let Err(e) = result {
            error!("Error connecting to rabbitmq: {:?}", e);
        }
    });


    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    debug!("Server connected and listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
