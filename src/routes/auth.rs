use axum::{extract::State, http::StatusCode, Json};
use bcrypt::verify;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    database::users,
    utils::jwt::{create_token, refresh_token},
};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AuthRequest {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AuthResponse {
    pub token: String,
}

pub struct User {
    pub id: i32,
    pub username: String,
}

pub async fn login(
    State(jwt_secret): State<String>,
    State(database_conn): State<DatabaseConnection>,
    Json(user_request): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    if let Err(errors) = user_request.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
    }

    let user = users::Entity::find()
        .filter(users::Column::Username.eq(user_request.username.clone()))
        .one(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?;

    match user {
        Some(user) => {
            let is_valid = verify(&user_request.password, &user.password)
                .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?;

            if !is_valid {
                return Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string()));
            }

            let token = create_token(jwt_secret, user.id)
                .await
                .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", errors)))?;

            return Ok(Json(AuthResponse { token }));
        }
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RenewRequest {
    #[validate(length(min = 160, max = 160, message = "Token is not valid"))]
    pub token: String,
}

pub async fn renew_auth(
    State(jwt_secret): State<String>,
    Json(user_request): Json<RenewRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    if let Err(errors) = user_request.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
    }

    let token = refresh_token(jwt_secret, user_request.token).await?;

    Ok(Json(AuthResponse { token }))
}
