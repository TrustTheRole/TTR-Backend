use std::sync::Arc;

use axum::{http::Request, response::IntoResponse, Extension, Json};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use hyper::StatusCode;
use serde_json::{json, Value};
use diesel::OptionalExtension;

use crate::{
    db::DbPool, models::{misc::colleges::College, user::{UpdateUser, User}}, schema::colleges::{college_name, id, students_registered}, utils::{dispatch_email, generate_token, get_uid, Claims}
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
    let user_gender = match decrypted_json.get("gender") {
        Some(u_gender) => match u_gender.as_str() {
            Some(gender_str) => gender_str.to_string(),
            None => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json!({
                        "error": "Gender must be a string"
                    })),
                )
                    .into_response();
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Gender is required"
                })),
            )
                .into_response();
        }
    };

    if user_gender != "MALE".to_string() && user_gender != "FEMALE".to_string() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "error":"Gender can only be MALE or FEMALE"
            })),
        )
            .into_response();
    }

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

    



    let _user_id = get_uid();

    let user = User {
        user_id: _user_id.clone(),
        email: user_email.clone(),
        name: user_name.clone(),
        alternate_email: user_alternate_email,
        college: user_college.clone(),
        github: user_github,
        linkedin: user_linkedin,
        graduation_year: user_graduation_year,
        phone: user_phone,
        gender: user_gender,
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

    if let Some(_college_name) = user_college {
        let existing_college = crate::schema::colleges::dsl::colleges
            .filter(college_name.eq(&college_name))
            .first::<College>(&mut conn)
            .optional();

        match existing_college {
            Ok(Some(mut _college)) => {
                _college.students_registered += 1;
                let _ = diesel::update(crate::schema::colleges::dsl::colleges.filter(id.eq(&_college.id)))
                    .set(students_registered.eq(_college.students_registered))
                    .execute(&mut conn);
            }
            Ok(None) => {
                return (
                    StatusCode::NOT_FOUND,
                    Json(json!({
                        "error":"College does not exist"
                    })),
                ).into_response();
            }
            Err(e) => {
                tracing::debug!("{}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error":"Failed to update college information"
                    })),
                )
                .into_response();
            }
        }
    }

    

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

    let message = "Welcome to the community".to_string();

    let html_content = r#"
    <body style="margin: 0; padding: 0; background-color: #f4f4f4;">
    <table role="presentation" border="0" cellpadding="0" cellspacing="0" width="100%">
        <tr>
            <td align="center" style="padding: 10px;">
                <table role="presentation" border="0" cellpadding="0" cellspacing="0" width="600" style="border: 1px solid #cccccc; background-color: white;">
                    <tr>
                        <td align="center" style="padding: 40px 0 30px 0;">
                            <img src="https://ik.imagekit.io/s1vtpplq4/TTR.png?updatedAt=1722362280763" alt="TTR Logo" style="display: block; width: 35%; border: 0;">
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px; text-align: center; font-family: Arial, sans-serif; color: #333333;">
                            <table role="presentation" border="0" cellpadding="0" cellspacing="0" align="center">
                                <tr>
                                    <td style="vertical-align: middle;">
                                        <img src="https://ik.imagekit.io/s1vtpplq4/icons8-success-48.png?updatedAt=1722362334334" alt="Success Icon" style="display: block;">
                                    </td>
                                    <td style="vertical-align: middle; padding-left: 10px;">
                                        <h1 style="color: green; font-size: 24px; margin: 0;">You are successfully registered.</h1>
                                    </td>
                                </tr>
                            </table>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding: 20px; text-align: center; font-family: Arial, sans-serif; color: #333333;">
                            <p style="font-size: 16px; margin: 0;">
                                Welcome to #TTR! You're now part of our interview-savvy community. Ready for more insider tips? Subscribe to our newsletter and stay ahead of the curve. Your career journey just got a serious upgrade!
                            </p>
                        </td>
                    </tr>
                    <tr>
                        <td align="center" style="padding: 20px;">
                            <a href="https://ttr.gridsphere.io" style="background-color: #4CAF50; color: white; padding: 10px 20px; text-decoration: none; border-radius: 5px; display: inline-block; font-family: Arial, sans-serif;">SUBSCRIBE</a>
                        </td>
                    </tr>
                </table>
            </td>
        </tr>
    </table>
</body>


    "#;

    dispatch_email(
        &user_name,
        &user_email,
        &message,
        "Welcome to the TTR Community".to_string(),
        &html_content,
    )
    .await;

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


pub async fn update_user_details(
    Extension(claim): Extension<Claims>,
    Extension(pool): Extension<Arc<DbPool>>,
    Json(req): Json<Value>,
) -> impl IntoResponse {
    let _user_id = claim.sub;
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

    match crate::schema::users::dsl::users
        .filter(user_id.eq(&_user_id))
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

    let update_user = UpdateUser {
        name: req.get("name").and_then(|v| v.as_str()),
        role: req.get("role").and_then(|v| v.as_str()),
        alternate_email: req.get("alternate_email").and_then(|v| v.as_str()),
        phone: req.get("phone").and_then(|v| v.as_str()),
        college: req.get("college").and_then(|v| v.as_str()),
        graduation_year: req.get("graduation_year").and_then(|v| v.as_i64()).map(|y| y as i32),
        linkedin: req.get("linkedin").and_then(|v| v.as_str()),
        github: req.get("github").and_then(|v| v.as_str()),
        gender: req.get("gender").and_then(|v| v.as_str()),
    };

    match diesel::update(users.filter(user_id.eq(&_user_id)))
        .set(update_user)
        .execute(&mut conn)
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({"message": "User details updated successfully"})),
        )
            .into_response(),
        Err(e) => {
            tracing::debug!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Failed to update user details"
                })),
            )
                .into_response()
        }
    }
}