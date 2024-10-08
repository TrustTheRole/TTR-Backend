use amiquip::{Connection, ExchangeDeclareOptions, ExchangeType, Publish};
use axum::{extract::Query, response::IntoResponse, Extension, Json};
use diesel::{
    dsl::sql,
    query_dsl::methods::{FilterDsl, LimitDsl, OrderDsl},
    ExpressionMethods, RunQueryDsl,
};
use hyper::StatusCode;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{
    db::DbPool,
    models::{
        actions::ActionType, insights::{Insight, InsightQuery, InsightResponse, UpdateInsight}, likes::Likes, user::User
    },
    utils::{dispatch_email, get_uid, Claims},
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
                debug!("{:?}", e);
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
                debug!("{:?}", e);
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
                debug!("{:?}", e);
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
    debug!("{}", _insight_company);
    debug!("{}", _insight_title);
    debug!("{}", _insight_role);
    debug!("{}", _insight_description);
    debug!("{:?}", _insight_picture_urls);
    debug!("{:?}", _insight_focus_points);
    debug!("{:?}", _insight_tags);

    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };
    debug!("{}", claim.sub);

    // extract_tags(&_insight_tags, &mut conn);

    let exising_user: User = match crate::schema::users::dsl::users
        .filter(crate::schema::users::dsl::user_id.eq(&claim.sub))
        .first::<crate::models::user::User>(&mut conn)
    {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":e.to_string(),
                })),
            )
                .into_response();
        }
    };

    let insight = Insight {
        insight_id: get_uid(),
        user_id: claim.sub.clone(),
        user_name: exising_user.name,
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

    let insight_stats = Likes {
        insight_id: insight.insight_id.clone(),
        like_count: 0,
        view_count: 0,
    };

    let result = diesel::insert_into(crate::schema::likes::dsl::likes)
        .values(&insight_stats)
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
            debug!("{}", e);
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

    let insights: Vec<InsightResponse> = result
        .unwrap()
        .into_iter()
        .map(|insight| InsightResponse {
            insight_id: insight.insight_id,
            insight_title: insight.insight_title,
            insight_company: insight.insight_company,
            insight_role: insight.insight_role,
            insight_tags: insight.insight_tags,
            insight_description: insight.insight_description,
            insight_picture_urls: insight.insight_picture_urls,
            insight_focus_points: insight.insight_focus_points,
            created_at: insight.created_at,
        })
        .collect();

    (
        StatusCode::OK,
        Json(json!({
            "insights": insights,
        })),
    )
        .into_response()
}

#[derive(Deserialize)]
pub struct InsightsQuery {
    pub limit: Option<usize>,
}

pub async fn get_recent_insights(
    Extension(pool): Extension<Arc<DbPool>>,
    Query(query): Query<InsightsQuery>,
) -> impl IntoResponse {
    let limit = query.limit.unwrap_or(5);

    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let limit_i64: i64 = limit.try_into().expect("limit conversion failed");

    let result = crate::schema::insights::dsl::insights
        .order(crate::schema::insights::dsl::created_at.desc())
        .limit(limit_i64)
        .load::<Insight>(&mut conn);

    if let Err(e) = result {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": e.to_string(),
            })),
        )
            .into_response();
    };

    let insights: Vec<InsightResponse> = result
        .unwrap()
        .into_iter()
        .map(|insight| InsightResponse {
            insight_id: insight.insight_id,
            insight_title: insight.insight_title,
            insight_company: insight.insight_company,
            insight_role: insight.insight_role,
            insight_tags: insight.insight_tags,
            insight_description: insight.insight_description,
            insight_picture_urls: insight.insight_picture_urls,
            insight_focus_points: insight.insight_focus_points,
            created_at: insight.created_at,
        })
        .collect();

    (
        StatusCode::OK,
        Json(json!({
            "insights": insights,
        })),
    )
        .into_response()
}

pub async fn get_insight_by_id(
    Extension(pool): Extension<Arc<DbPool>>,
    Query(query): Query<Value>,
) -> impl IntoResponse {
    let insight_id = match query.get("insight_id") {
        Some(i_id) => match i_id.as_str() {
            Some(id_str) => id_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Insight ID must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Insight ID is required"
                })),
            )
                .into_response();
        }
    };

    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let result = crate::schema::insights::dsl::insights
        .filter(crate::schema::insights::dsl::insight_id.eq(insight_id))
        .first::<Insight>(&mut conn);

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
            "insight":result.unwrap(),
        })),
    )
        .into_response()
}

