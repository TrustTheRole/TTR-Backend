use axum::{routing::get, Router};

use crate::handlers::health::health_check;

pub fn create_route() -> Router {
    Router::new().route("/health", get(health_check))
}
