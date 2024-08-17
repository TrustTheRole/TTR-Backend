use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, post, patch},
    Extension, Router,
};

use crate::{
    db::DbPool,
    handlers::insights::{
        create_insight, delete_insight, disaprove, get_all_insights, get_insight_by_id,
        get_insights_by_user_id, get_recent_insights, update_insight,
    },
    middlewares::auth::{auth_middleware, check_superadmin},
};

pub fn create_route(pool: Arc<DbPool>) -> Router {
    Router::new()
        .nest(
            "/insights",
            Router::new()
                .route("/disaprove", delete(disaprove))
                .route_layer(middleware::from_fn(check_superadmin))
                .route("/create", post(create_insight))
                .route("/delete", delete(delete_insight))
                .route("/update", patch(update_insight))
                .route_layer(middleware::from_fn(auth_middleware))
                .route("/get-all", get(get_all_insights))
                .route("/get-insight-userid", get(get_insights_by_user_id))
                .route("/get-insight", get(get_insight_by_id))
                .route("/get-recent-insights", get(get_recent_insights)),
        )
        .layer(Extension(pool))
}
