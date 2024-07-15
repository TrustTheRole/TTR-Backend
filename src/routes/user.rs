use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use tower::ServiceBuilder;

use crate::{
    db::DbPool,
    handlers::user::{authenticate, get_user, register},
    middlewares::decrypt::decrypt_data,
};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .merge(
            Router::new()
                .nest(
                    "/user/auth",
                    Router::new()
                        .route("/register", post(register))
                        .route("/authenticate", post(authenticate)),
                )
                .route_layer(middleware::from_fn(decrypt_data)),
        )
        .merge(Router::new().route("/user/get_details", get(get_user)))
        .layer(ServiceBuilder::new().layer(Extension(pool)))
}
