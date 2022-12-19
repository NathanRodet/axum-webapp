use crate::routes::auth::{login, renew_auth};
use crate::routes::index::hello_world;
use crate::routes::task::{create_task, delete_task, get_all_tasks, get_task, update_task};
use crate::routes::user::{create_user, delete_user_by_username, get_all_users};
use crate::server::AppState;
use axum::routing::{delete, get, post};
use axum::Router;

pub async fn create_routes(app_state: AppState) -> Router {
    // let task_nest = Router::new()
    //     .route("/", post(create_task).get(get_all_tasks))
    //     .route("/:id", get(get_task).put(update_task).delete(delete_task));
    // https://docs.rs/axum/0.2.3/axum/routing/struct.Router.html

    let guest_nest = Router::new()
        .route("/", get(hello_world))
        .route("/login", post(login))
        .route("/register", post(create_user))
        .route("/renew_auth", post(renew_auth));

    let user_nest = Router::new()
        .route("/", post(create_task).get(get_all_tasks))
        .route("/:id", get(get_task).put(update_task).delete(delete_task));

    let admin_nest = Router::new()
        .route("/", get(get_all_users))
        .route("/:username", delete(delete_user_by_username));

    Router::new()
        .nest("", guest_nest)
        .nest("/task", user_nest)
        .nest("/user", admin_nest)
        .with_state(app_state)
}
