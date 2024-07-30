use std::sync::Arc;

use axum::{http::Request, response::IntoResponse, Extension, Json};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use hyper::StatusCode;
use serde_json::{json, Value};

use crate::{
    db::DbPool,
    models::user::User,
    utils::{dispatch_email, generate_token, get_uid, Claims},
};

use crate::schema::users::dsl::*;

pub async fn register(
    Extension(decrypted_json): Extension<Value>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let user_email = match decrypted_json.get("email") {
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
    let user_name = match decrypted_json.get("name") {
        Some(u_name) => match u_name.as_str() {
            Some(email_str) => email_str.to_string(),
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
    let user_alternate_email = decrypted_json
        .get("alternate_email")
        .and_then(|v| v.as_str().map(|s| s.to_string()));
    let user_college = decrypted_json
        .get("college")
        .and_then(|v| v.as_str().map(|s| s.to_string()));
    let user_github = decrypted_json
        .get("github")
        .and_then(|v| v.as_str().map(|s| s.to_string()));
    let user_linkedin = decrypted_json
        .get("linkedin")
        .and_then(|v| v.as_str().map(|s| s.to_string()));
    let user_graduation_year: Option<i32> = decrypted_json
        .get("linkedin")
        .and_then(|v| v.as_i64().and_then(|n| n.try_into().ok()));
    let user_phone = decrypted_json
        .get("phone")
        .and_then(|v| v.as_str().map(|s| s.to_string()));
    let user_role = decrypted_json
        .get("role")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    let _user_id=get_uid();

    let user = User {
        user_id: _user_id.clone(),
        email: user_email.clone(),
        name: user_name.clone(),
        alternate_email: user_alternate_email,
        college: user_college,
        github: user_github,
        linkedin: user_linkedin,
        graduation_year: user_graduation_year,
        phone: user_phone,
        role: user_role,
    };

    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            tracing::debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
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

    let token = generate_token(_user_id);

    if let Err(e) = token {
        tracing::debug!("{}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":"Failed to generate authorization token"
            })),
        )
            .into_response();
    }
    let token = token.unwrap();

    let message="Welcome to the community".to_string();

    dispatch_email(&user_name,&user_email,&message,"Welcome to the TTR Community".to_string()).await;

    (
        StatusCode::CREATED,
        Json(json!({
            "message":"User registered successfully",
            "user":user,
            "token":token
        })),
    )
        .into_response()
}

pub async fn get_user(
    Extension(claim): Extension<Claims>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            tracing::debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };
    let user_email = claim.sub;
    let existing_user = match crate::schema::users::dsl::users
        .filter(email.eq(&user_email))
        .first::<User>(&mut conn)
    {
        Ok(e_user) => e_user,
        Err(e) => {
            tracing::debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Cannot find user"
                })),
            )
                .into_response();
        }
    };
    (
        StatusCode::OK,
        Json(json!({
            "message":"User found",
            "user":existing_user
        })),
    )
        .into_response()
}

pub async fn check_user<B>(
    Extension(pool): Extension<Arc<DbPool>>,
    req: Request<B>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            tracing::debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };
    let user_email = match req.headers().get("email") {
        Some(u_email) => u_email,
        None => {
            return (
                StatusCode::EXPECTATION_FAILED,
                Json(json!({
                    "error":"Please provider email"
                })),
            )
                .into_response();
        }
    };

    let user_email = match user_email.to_str() {
        Ok(u_mail) => u_mail,
        Err(e) => {
            tracing::debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Please provide email as string"
                })),
            )
                .into_response();
        }
    };
    if let Err(e) = crate::schema::users::dsl::users
        .filter(email.eq(&user_email))
        .first::<User>(&mut conn)
    {
        tracing::debug!("{}", e);
        return (
            StatusCode::OK,
            Json(json!({
                "error":"User does not exist"
            })),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({
            "message":"User exists"
        })),
    )
        .into_response()
}

pub async fn authenticate(
    Extension(decrypted_json): Extension<Value>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let user_email = match decrypted_json.get("email") {
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

    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            tracing::debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let existing_user = match crate::schema::users::dsl::users
        .filter(email.eq(&user_email))
        .first::<User>(&mut conn)
    {
        Ok(e_user) => e_user,
        Err(e) => {
            tracing::debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Cannot find user"
                })),
            )
                .into_response();
        }
    };

    tracing::debug!("existing user = {:?}", existing_user);

    let token = generate_token(existing_user.user_id.clone());

    if let Err(e) = token {
        tracing::debug!("{}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":"Failed to generate authorization token"
            })),
        )
            .into_response();
    }
    let token = token.unwrap();

    (
        StatusCode::OK,
        Json(json!({
            "message":"User authenticated successfully",
            "user":existing_user,
            "token":token
        })),
    )
        .into_response()
}
