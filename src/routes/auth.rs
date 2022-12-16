use axum::{extract::State, http::StatusCode, Json};
use bcrypt::verify;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{database::users, utils::jwt::create_token};

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

            let token = create_token(jwt_secret).await?;

            return Ok(Json(AuthResponse { token }));
        }
        None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    }
}
