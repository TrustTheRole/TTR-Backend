pub mod health;
pub mod user;

use std::sync::Arc;

use axum::Router;

use crate::db::DbPool;

pub fn create_routes(pool: Arc<DbPool>) -> Router {
    Router::new()
        .merge(health::create_route())
        .merge(user::create_route(pool.clone()))
}
