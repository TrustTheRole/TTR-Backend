use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
    Extension,
};
use bcrypt::DEFAULT_COST;
use serde_json::{json, Value};

use crate::{
    db::DbPool,
    models::user::User,
    utils::{genrate_token, get_uid},
};
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
        Some(u_email) => match u_email.as_str() {
            Some(email_str) => email_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Email must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Email is required"
                })),
            )
                .into_response();
        }
    };

    let user_name = match req.get("name") {
        Some(u_name) => match u_name.as_str() {
            Some(name_str) => name_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Name must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Name is required"
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

    let token = genrate_token(user_email);

    if let Err(e) = token {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":e.to_string(),
            })),
        )
            .into_response();
    }

    // tracing::debug!("{:?}", token);

    let mut response = (
        StatusCode::CREATED,
        Json(json!({
            "message":"User created successfully",
            "user":user,
        })),
    )
        .into_response();

    response.headers_mut().insert(
        "Set-Cookie",
        format!("access_token={}; HttpOnly; SameSite=Strict", token.unwrap())
            .parse()
            .unwrap(),
    );

    response
}
