use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};

use crate::{
    db::DbPool,
    handlers::misc::{
        add_college_name, add_company_name, get_all_tags, get_colleges, get_newsletter_subscibers, send_newsletter, subscibe_to_newsletter
    },
    middlewares::auth::auth_middleware,
};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .nest(
            "/misc",
            Router::new()
                .merge(
                    Router::new()
                        .route("/add-college", post(add_college_name))
                        .route("/add-company", post(add_company_name))
                        .route("/get-newsletter-subs", get(get_newsletter_subscibers)),
                )
                .route_layer(middleware::from_fn(auth_middleware))
                .merge(
                    Router::new()
                        .route("/subscribe-newsletter", post(subscibe_to_newsletter))
                        .route("/get-tags", get(get_all_tags))
                        .route("/get-colleges", get(get_colleges))
                        .route("/send-newsletter", post(send_newsletter)),
                ),
        )
        .layer(Extension(pool))
}
