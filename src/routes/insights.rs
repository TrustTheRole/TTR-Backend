use std::sync::Arc;

use axum::{middleware, routing::{delete, get, post}, Extension, Router};

use crate::{
    db::DbPool,
    handlers::insights::{create_insight, delete_insight, get_all_insights, get_insight_by_id, get_recent_insights},
    middlewares::auth::auth_middleware,
};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .nest(
            "/insights",
            Router::new()
                .route("/create", post(create_insight))
                .route("/delete", delete(delete_insight))
                .route_layer(middleware::from_fn(auth_middleware))
                .route("/get-all", get(get_all_insights))
                .route("/get-insight", get(get_insight_by_id))
                .route("/get-recent-insights", get(get_recent_insights)),
        )
        .layer(Extension(pool))
}
