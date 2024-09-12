use chrono::{Duration, Utc};
use diesel::{deserialize::FromSqlRow, expression::AsExpression, query_dsl::methods::FilterDsl, r2d2::{ConnectionManager, PooledConnection}, sql_types::Text, ExpressionMethods, PgConnection, RunQueryDsl};
use jsonwebtoken::{decode, encode, errors::Error, DecodingKey, EncodingKey, Header};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::models::tag::Tag;

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
    .timestamp();

    let expiration_usize = expiration.try_into().expect("timestamp conversion failed");

    let claims = Claims {
        sub: user_id,
        exp: expiration_usize,
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

pub async fn dispatch_email(fullname: &str, email: &str, message: &str, email_subject: String, html_content: &str) {
    println!("Sending email to {}", email);
    println!("Full Name: {}", fullname);

    let admin_email = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not specified");
    let admin_name = env::var("ADMIN_NAME").expect("ADMIN_NAME not specified");


    let from_address = format!("{} <{}>", admin_name, admin_email);
    let to_address = format!("{} <{}>", fullname, email);
    
    
    
    let reply_to = format!("{} <{}>", admin_name, admin_email);

    let email = Message::builder()
        .from(from_address.parse().expect("Invalid from address format"))
        .reply_to(reply_to.parse().expect("Invalid reply-to address format"))
        .to(to_address.parse().expect("Invalid to address format"))
        .subject(email_subject)
        .multipart(lettre::message::MultiPart::alternative_plain_html(
            message.to_string(),
            html_content.to_string(),
        ))
        .expect("Failed to build the email");

    let creds = Credentials::new(
        env::var("SMTP_USERNAME").expect("SMTP Username not specified"),
        env::var("SMTP_PASSWORD").expect("SMTP Password not specified"),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => eprintln!("Could not send email: {:?}", e),
    }
}

pub fn extract_tags(tags:&Vec<String>,conn:&mut PooledConnection<ConnectionManager<PgConnection>>){

    for tag in tags{
        let tag_exists = crate::schema::tags::dsl::tags
            .filter(crate::schema::tags::dsl::name.eq(tag))
            .first::<Tag>(conn)
            .is_ok();

        if !tag_exists {
            let new_tag = Tag {
                name: tag.to_string(),
                created_at: chrono::Utc::now().naive_utc(),
            };

            diesel::insert_into(crate::schema::tags::dsl::tags)
                .values(&new_tag)
                .execute(conn)
                .expect("Failed to insert tag into database");
        }
    }

}
