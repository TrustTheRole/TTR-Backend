use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, patch, post},
    Extension, Router,
};
use tower::ServiceBuilder;

use crate::{
    db::DbPool,
    handlers::user::{authenticate, check_user, get_user, register, update_user_details, get_all_users},
    middlewares::{auth::auth_middleware, decrypt::decrypt_data},
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
        .merge(
            Router::new()
                .route("/user/get_details", get(get_user))
                .route("/user/update-details", patch(update_user_details))
                .route_layer(middleware::from_fn(auth_middleware)),
        )
        .merge(Router::new().route("/check_user", get(check_user)))
        .merge(Router::new().route("/users", get(get_all_users)))
        .layer(ServiceBuilder::new().layer(Extension(pool)))
}
