use axum::{
    routing::{get, post},
    Router,
};
use dotenvy_macro::dotenv;
use sea_orm::Database;

use crate::{
    routes::{
        index::hello_world,
        task::{create_task, delete_task, get_all_tasks, get_task, update_task},
    },
    server::AppState,
};

pub async fn app_test() -> Router {
    let database_uri = dotenv!("DATABASE_URL").to_owned();
    let jwt_secret = dotenv!("JWT_SECRET").to_owned();

    let database_conn = Database::connect(database_uri).await.unwrap();

    let app_state = AppState {
        database_conn,
        jwt_secret,
    };

    let guest_nest = Router::new().route("/", get(hello_world));

    let user_nest = Router::new()
        .route("/", post(create_task).get(get_all_tasks))
        .route("/:id", get(get_task).put(update_task).delete(delete_task));

    Router::new()
        .nest("", guest_nest)
        .nest("/task", user_nest)
        .with_state(app_state)
}
