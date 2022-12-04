use axum::routing::post;
use axum::{routing::get, Router};

use crate::routes::index::hello_world;
use crate::routes::validate_data::custom_json_extractor;
use crate::routes::validate_data::validate_user;

pub async fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/validate_user", post(validate_user))
        .route("/custom_json_extractor", post(custom_json_extractor))
}
