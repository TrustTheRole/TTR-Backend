pub mod health;
pub mod insights;
pub mod user;

use axum::Router;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

use crate::db::DbPool;

pub fn create_routes(pool: Arc<DbPool>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(["http://localhost:5173".parse().unwrap()])
        .allow_methods(Any)
        .allow_headers(Any);
    Router::new()
        .merge(health::create_route())
        .merge(user::create_route(pool.clone()))
        .merge(insights::create_route(pool.clone()))
        .layer(cors)
}
