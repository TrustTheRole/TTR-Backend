use std::sync::Arc;

use axum::{routing::post, Extension, Router};

use crate::{db::DbPool, handlers::auth::register};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .nest("/auth", Router::new().route("/register", post(register)))
        .layer(Extension(pool))
}