pub async fn delete_insight(
    Extension(pool): Extension<Arc<DbPool>>,
    Query(query): Query<Value>,
) -> impl IntoResponse {
    let insight_id = match query.get("insight_id") {
        Some(i_id) => match i_id.as_str() {
            Some(id_str) => id_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Insight ID must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Insight ID is required"
                })),
            )
                .into_response();
        }
    };

    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let result = diesel::delete(
        crate::schema::insights::dsl::insights
            .filter(crate::schema::insights::dsl::insight_id.eq(insight_id)),
    )
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
        StatusCode::OK,
        Json(json!({
            "message":"Insight deleted successfully",
        })),
    )
        .into_response()
}

pub async fn get_insights_by_user_id(
    Extension(pool): Extension<Arc<DbPool>>,
    Query(query): Query<Value>,
) -> impl IntoResponse {
    let user_id = match query.get("user_id") {
        Some(u_id) => match u_id.as_str() {
            Some(id_str) => id_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({ "error": "User ID must be a string" })),
                )
                    .into_response()
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "User ID is required" })),
            )
                .into_response()
        }
    };

    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database connection failed: {}", e) })),
            )
                .into_response()
        }
    };

    match crate::schema::insights::dsl::insights
        .filter(crate::schema::insights::dsl::user_id.eq(user_id))
        .load::<Insight>(&mut conn)
    {
        Ok(insights) => (StatusCode::OK, Json(json!({ "insights": insights }))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub async fn update_insight(
    Extension(pool): Extension<Arc<DbPool>>,
    Json(req): Json<Value>,
    Extension(claim): Extension<Claims>
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let _insight_id = match req.get("insight_id") {
        Some(_i_id) => match _i_id.as_str() {
            Some(i_id) => i_id.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Insight ID must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Insight ID is required"
                })),
            )
                .into_response();
        }
    };

    let existing_insight = match crate::schema::insights::dsl::insights
        .filter(crate::schema::insights::insight_id.eq(&_insight_id))
        .first::<Insight>(&mut conn)
    {
        Ok(insight) => insight,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": e.to_string(),
                })),
            )
                .into_response();
        }
    };

    if existing_insight.user_id != claim.sub {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error":"You are not authorized to update this insight"
            })),
        )
            .into_response();
    }

    let updated_insight = UpdateInsight {
        insight_description: req.get("insight_description").and_then(|v| v.as_str()),
        insight_focus_points: req
            .get("insight_focus_points")
            .and_then(|v| serde_json::from_value(v.clone()).ok()),
        insight_role: req.get("insight_role").and_then(|v| v.as_str()),
        insight_tags: req
            .get("insight_tags")
            .and_then(|v| serde_json::from_value(v.clone()).ok()),
        insight_title: req.get("insight_title").and_then(|v| v.as_str()),
    };

    match diesel::update(
        crate::schema::insights::table.filter(crate::schema::insights::insight_id.eq(&_insight_id)),
    )
    .set(&updated_insight)
    .execute(&mut conn)
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "message":"Insight updated successfully"
            })),
        )
            .into_response(),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": e.to_string(),
                })),
            )
                .into_response();
        }
    }
}

