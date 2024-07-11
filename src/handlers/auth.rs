use std::env;

use axum::{
    extract::Query,
    http::header::{HeaderValue, SET_COOKIE},
    response::{IntoResponse, Redirect},
    Json,
};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct AuthQuery {
    pub code: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

fn generate_state() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}

pub async fn authorize() -> Redirect {
    tracing::debug!("Authorizing user");
    let client_id = env::var("KINDE_CLIENT_ID").unwrap();
    let redirect_uri = "http://localhost:8080/token_exchange";
    let state = generate_state();

    let auth_url=format!(
            "https://trusttherole.kinde.com/oauth2/auth?response_type=code&client_id={}&redirect_uri={}&scope=openid+profile+email&state={}",
            client_id, redirect_uri, state
        );

    Redirect::to(&auth_url)
}

pub async fn token_exchange(
    Query(params): Query<AuthQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(code) = params.code {
        let client_id = env::var("KINDE_CLIENT_ID").unwrap();
        let client_secret = env::var("KINDE_CLIENT_SECRET").unwrap();
        let redirect_uri = "http://localhost:8080/token_exchange";

        let client = Client::new();
        let response = client
            .post("https://trusttherole.kinde.com/oauth2/token")
            .form(&[
                ("grant_type", "authorization_code"),
                ("code", &code),
                ("redirect_uri", redirect_uri),
                ("client_id", &client_id),
                ("client_secret", &client_secret),
            ])
            .send()
            .await
            .unwrap();

        if response.status().is_success() {
            let token_response: TokenResponse = response.json().await.unwrap();
            let json_response = Json({
                json!({
                    "message": "Successfully exchanged authorization code for token",
                })
            });
            let mut response = json_response.into_response();

            let cookie_value = format!(
                "access_token={}; HttpOnly; Secure; SameSite=Strict",
                token_response.access_token
            );
            response.headers_mut().insert(
                SET_COOKIE,
                HeaderValue::from_str(&cookie_value).expect("Invalid cookie"),
            );

            Ok(response)
        } else {
            Err(response.status())
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
