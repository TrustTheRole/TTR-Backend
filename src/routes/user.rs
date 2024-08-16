use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, patch, post},
    Extension, Router,
};
use tower::ServiceBuilder;

use crate::{
    db::DbPool,
    handlers::user::{
        authenticate, check_user, get_all_users, get_user, register, update_user_details,
    },
    middlewares::{auth::{auth_middleware, check_superadmin}, decrypt::decrypt_data},
};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .merge(
            Router::new()
                .route("/users", get(get_all_users))
                .route_layer(middleware::from_fn(check_superadmin)),
        )
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
        .merge(
            Router::new()
                .route("/user/get_details", get(get_user))
                .route("/user/update-details", patch(update_user_details))
                .route_layer(middleware::from_fn(auth_middleware))
                .route("/check-user", get(check_user)),
        )
        .layer(ServiceBuilder::new().layer(Extension(pool)))
}
