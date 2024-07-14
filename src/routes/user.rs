use std::sync::Arc;

use axum::{middleware, routing::post, Extension, Router};
use serde_json::Value;
use tower::ServiceBuilder;

use crate::{db::DbPool, middlewares::decrypt::decrypt_data};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .nest("/user", Router::new().route("/add_user", post(hello)))
        .route_layer(middleware::from_fn(decrypt_data))
        .layer(ServiceBuilder::new().layer(Extension(pool)))
}

async fn hello(Extension(decrypted_json): Extension<Value>) -> &'static str {
    println!("from hello fn");
    print!("{:?}", decrypted_json);
    "hello"
}