pub async fn disaprove(
    Extension(pool): Extension<Arc<DbPool>>,
    Query(query): Query<Value>,
) -> impl IntoResponse {
    let insight_id = match query.get("insight_id") {
        Some(i_id) => match i_id.as_str() {
            Some(id_str) => id_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Insight ID must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Insight ID is required"
                })),
            )
                .into_response();
        }
    };

    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            debug!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let _existing_insight: Insight = match crate::schema::insights::dsl::insights
        .filter(crate::schema::insights::dsl::insight_id.eq(&insight_id))
        .first::<Insight>(&mut conn)
    {
        Ok(insight) => insight,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":e.to_string(),
                })),
            )
                .into_response();
        }
    };

    let _user: User = match crate::schema::users::dsl::users
        .filter(crate::schema::users::dsl::user_id.eq(&_existing_insight.user_id))
        .first::<User>(&mut conn)
    {
        Ok(user) => user,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":e.to_string(),
                })),
            )
                .into_response();
        }
    };

    let result = diesel::delete(
        crate::schema::insights::dsl::insights
            .filter(crate::schema::insights::dsl::insight_id.eq(insight_id)),
    )
    .execute(&mut conn);

    let html_content = format!(
        r#"
        <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>[TTR] Insight Disapproval Notification</title>
        <style>
            body {{
                margin: 0;
                padding: 0;
                background-color: #f4f4f4;
                font-family: Arial, sans-serif;
            }}
            table {{
                border-collapse: collapse;
                width: 100%;
            }}
            .container {{
                width: 600px;
                margin: 20px auto;
                background-color: #ffffff;
                border: 1px solid #cccccc;
            }}
            .header, .content, .footer {{
                padding: 20px;
                text-align: center;
            }}
            .header img {{
                width: 35%;
                border: 0;
            }}
            .status-icon img {{
                width: 48px;
                vertical-align: middle;
            }}
            .status-text {{
                font-size: 24px;
                color: #ff0000;
                margin: 0;
            }}
            .remarks {{
                font-size: 16px;
                color: #333333;
                margin-top: 20px;
            }}
            .button {{
                background-color: #4CAF50;
                color: white;
                padding: 10px 20px;
                text-decoration: none;
                border-radius: 5px;
                display: inline-block;
                font-family: Arial, sans-serif;
            }}
        </style>
    </head>
    <body>
        <table role="presentation" border="0" cellpadding="0" cellspacing="0" class="container">
            <tr>
                <td class="header">
                    <img src="https://ik.imagekit.io/s1vtpplq4/TTR.png?updatedAt=1722362280763" alt="Company Logo">
                </td>
            </tr>
            <tr>
                <td class="content">
                    <table role="presentation" border="0" cellpadding="0" cellspacing="0" align="center">
                        <tr>
                            <!-- <td class="status-icon">
                                <img src="https://images.unsplash.com/photo-1719937206158-cad5e6775044?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDF8MHxmZWF0dXJlZC1waG90b3MtZmVlZHwxfHx8ZW58MHx8fHx8" alt="Error Icon">
                            </td> -->
                            <td>
                                <h1 class="status-text">Insight Disapproved</h1>
                            </td>
                        </tr>
                    </table>
                    <p><strong>Title:</strong> [Insight Title]</p>
                    <p><strong>ID:</strong> [Insight ID]</p>
                    <p><strong>Image:</strong></p>
                    <img src="https://images.unsplash.com/photo-1719937206158-cad5e6775044?w=500&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDF8MHxmZWF0dXJlZC1waG90b3MtZmVlZHwxfHx8ZW58MHx8fHx8" alt="Insight Image" style="width: 100%; max-width: 600px; height: auto; border-radius: 10px;">
                    <p class="remarks"><strong>Remarks for Disapproval:</strong> [Disapproval Remarks]</p>
                </td>
            </tr>
            <tr>
                <td class="footer">
                    <a href="https://yourcompanywebsite.com" class="button">Visit Our Website</a>
                </td>
            </tr>
        </table>
    </body>
    </html>
        "#
    );

    let message = format!(
        "Hello {},\n\nYour insight with the title '{}' has been disapproved. Please check your email for more details.\n\nRegards,\nTeam TTR",
        _user.name, _existing_insight.insight_title
    );

    dispatch_email(
        &_user.name,
        &_user.email,
        &message,
        "[TTR] Insight Disapproval Notification".to_string(),
        &html_content,
    )
    .await;

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
            "message":"Insight Disaprooved"
        })),
    )
        .into_response()
}



#[derive(Debug, Deserialize, Serialize)]
pub struct InsightAction {
    pub insight_id: String,
    pub action_type: ActionType,
    pub user_id: String,
}

pub async fn modify_insight(
    Query(query): Query<InsightQuery>,
    Extension(claim): Extension<Claims>,
) -> impl IntoResponse {
    let action_type = match (query.action.as_str(), query.operation.as_str()) {
        ("like", "increment") => ActionType::IncrementLikes,
        ("like", "decrement") => ActionType::DecrementLikes,
        ("view", "increment") => ActionType::IncrementViews,
        _ => {
            return (
                StatusCode::NOT_ACCEPTABLE,
                Json(json!({
                    "error":"Invalid action"
                })),
            )
                .into_response()
        }
    };

    let action_message = InsightAction {
        insight_id: query.insight_id.clone(),
        action_type,
        user_id: claim.sub,
    };

    let message = serde_json::to_string(&action_message).expect("Failed to serialize");

    let connection = Connection::insecure_open(std::env::var("RABBITMQ_URL").unwrap().as_str());

    if let Err(e) = connection {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":e.to_string()
            })),
        )
            .into_response();
    }

    let mut connection = connection.unwrap();

    let channel = connection.open_channel(None);

    if let Err(e) = channel {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":e.to_string()
            })),
        )
            .into_response();
    }

    let channel = channel.unwrap();

    let exchange = channel.exchange_declare(
        ExchangeType::Direct,
        "insight_actions_exchange",
        ExchangeDeclareOptions::default(),
    );

    if let Err(e) = exchange {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":e.to_string()
            })),
        )
            .into_response();
    }

    let exchange = exchange.unwrap();

    let result = exchange.publish(Publish::new(message.as_bytes(), "routing_key"));

    if let Err(e) = result {
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
            "message":"Action sent to RabbitMQ"
        })),
    )
        .into_response()
}

