use std::sync::Arc;

use amiquip::{Connection, ExchangeDeclareOptions, ExchangeType, Publish};
use axum::{response::IntoResponse, Extension, Json};
use diesel::RunQueryDsl;
use futures::future::join_all;
use hyper::StatusCode;
use log::debug;
use serde_json::{json, Value};

use crate::{
    db::DbPool,
    models::{actions::EmailAction, 
        misc::{colleges::College, companies::Companies, newsletter_sub::NewsletterSub}}
    ,
    utils::{dispatch_email, get_uid},
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

    let _college = College {
        id: get_uid(),
        college_name: _college_name,
        college_location: _college_location,
        students_registered: 0,
    };

    match diesel::insert_into(crate::schema::colleges::dsl::colleges)
        .values(&_college)
        .execute(&mut conn)
    {
        Ok(_) => (),
        Err(e) => {
            debug!("{}", e);
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

    match diesel::insert_into(crate::schema::companies::dsl::companies)
        .values(&_company)
        .execute(&mut conn)
    {
        Ok(_) => (),
        Err(e) => {
            debug!("{}", e);
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
            debug!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let newsletter = NewsletterSub {
        email: _user_email,
        created_at: chrono::Utc::now().naive_utc(),
    };

    if let Err(e) = diesel::insert_into(crate::schema::newsletter_sub::dsl::newsletter_sub)
        .values(&newsletter)
        .execute(&mut conn)
    {
        debug!("{:?}", e);
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
            debug!("{:?}", e);
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
        crate::schema::newsletter_sub::dsl::newsletter_sub.load::<NewsletterSub>(&mut conn);

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
            debug!("{:?}", e);
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
            debug!("{:?}", e);
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

pub async fn send_newsletter(
    Json(req): Json<Value>,
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let _newsletter_title = match req.get("newsletter_title") {
        Some(title) => match title.as_str() {
            Some(title_str) => title_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Title must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Title is required"
                })),
            )
                .into_response();
        }
    };


    let _title = match req.get("title") {
        Some(title) => match title.as_str() {
            Some(title_str) => title_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Title must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Title is required"
                })),
            )
                .into_response();
        }
    };

    let _content = match req.get("content") {
        Some(content) => match content.as_str() {
            Some(content_str) => content_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error":"Content must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error":"Content is required"
                })),
            )
                .into_response();
        }
    };

    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            debug!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let subscribers = crate::schema::newsletter_sub::dsl::newsletter_sub
        .load::<crate::models::misc::newsletter_sub::NewsletterSub>(&mut conn);

    if let Err(e) = subscribers {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error":e.to_string()
            })),
        )
            .into_response();
    }
    let subscribers: Vec<NewsletterSub> = subscribers.unwrap();

    let html_content = format!(
        r#"
        <div style="width: 80%; margin: auto; background-color: #ffffff; padding: 20px; box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);">
    <div style="background-color: #4CAF50; color: white; padding: 10px 0; text-align: center; border-radius: 6px;">
        <h1>{}</h1>
    </div>
    <div style="margin: 20px 0;">
        <p>
            Hey there,
        </p>
        <p>
            Welcome to our monthly newsletter! We are excited to share the latest updates and insights with you.
        </p>
        <h2>{}</h2>
        <p>
            {}
        </p>
        <p>
            Thank you for being a part of our community.
        </p>
    </div>
    <div style="text-align: center; padding: 10px 0; background-color: #f1f1f1;">
        <p>
            © 2024 TTR. All rights reserved.
        </p>
    </div>
</div>
        "#,_newsletter_title,_title,_content
    );

    let message = "Hey there! We are excited to share the latest updates and insights with you.".to_string();

    let subject = "Welcome to our monthly newsletter!".to_string();

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
        "email_actions_exchange",
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

    for subsriber in &subscribers {
        let email_action = EmailAction{
            user_name: "User".to_string(),
            user_email: subsriber.email.clone(),
            message: message.clone(),
            subject: subject.clone(),
            html_content: html_content.clone(),
        };
        let message = serde_json::to_string(&email_action).expect("Failed to serialize email action");
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
    }

    (
        StatusCode::OK,
        Json(json!({
            "message":"Newsletter sent successfully"
        })),
    )
        .into_response()
}

pub async fn get_all_companies(
    Extension(pool): Extension<Arc<DbPool>>,
) -> impl IntoResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            debug!("{:?}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Database connection failed"
                })),
            )
                .into_response();
        }
    };

    let companies = crate::schema::companies::dsl::companies
        .load::<Companies>(&mut conn);

    if let Err(e) = companies {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": e.to_string(),
            })),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({
            "companies": companies.unwrap()
        })),
    )
        .into_response()
}
