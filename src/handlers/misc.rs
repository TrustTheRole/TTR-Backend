use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use diesel::RunQueryDsl;
use hyper::StatusCode;
use serde_json::{json, Value};

use crate::{
    db::DbPool,
    models::misc::{colleges::College, companies::Companies, newsletter_sub::Newsletter},
    utils::get_uid,
};

pub async fn add_college_name(
    Json(req): Json<Value>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let _college_name = match req.get("college_name") {
        Some(c_name) => match c_name.as_str() {
            Some(name_str) => name_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"College name must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"College name is required"
                })),
            )
                .into_response();
        }
    };
    let _college_location = match req.get("college_location") {
        Some(c_location) => match c_location.as_str() {
            Some(location_str) => location_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"College location must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"College location is required"
                })),
            )
                .into_response();
        }
    };
    let _college_state = match req.get("college_state") {
        Some(c_state) => match c_state.as_str() {
            Some(state_str) => state_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"College state must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"College state is required"
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

    let _college = College {
        id: get_uid(),
        college_name: _college_name,
        college_location: _college_location,
        college_state: _college_state,
    };

    match diesel::insert_into(crate::schema::colleges::dsl::colleges)
        .values(&_college)
        .execute(&mut conn)
    {
        Ok(_) => (),
        Err(e) => {
            tracing::debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Failed to insert college into database"
                })),
            )
                .into_response();
        }
    };

    (
        StatusCode::CREATED,
        Json(json!({
            "message":"College added successfully",
            "college":_college
        })),
    )
        .into_response()
}

pub async fn add_company_name(
    Json(req): Json<Value>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let _company_name = match req.get("company_name") {
        Some(c_name) => match c_name.as_str() {
            Some(name_str) => name_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Company name must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Company name is required"
                })),
            )
                .into_response();
        }
    };

    let _company = Companies {
        id: get_uid(),
        company_name: _company_name,
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

    match diesel::insert_into(crate::schema::companies::dsl::companies)
        .values(&_company)
        .execute(&mut conn)
    {
        Ok(_) => (),
        Err(e) => {
            tracing::debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to insert company into database"
                })),
            )
                .into_response();
        }
    }

    (
        StatusCode::CREATED,
        Json(json!({
            "message":"Company added successfully",
            "company":_company
        })),
    )
        .into_response()
}

pub async fn subscibe_to_newsletter(
    Json(req): Json<Value>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let _user_email = match req.get("email") {
        Some(u_email) => match u_email.as_str() {
            Some(str_email) => str_email.to_string(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error":"email should be a string"
                    })),
                )
                    .into_response()
            }
        },
        None => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"email is required"
                })),
            )
                .into_response()
        }
    };

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::debug!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let newsletter = Newsletter {
        email: _user_email,
        created_at: chrono::Utc::now().naive_utc(),
    };

    if let Err(e) = diesel::insert_into(crate::schema::newsletter_sub::dsl::newsletter_sub)
        .values(&newsletter)
        .execute(&mut conn)
    {
        tracing::debug!("{:?}", e);
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error":e.to_string()
            })),
        )
            .into_response();
    }

    (
        StatusCode::CREATED,
        Json(json!({
            "message":"Subscription added"
        })),
    )
        .into_response()
}

pub async fn get_newsletter_subscibers(
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::debug!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let subscibed_users =
        crate::schema::newsletter_sub::dsl::newsletter_sub.load::<Newsletter>(&mut conn);

    if let Err(e) = subscibed_users {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":e.to_string(),
            })),
        )
            .into_response();
    };
    (
        StatusCode::OK,
        Json(json!({
            "message":"Subscibers fetched successfully",
            "subscibers":subscibed_users.unwrap()
        })),
    )
        .into_response()
}

pub async fn get_all_tags(Extension(pool): Extension<Arc<DbPool>>) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::debug!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let tags = crate::schema::tags::dsl::tags.load::<crate::models::tag::Tag>(&mut conn);
    if let Err(e) = tags {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":e.to_string()
            })),
        )
            .into_response();
    }
    (
        StatusCode::OK,
        Json(json!({
            "tags":tags.unwrap()
        })),
    )
        .into_response()
}

pub async fn get_colleges(Extension(pool): Extension<Arc<DbPool>>) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            tracing::debug!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let colleges = crate::schema::colleges::dsl::colleges
        .load::<crate::models::misc::colleges::College>(&mut conn);
    if let Err(e) = colleges {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":e.to_string()
            })),
        )
            .into_response();
    }
    (
        StatusCode::OK,
        Json(json!({
            "colleges":colleges.unwrap()
        })),
    )
        .into_response()
}