pub fn db_actions_for_insight_stat(body: Json<Value>, pool: Arc<DbPool>) {
    let _insight_id = body["insight_id"].as_str().unwrap();
    let _action_type: ActionType = match body["action_type"].as_str().unwrap() {
        "IncrementLikes" => ActionType::IncrementLikes,
        "DecrementLikes" => ActionType::DecrementLikes,
        "IncrementViews" => ActionType::IncrementViews,
        _ => return,
    };
    let _user_id = body["user_id"].as_str().unwrap();

    let mut conn = match pool.get() {
        Ok(connection) => connection,
        Err(e) => {
            debug!("{}", e);
            return;
        }
    };

    use crate::schema::likes;
    match _action_type {
        ActionType::IncrementLikes => {
            diesel::update(likes::table.filter(likes::insight_id.eq(_insight_id)))
                .set(likes::like_count.eq(likes::like_count + 1))
                .execute(&mut conn)
                .expect("Failed to increment likes");

            let existing_user:User = match crate::schema::users::dsl::users
                .filter(crate::schema::users::user_id.eq(&_user_id))
                .first::<User>(&mut conn)
            {
                Ok(e_user) => e_user,
                Err(e) => {
                    debug!("{}", e);
                    return;
                }
            };

            if !existing_user.liked_insights.contains(&(_insight_id.to_string())){
                let mut new_liked_insights=existing_user.liked_insights.clone();
                new_liked_insights.push(_insight_id.to_string());

                diesel::update(crate::schema::users::table.filter(crate::schema::users::user_id.eq(_user_id)))
                .set(crate::schema::users::liked_insights.eq(new_liked_insights))
                .execute(&mut conn)
                .expect("Failed to update liked insights");
            }


        }
        ActionType::DecrementLikes => {
            diesel::update(likes::table.filter(likes::insight_id.eq(_insight_id)))
                .set(likes::like_count.eq(sql("GREATEST(like_count - 1, 0)")))
                .execute(&mut conn)
                .expect("Failed to decrement likes");

                let existing_user:User = match crate::schema::users::dsl::users
                .filter(crate::schema::users::user_id.eq(&_user_id))
                .first::<User>(&mut conn)
            {
                Ok(e_user) => e_user,
                Err(e) => {
                    debug!("{}", e);
                    return;
                }
            };

            if existing_user.liked_insights.contains(&(_insight_id.to_string())){
                let new_liked_insights: Vec<String> = existing_user
                .liked_insights
                .into_iter()
                .filter(|insight| insight != &_insight_id)
                .collect();

                diesel::update(crate::schema::users::table.filter(crate::schema::users::user_id.eq(_user_id)))
                .set(crate::schema::users::liked_insights.eq(new_liked_insights))
                .execute(&mut conn)
                .expect("Failed to update liked insights");
            }
            
        }
        ActionType::IncrementViews => {
            diesel::update(likes::table.filter(likes::insight_id.eq(_insight_id)))
                .set(likes::view_count.eq(likes::view_count + 1))
                .execute(&mut conn)
                .expect("Failed to increment views");

                let existing_user:User = match crate::schema::users::dsl::users
                .filter(crate::schema::users::user_id.eq(&_user_id))
                .first::<User>(&mut conn)
            {
                Ok(e_user) => e_user,
                Err(e) => {
                    debug!("{}", e);
                    return;
                }
            };

            if !existing_user.viewed_insights.contains(&(_insight_id.to_string())){
                let mut new_viewed_insights=existing_user.viewed_insights.clone();
                new_viewed_insights.push(_insight_id.to_string());

                diesel::update(crate::schema::users::table.filter(crate::schema::users::user_id.eq(_user_id)))
                .set(crate::schema::users::viewed_insights.eq(new_viewed_insights))
                .execute(&mut conn)
                .expect("Failed to update liked insights");
            }
        }
    }
}
