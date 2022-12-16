use axum::extract::{Query, State};
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, EntityTrait,
    ModelTrait, QueryFilter, Set,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::database::tasks;

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct TaskRequest {
    #[validate(length(min = 3, max = 32, message = "must have between 3 and 32 characters"))]
    pub title: String,
    #[validate(length(max = 3, message = "must have maximum 3 characters"))]
    pub priority: Option<String>,
    #[validate(length(min = 3, max = 120, message = "must have between 3 and 32 characters"))]
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Validate, Debug)]
pub struct TaskResponse {
    pub id: i32,
    pub title: String,
    pub priority: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Validate, Debug)]
pub struct GetTaskQueryParams {
    #[validate(length(max = 3, message = "must have maximum 3 characters"))]
    pub priority: String,
}

pub async fn create_task(
    State(database_conn): State<DatabaseConnection>,
    Json(request): Json<TaskRequest>,
) -> Result<(), (StatusCode, String)> {
    if let Err(errors) = request.validate() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
    }

    let new_task = tasks::ActiveModel {
        title: Set(request.title),
        priority: Set(request.priority),
        description: Set(request.description),
        ..Default::default()
    };

    let _result = new_task
        .save(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, errors));

    Ok(())
}

pub async fn get_task(
    Path(id): Path<i32>,
    State(database_conn): State<DatabaseConnection>,
) -> impl IntoResponse {
    let task = tasks::Entity::find_by_id(id)
        .one(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, errors));

    if let Some(task) = task.unwrap() {
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

pub async fn get_all_tasks(
    State(database_conn): State<DatabaseConnection>,
    query_params: Option<Query<GetTaskQueryParams>>,
) -> Result<Json<Vec<TaskResponse>>, (StatusCode, String)> {
    let priority_filter = match query_params {
        Some(query_params) => {
            if let Err(errors) = query_params.validate() {
                return Err((StatusCode::BAD_REQUEST, format!("{}", errors)));
            }
            Condition::all().add(tasks::Column::Priority.eq(&*query_params.priority))
        }
        None => Condition::all(),
    };

    let tasks = tasks::Entity::find()
        .filter(priority_filter)
        .all(&database_conn)
        .await
        .map_err(|errors| (StatusCode::INTERNAL_SERVER_ERROR, errors.to_string()))?
        .into_iter()
        .map(|db_task| TaskResponse {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
        })
        .collect();

    Ok(Json(tasks))
}

pub async fn update_task(
    Path(id): Path<i32>,
    State(database_conn): State<DatabaseConnection>,
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

        task.title = Set(request.title);
        task.priority = Set(request.priority);
        task.description = Set(request.description);

        let task: tasks::Model = task
            .update(&database_conn)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

        Ok(Json({
            TaskRequest {
                title: task.title,
                priority: task.priority,
                description: task.description,
            }
        }))
    } else {
        Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
    }
}

pub async fn delete_task(
    Path(id): Path<i32>,
    State(database_conn): State<DatabaseConnection>,
) -> Result<(), (StatusCode, String)> {
    let task: Option<tasks::Model> = tasks::Entity::find_by_id(id)
        .one(&database_conn)
        .await
        .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;

    if let Some(task) = task {
        let task: tasks::Model = task.into();
        let res: DeleteResult = task
            .delete(&database_conn)
            .await
            .map_err(|error| (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()))?;
        assert_eq!(res.rows_affected, 1);
        Ok(())
    } else {
        Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
    }
}
