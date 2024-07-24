use std::sync::Arc;

use axum::{middleware, routing::post, Extension, Router};

use crate::{db::DbPool, handlers::insights::create_insight, middlewares::auth::auth_middleware};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .nest(
            "/insights",
            Router::new().route("/create", post(create_insight)),
        )
        .route_layer(middleware::from_fn(auth_middleware))
        .layer(Extension(pool))
}
