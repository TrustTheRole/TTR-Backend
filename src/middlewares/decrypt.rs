use axum::{
    body::{Body, HttpBody},
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};
use base64::{engine::general_purpose, Engine};
use rsa::{pkcs1::DecodeRsaPrivateKey, Oaep, RsaPrivateKey};
use serde_json::{json, Value};
use sha2::Sha256;
use std::fs;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub async fn decrypt_data<B: std::fmt::Debug>(
    request: Request<B>,
    next: Next<Body>,
) -> Result<impl IntoResponse, impl IntoResponse>
where
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    let private_key_pem = match fs::read_to_string("private_key.pem") {
        Ok(private_pem) => private_pem,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":e.to_string()
                })),
            )
                .into_response());
        }
    };

    let private_key = match RsaPrivateKey::from_pkcs1_pem(&private_key_pem) {
        Ok(priv_key) => priv_key,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":e.to_string()
                })),
            )
                .into_response());
        }
    };

    let (parts, body) = request.into_parts();
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":"error parsing body"
                })),
            )
                .into_response());
        }
    };

    let json: Value = match serde_json::from_slice(&bytes) {
        Ok(json_data) => json_data,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error":e.to_string()
                })),
            )
                .into_response())
        }
    };

    let data = match json.get("encrypted_data") {
        Some(data) => data,
        None => {
            return Err((
                StatusCode::EXPECTATION_FAILED,
                Json(json!({
                    "error":"Please provide the data"
                })),
            )
                .into_response());
        }
    };
    let encypted_data = match data {
        Value::String(s) => s.clone(),
        _ => data.to_string(),
    };

    let cipher_text = match general_purpose::STANDARD.decode(encypted_data) {
        Ok(c_text) => c_text,
        Err(e) => {
            return Err((
                StatusCode::EXPECTATION_FAILED,
                Json(json!({
                    "error":e.to_string()
                })),
            )
                .into_response());
        }
    };

    let padding = Oaep::new::<Sha256>();
    let decrypted_data = match private_key.decrypt(padding, &cipher_text) {
        Ok(d_data) => d_data,
        Err(e) => {
            return Err((
                StatusCode::EXPECTATION_FAILED,
                Json(json!({
                    "error":e.to_string()
                })),
            )
                .into_response())
        }
    };

    let decrypted_string = match String::from_utf8(decrypted_data) {
        Ok(d_data) => d_data,
        Err(e) => {
            return Err((
                StatusCode::EXPECTATION_FAILED,
                Json(json!({
                    "error":e.to_string()
                })),
            )
                .into_response())
        }
    };

    let decrypted_json: Value = match serde_json::from_str(&decrypted_string) {
        Ok(json_data) => json_data,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!("Failed to parse decrypted data as JSON: {}", e)
                })),
            )
                .into_response());
        }
    };

    tracing::debug!("{}", decrypted_json);

    let mut request = Request::from_parts(parts, Body::from(bytes));
    request.extensions_mut().insert(decrypted_json);
    Ok(next.run(request).await)
}
