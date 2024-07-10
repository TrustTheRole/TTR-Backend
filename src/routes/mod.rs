pub mod health;

use axum::Router;

pub fn create_routes() -> Router {
    Router::new().merge(health::create_route())
}
