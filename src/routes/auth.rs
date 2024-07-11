use axum::{routing::get, Router};

use crate::handlers::auth::{authorize, token_exchange};

pub fn create_route() -> Router {
    Router::new()
        .route("/authorize", get(authorize))
        .route("/token_exchange", get(token_exchange))
}
