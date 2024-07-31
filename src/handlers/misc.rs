use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use hyper::StatusCode;
use serde_json::{json,Value};
use diesel::RunQueryDsl;

use crate::{db::DbPool, models::{colleges::College, companies::Companies}, utils::get_uid};

pub async fn add_college_name(Json(req):Json<Value>,Extension(pool):Extension<Arc<DbPool>>)->impl IntoResponse{
    let _college_name = match req.get("college_name"){
        Some(c_name)=>match c_name.as_str(){
            Some(name_str)=>name_str.to_string(),
            None=>{
                return(
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"College name must be a string"
                    })),
                )
                .into_response();
            }
        },
        None=>{
            return(
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"College name is required"
                })),
            )
            .into_response();
        }
    };
    let _college_location = match req.get("college_location"){
        Some(c_location)=>match c_location.as_str(){
            Some(location_str)=>location_str.to_string(),
            None=>{
                return(
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"College location must be a string"
                    })),
                )
                .into_response();
            }
        },
        None=>{
            return(
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"College location is required"
                })),
            )
            .into_response();
        }
    };
    let _college_state = match req.get("college_state"){
        Some(c_state)=>match c_state.as_str(){
            Some(state_str)=>state_str.to_string(),
            None=>{
                return(
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"College state must be a string"
                    })),
                )
                .into_response();
            }
        },
        None=>{
            return(
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

    let _college = College{
        id:get_uid(),
        college_name:_college_name,
        college_location:_college_location,
        college_state:_college_state,
    };

    match diesel::insert_into(crate::schema::colleges::dsl::colleges)
        .values(&_college)
        .execute(&mut conn){
            Ok(_)=>(),
            Err(e)=>{
                tracing::debug!("{}",e);
                return(
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
    ).into_response()
}

pub async fn add_company_name(Json(req):Json<Value>,Extension(pool):Extension<Arc<DbPool>>)->impl IntoResponse{
    let _comapny_name = match req.get("company_name"){
        Some(c_name)=>match c_name.as_str(){
            Some(name_str)=>name_str.to_string(),
            None=>{
                return(
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Company name must be a string"
                    })),
                )
                .into_response();
            }
        },
        None=>{
            return(
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Company name is required"
                })),
            )
            .into_response();
        }
    };

    let _company = Companies{
        id:get_uid(),
        company_name:_comapny_name,
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

    match diesel::insert_into(crate::schema::companies::dsl::companies).values(&_company).execute(&mut conn) {
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
    ).into_response()
}