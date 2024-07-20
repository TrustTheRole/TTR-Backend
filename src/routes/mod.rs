pub mod health;
pub mod user;

use std::sync::Arc;

use axum::Router;
use tower_http::cors::{CorsLayer, Any};
use crate::db::DbPool;

pub fn create_routes(pool: Arc<DbPool>) -> Router {
    let cors = CorsLayer::new()
            .allow_origin(["http://localhost:5173".parse().unwrap()])
            .allow_methods(Any)
            .allow_headers(Any);
    Router::new()
        .merge(health::create_route())
        .merge(user::create_route(pool.clone()))
        .layer(cors)
}
