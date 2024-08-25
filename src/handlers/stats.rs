use axum::{extract::Query, response::IntoResponse, Extension, Json};
use diesel::prelude::*;
use hyper::StatusCode;
use serde_json::{json, Value};

use crate::{db::DbPool, models::likes::Likes};
use std::sync::Arc;

pub async fn get_stats(
    Extension(pool): Extension<Arc<DbPool>>,
    query: Query<Value>,
) -> impl IntoResponse {
    let _insight_id:String = match query.get("insight_id") {
        Some(_i_id) => match _i_id.as_str() {
            Some(_id) => _id.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!(
                        {"error":"insight_id must of type string"}
                    )),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!(
                    {"error":"insight_id is required"}
                )),
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
                    "error": "Database connection failed"
                })),
            )
                .into_response();
        }
    };

    

    match crate::schema::likes::dsl::likes
        .filter(crate::schema::likes::dsl::insight_id.eq(_insight_id))
        .load::<Likes>(&mut conn)
    {
        Ok(stats) => (StatusCode::OK, Json(json!({ "stats": stats }))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
