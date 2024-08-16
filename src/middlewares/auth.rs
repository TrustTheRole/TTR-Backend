use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use diesel::{query_dsl::methods::FilterDsl, RunQueryDsl};
use diesel::ExpressionMethods;
use serde_json::json;
use std::sync::Arc;

use crate::{db::DbPool, models::user::User, utils::validate_token};

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

pub async fn check_superadmin<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, Response> {
    let pool = request.extensions().get::<Arc<DbPool>>().ok_or_else(|| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Database pool not found in request extensions"
            })),
        )
            .into_response()
    })?;
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

    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            tracing::debug!("{}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to establish connection to the database"
                })),
            )
                .into_response());
        }
    };


    let existing_user:User = match crate::schema::users::dsl::users
        .filter(crate::schema::users::dsl::user_id.eq(&is_valid.sub))
        .first::<User>(&mut conn)
    {
        Ok(e_user) => e_user,
        Err(e) => {
            tracing::debug!("{}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to get user"
                })),
            )
                .into_response());
        }
    };

    let _role:Option<String> = existing_user.role;


    if None==_role {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "Superadmin is not authorized"
            })),
        )
            .into_response());
    }

    let _role:String = _role.unwrap();


    if _role=="SUPERADMIN".to_string() {
        return Ok(next.run(request).await)
    }

    return Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "error": "Superadmin is not authorized"
        })),
    )
        .into_response());




}
