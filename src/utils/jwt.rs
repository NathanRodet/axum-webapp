use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    Json, RequestPartsExt, TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    id: i32,
    is_admin: bool,
    exp: usize,
    iat: usize,
}

pub async fn create_token(jwt_secret: String, id: i32) -> Result<String, String> {
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
    .map_err(|errors| errors.to_string())?;

    Ok(token)
}

async fn decode_token(jwt_secret: String, token: String) -> Result<Json<Claims>, String> {
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|errors| errors.to_string())?;

    return Ok(Json(token_data.claims));
}

pub async fn refresh_token(jwt_secret: String, token: String) -> Result<String, String> {
    let token = decode_token(jwt_secret.clone(), token)
        .await
        .map_err(|errors| errors.to_string())?;

    let claims = Claims {
        id: token.id,
        is_admin: token.is_admin,
        exp: token.exp,
        iat: token.iat,
    };

    let token = create_token(jwt_secret, claims.id)
        .await
        .map_err(|errors| errors.to_string())?;

    Ok(token)
}