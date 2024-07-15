use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::utils::validate_token;

pub async fn auth_middleware<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, Response> {
    let headers = request.headers();
    let auth_token = headers.get("Authorization").ok_or_else(|| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Authorization token is required"
            })),
        )
            .into_response()
    })?;

    let auth_token = auth_token
        .to_str()
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "Error converting auth_token to string"
                })),
            )
                .into_response()
        })?
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "Authorization token is required"
                })),
            )
                .into_response()
        })?;

    tracing::debug!("auth_token: {:?}", auth_token);

    let is_valid = validate_token(auth_token).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Invalid token"
            })),
        )
            .into_response()
    })?;

    request.extensions_mut().insert(is_valid);
    Ok(next.run(request).await)
}
