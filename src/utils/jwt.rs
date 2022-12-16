use axum::{http::StatusCode, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    id: i32,
    is_admin: bool,
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
}

pub async fn create_token(jwt_secret: String, id: i32) -> Result<String, (StatusCode, String)> {
    let created_at = Utc::now();
    let expires_at = created_at + Duration::hours(24);

    let claims = Claims {
        id,
        is_admin: false,
        exp: expires_at.timestamp() as usize,
        iat: created_at.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, errors.to_string()))?;

    Ok(token)
}

async fn decode_token(
    jwt_secret: String,
    token: String,
) -> Result<Json<Claims>, (StatusCode, String)> {
    let token_message = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_errors| (StatusCode::UNAUTHORIZED, "Token is not valid".to_string()))?;

    return Ok(Json(token_message.claims));
}

pub async fn refresh_token(
    jwt_secret: String,
    token: String,
) -> Result<String, (StatusCode, String)> {
    let token = decode_token(jwt_secret.clone(), token).await?;
    let token = create_token(jwt_secret, token.id).await?;

    Ok(token)
}
