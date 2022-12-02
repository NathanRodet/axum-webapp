use axum::{Router, routing::get};

use crate::routes::index::hello_world;

pub async fn create_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
}