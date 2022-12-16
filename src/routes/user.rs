use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::database::users;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
    #[validate(length(min = 8, message = "must have at least 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateUserResponse {
    pub id: i32,
    pub username: String,
}

pub async fn create_user(
    State(database_conn): State<DatabaseConnection>,
    Json(user_request): Json<CreateUserRequest>,
) -> Result<Json<CreateUserResponse>, (StatusCode, String)> {
    if let Err(errors) = user_request.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
    }

    let new_user = users::ActiveModel {
        username: Set(user_request.username),
        password: Set(user_request.password),
        ..Default::default()
    };

    let new_user = new_user
        .save(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?
        .try_into_model()
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?;

    Ok(Json(CreateUserResponse {
        id: new_user.id,
        username: new_user.username,
    }))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct GetAllUsersResponse {
    pub id: i32,
    pub username: String,
}

pub async fn get_all_users() -> Result<Json<Vec<GetAllUsersResponse>>, (StatusCode, String)> {
    
    todo!()
}
