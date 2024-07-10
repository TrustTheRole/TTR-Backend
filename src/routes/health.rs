use std::sync::Arc;

use axum::{routing::get, Extension, Router};
use tower::ServiceBuilder;

use crate::{db::DbPool, handlers::health::health_check};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .layer(ServiceBuilder::new().layer(Extension(pool)))
}
