use axum::{response::IntoResponse, Extension, Json};
use diesel::RunQueryDsl;
use hyper::StatusCode;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{
    db::DbPool,
    models::insights::Insight,
    utils::{get_uid, Claims},
};

pub async fn create_insight(
    Extension(claim): Extension<Claims>,
    Json(req): Json<Value>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let _insight_title = match req.get("insight_title") {
        Some(i_title) => match i_title.as_str() {
            Some(title_str) => title_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Title must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Title is required"
                })),
            )
                .into_response();
        }
    };
    let _insight_description = match req.get("insight_description") {
        Some(i_description) => match i_description.as_str() {
            Some(description_str) => description_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Description must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Description is required"
                })),
            )
                .into_response();
        }
    };
    let _insight_role = match req.get("insight_role") {
        Some(i_role) => match i_role.as_str() {
            Some(role_str) => role_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Role must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Role is required"
                })),
            )
                .into_response();
        }
    };
    let _insight_company = match req.get("insight_company") {
        Some(i_company) => match i_company.as_str() {
            Some(company_str) => company_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Company must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Company is required"
                })),
            )
                .into_response();
        }
    };

    let _insight_tags: Vec<String> = match req.get("insight_tags") {
        Some(i_tags) => match serde_json::from_value(i_tags.clone()) {
            Ok(vec_tags) => vec_tags,
            Err(e) => {
                tracing::debug!("{:?}", e);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Tags need to be a list"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Tags are required"
                })),
            )
                .into_response();
        }
    };
    let _insight_picture_urls: Vec<String> = match req.get("insight_picture_urls") {
        Some(i_picture_urls) => match serde_json::from_value(i_picture_urls.clone()) {
            Ok(vec_pic_urls) => vec_pic_urls,
            Err(e) => {
                tracing::debug!("{:?}", e);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Urls need to be a list"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Urls are requried are required"
                })),
            )
                .into_response();
        }
    };
    let _insight_focus_points: Vec<String> = match req.get("insight_focus_points") {
        Some(i_focus_points) => match serde_json::from_value(i_focus_points.clone()) {
            Ok(vec_focus_points) => vec_focus_points,
            Err(e) => {
                tracing::debug!("{:?}", e);
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Focus Points need to be a list"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Focus Points are required"
                })),
            )
                .into_response();
        }
    };
    println!("{}", _insight_company);
    println!("{}", _insight_title);
    println!("{}", _insight_role);
    println!("{}", _insight_description);
    println!("{:?}", _insight_picture_urls);
    println!("{:?}", _insight_focus_points);
    println!("{:?}", _insight_tags);

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
    println!("{}", claim.sub);

    let insight = Insight {
        insight_id: get_uid(),
        user_id: claim.sub,
        insight_title: _insight_title,
        insight_company: _insight_company,
        insight_description: _insight_description,
        insight_focus_points: _insight_focus_points,
        insight_picture_urls: _insight_picture_urls,
        insight_role: _insight_role,
        insight_tags: _insight_tags,
        created_at: chrono::Utc::now().naive_utc(),
    };

    let result = diesel::insert_into(crate::schema::insights::dsl::insights)
        .values(&insight)
        .execute(&mut conn);
    if let Err(e) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":e.to_string(),
            })),
        )
            .into_response();
    };

    (
        StatusCode::CREATED,
        Json(json!({
            "message":"Insight created successfully",
            "insight":insight,
        })),
    )
        .into_response()
}

pub async fn get_all_insights(Extension(pool): Extension<Arc<DbPool>>) -> impl IntoResponse {
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
    let result = crate::schema::insights::dsl::insights.load::<Insight>(&mut conn);
    if let Err(e) = result {
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
            "insights":result.unwrap(),
        })),
    )
        .into_response(
    )
}
