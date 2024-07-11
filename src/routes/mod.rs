pub mod auth;
pub mod health;

use std::sync::Arc;

use axum::Router;

use crate::db::DbPool;

pub fn create_routes(pool: Arc<DbPool>) -> Router {
    Router::new()
        .merge(health::create_route())
        .merge(auth::create_route(pool.clone()))
}
