use axum::{extract::State, http::StatusCode};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    is_admin: bool,
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
}

pub async fn create_token(
    State(jwt_secret): State<String>,
) -> Result<String, (StatusCode, String)> {
    let created_at = Utc::now();
    let expires_at = created_at + Duration::hours(24);

    let claims = Claims {
        is_admin: false,
        exp: expires_at.timestamp() as usize,
        iat: created_at.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?;

    Ok(token)
}
