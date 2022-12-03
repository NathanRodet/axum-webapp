use axum::routing::post;
use axum::{Router, routing::get};

use crate::routes::index::hello_world;
use crate::routes::validate_data::validate_user;

pub async fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/validate_user", post(validate_user))
}