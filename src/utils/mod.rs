use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types::Text};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header};
use std::env;

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Role {
    SuperAdmin,
    Professional,
    Student,
}

pub fn get_role(role: &str) -> Role {
    match role {
        "super_admin" => Role::SuperAdmin,
        "professional" => Role::Professional,
        "student" => Role::Student,
        _ => Role::Student,
    }
}

pub fn get_role_str(role: &Role) -> &str {
    match role {
        Role::SuperAdmin => "super_admin",
        Role::Professional => "professional",
        Role::Student => "student",
    }
}

pub fn get_uid() -> String {
    Uuid::new_v4().to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn genrate_token(user_id: String) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &jsonwebtoken::Validation::default(),
    )?;

    Ok(token_data.claims)
}
