use axum::extract::Query;
use axum::response::Response;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, EntityTrait,
    ModelTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::database::tasks;

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct TaskRequest {
    #[validate(length(min = 3, max = 32, message = "must have between 3 and 32 characters"))]
    pub title: String,
    #[validate(length(min = 3, message = "must have maximum 3 characters"))]
    pub priority: Option<String>,
    #[validate(length(min = 3, max = 120, message = "must have between 3 and 32 characters"))]
    pub description: Option<String>,
}

// This is the post route handler for creating a new task
pub async fn create_task(
    Extension(database_conn): Extension<DatabaseConnection>,
    Json(request): Json<TaskRequest>,
) {
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

#[derive(Deserialize, Serialize, Debug)]
pub struct TaskResponse {
    pub id: i32,
    pub title: String,
    pub priority: Option<String>,
    pub description: Option<String>,
}

// This is the get route handler for a single task
pub async fn get_task(
    Path(id): Path<i32>,
    Extension(database_conn): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    // Find the task by id
    let task = tasks::Entity::find_by_id(id)
        .one(&database_conn)
        .await
        .unwrap();

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

#[derive(Deserialize, Debug)]
pub struct GetTaskQueryParams {
    pub priority: String,
}

// This is the get route handler for all tasks
pub async fn get_all_task(
    Extension(database_conn): Extension<DatabaseConnection>,
    query_params: Option<Query<GetTaskQueryParams>>,
) -> Result<Json<Vec<TaskResponse>>, StatusCode> {
    let priority_filter = match query_params {
        Some(query_params) => {
            Condition::all().add(tasks::Column::Priority.eq(&*query_params.priority))
        }
        None => Condition::all(),
    };

    // Find all tasks
    let tasks = tasks::Entity::find()
        .filter(priority_filter)
        .all(&database_conn)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_task| TaskResponse {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
        })
        .collect();

    // Return all tasks
    Ok(Json(tasks))
}

// enum CustomError {
//     NotFound,
//     InternalServerError,
//     Validation(ValidationErrors),
// }

// impl IntoResponse for CustomError {
//     fn into_response(self) -> Response {
//         match self {
//             CustomError::NotFound => (StatusCode::INTERNAL_SERVER_ERROR,String::from""),
//             CustomError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, ""),
//             CustomError::Validation(_errors) => (StatusCode::INTERNAL_SERVER_ERROR,"")
//         }.into_response()

//     }
// }


// This is the put route handler for updating a task
#[axum_macros::debug_handler]
pub async fn update_task(
    Path(id): Path<i32>,
    Extension(database_conn): Extension<DatabaseConnection>,
    Json(request): Json<TaskRequest>,
) -> Result<Json<TaskRequest>, (StatusCode, String)> {

    if let Err(errors) = request.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
    }

    let task: Option<tasks::Model> = tasks::Entity::find_by_id(id)
        .one(&database_conn)
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

    if let Some(task) = task {
        let mut task: tasks::ActiveModel = task.into();

        // Update attributes
        task.title = Set(request.title);
        task.priority = Set(request.priority);
        task.description = Set(request.description);

        // Update corresponding row in database using primary key value
        let task: tasks::Model = task
            .update(&database_conn)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

        // Return the updated data for task
        Ok(Json({
            TaskRequest {
                title: task.title,
                priority: task.priority,
                description: task.description,
            }
        }), )
    } else {
        Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
    }
}

// This is the delete route handler for deleting a task
pub async fn delete_task(
    Path(id): Path<i32>,
    Extension(database_conn): Extension<DatabaseConnection>,
) -> Result<(), StatusCode> {
    // Find the task by id
    let task: Option<tasks::Model> = tasks::Entity::find_by_id(id)
        .one(&database_conn)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    let task: tasks::Model = task.unwrap();
    // Delete the task
    let res: DeleteResult = task
        .delete(&database_conn)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    // Check if the task was deleted
    assert_eq!(res.rows_affected, 1);

    Ok(())
}
