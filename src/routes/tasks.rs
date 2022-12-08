use axum::{Extension, Json, extract::Path, response::IntoResponse, http::StatusCode};
use sea_orm::{DatabaseConnection, Set, ActiveModelTrait, EntityTrait};
use serde::{Serialize, Deserialize};
use validator::Validate;

use crate::database::tasks;

// This is the JSON body
#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct TaskRequest {
    pub title: String,
    pub priority: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct TaskResponse {
    pub id: i32,
    pub title: String,
    pub priority: Option<String>,
    pub description: Option<String>,
}

// This is the post route handler for creating a new task
pub async fn create_task(Extension(database_conn): Extension<DatabaseConnection>, Json(request): Json<TaskRequest>) {
    // Validate the request
    let new_task = tasks::ActiveModel {
        title: Set(request.title),
        priority: Set(request.priority),
        description: Set(request.description),
        ..Default::default()
    };
    // Save the new task to the database
    let result = new_task.save(&database_conn).await.unwrap();
    // Output the result in the console
    dbg!(result);
}

// This is the get route handler for a single task
pub async fn get_task(Path(id): Path<i32>, Extension(database_conn): Extension<DatabaseConnection>) -> impl IntoResponse {
    // Find the task by id
    let task = tasks::Entity::find_by_id(id).one(&database_conn).await.unwrap();

    // Return the task
    if let Some(task) = task {
        let response = TaskResponse {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
        };
        Ok(Json(response))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}