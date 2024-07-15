use chrono::{Duration, Utc};
use diesel::{deserialize::FromSqlRow, expression::AsExpression, sql_types::Text};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = Text)]
pub enum Role {
    Admin,
    User,
    Guest,
}
pub fn get_uid() -> String {
    let uid = Uuid::new_v4().to_string();
    uid
}

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_token(user_id: String) -> Result<String, Error> {
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
