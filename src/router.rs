use axum::Extension;
use axum::routing::post;
use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;
use crate::routes::tasks::{create_task, get_task};
use crate::routes::index::hello_world;
use crate::routes::validate_data::custom_json_extractor;
use crate::routes::validate_data::validate_user;

// This is the function that creates the routes
pub async fn create_routes(database_conn: DatabaseConnection) -> Router {

    Router::new()
        .route("/", get(hello_world))
        .route("/validate_user", post(validate_user))
        .route("/custom_json_extractor", post(custom_json_extractor))
        .route("/task", post(create_task))
        .route("/task/:id", get(get_task))
        .layer(Extension(database_conn))

}

