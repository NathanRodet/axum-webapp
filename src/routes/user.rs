use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DeleteResult, EntityTrait, ModelTrait,
    QueryFilter, Set, TryIntoModel,
};
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

    if let Some(_new_user) = users::Entity::find()
        .filter(users::Column::Username.eq(user_request.username.clone()))
        .one(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?
    {
        return Err((StatusCode::BAD_REQUEST, format!("User already exists")));
    }

    let new_user = hash(user_request.password, DEFAULT_COST)
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?;

    let new_user = users::ActiveModel {
        username: Set(user_request.username),
        password: Set(new_user),
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
    pub password: String,
}

#[axum_macros::debug_handler]
pub async fn get_all_users(
    State(database_conn): State<DatabaseConnection>,
) -> Result<Json<Vec<GetAllUsersResponse>>, (StatusCode, String)> {
    let users: Vec<GetAllUsersResponse> = users::Entity::find()
        .all(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?
        .into_iter()
        .map(|users| GetAllUsersResponse {
            id: users.id,
            username: users.username,
            password: users.password,
        })
        .collect();

    Ok(Json(users))
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct DeleteUserByUsernameRequest {
    #[validate(email(message = "must be a valid email"))]
    pub username: String,
}

pub async fn delete_user_by_username(
    Path(username): Path<DeleteUserByUsernameRequest>,
    State(database_conn): State<DatabaseConnection>,
) -> Result<(), (StatusCode, String)> {
    if let Err(errors) = username.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
    }

    let user = users::Entity::find()
        .filter(users::Column::Username.eq(username.username))
        .one(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", errors)))?;

    if let Some(user) = user {
        let user: users::Model = user.into();
        let res: DeleteResult = user
            .delete(&database_conn)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
        assert_eq!(res.rows_affected, 1);
        Ok(())
    } else {
        Err((StatusCode::NOT_FOUND, "User not found".to_string()))
    }
}
