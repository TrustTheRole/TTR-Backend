use std::sync::Arc;

use axum::{middleware, routing::get, routing::post, Extension, Router};

use crate::{
    db::DbPool,
    handlers::insights::{create_insight, get_all_insights},
    middlewares::auth::auth_middleware,
};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .nest(
            "/insights",
            Router::new()
                .route("/create", post(create_insight))
                .route_layer(middleware::from_fn(auth_middleware))
                .route("/get-all", get(get_all_insights)),
        )
        .layer(Extension(pool))
}
