use crate::routes::auth::login;
use crate::routes::index::hello_world;
use crate::routes::task::{create_task, delete_task, get_all_tasks, get_task, update_task};
use crate::routes::user::{create_user, delete_user_by_username, get_all_users};
use crate::server::AppState;
use axum::routing::{delete, post, put};
use axum::{routing::get, Router};

pub async fn create_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/task", post(create_task))
        .route("/task", get(get_all_tasks))
        .route("/task/:id", get(get_task))
        .route("/task/:id", put(update_task))
        .route("/task/:id", delete(delete_task))
        .route("/user", post(create_user))
        .route("/user", get(get_all_users))
        .route("/user/:username", delete(delete_user_by_username))
        .route("/login", post(login))
        .with_state(app_state)
}
