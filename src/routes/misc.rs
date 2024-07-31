use std::sync::Arc;

use axum::{middleware, routing::post, Extension, Router};

use crate::{db::DbPool, handlers::misc::{add_college_name, add_company_name}, middlewares::auth::auth_middleware};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .nest(
            "/misc",
            Router::new()
                .route("/add-college", post(add_college_name))
                .route("/add-company", post(add_company_name)),
        )
        .layer(Extension(pool))
        .route_layer(middleware::from_fn(auth_middleware))
}
