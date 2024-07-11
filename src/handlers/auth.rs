use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use bcrypt::DEFAULT_COST;
use serde_json::{json, Value};

use crate::{db::DbPool, models::user::User, utils::get_uid};
use crate::{
    schema::users::dsl::*,
    utils::{get_role_str, Role},
};
use diesel::prelude::*;

pub async fn register(
    Json(req): Json<Value>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let user_email = match req.get("email") {
        Some(user_email) => user_email.to_string(),
        None => {
            return (
                StatusCode::EXPECTATION_FAILED,
                Json(json!({
                    "error":"Name is required"
                })),
            )
                .into_response();
        }
    };

    let user_name = match req.get("name") {
        Some(user_name) => user_name.to_string(),
        None => {
            return (
                StatusCode::EXPECTATION_FAILED,
                Json(json!({
                    "error":"Name is required"
                })),
            )
                .into_response();
        }
    };

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":e.to_string()
                })),
            )
                .into_response();
        }
    };

    let hashed_password = match bcrypt::hash(&user_email, DEFAULT_COST) {
        Ok(user_password) => user_password,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":e.to_string()
                })),
            )
                .into_response();
        }
    };

    let user = User {
        email: user_email.clone(),
        name: user_name.clone(),
        password: hashed_password.clone(),
        user_id: get_uid(),
        alternate_email: None,
        college: None,
        github: None,
        linkedin: None,
        graduation_year: None,
        phone: None,
        role: Some(get_role_str(&Role::Student).to_string()),
    };

    let result = diesel::insert_into(users).values(&user).execute(&mut conn);

    if let Err(e) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":e.to_string(),
            })),
        )
            .into_response();
    }

    (StatusCode::CREATED).into_response()
}
