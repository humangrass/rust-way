use crate::app::AppState;
use crate::errors::ErrorResponse;
use crate::models::task::{Task, TaskModel, TaskRequest, TaskResponse};
use crate::repository::task::TaskRepository;
use auth::tokens::TokenManager;
use auth::AuthenticatedUser;
use axum::{
    extract::{Extension, Json, Path},
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use log::info;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/task",
    request_body = TaskRequest,
    responses(
        (status = 201, description = "Task created successfully"),
        (status = 400, description = "Invalid data", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_task(
    AuthenticatedUser { id }: AuthenticatedUser,
    Extension(_token_manager): Extension<Arc<TokenManager>>,
    Extension(task_repository): Extension<Arc<TaskRepository>>,
    Json(payload): Json<TaskRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    info!("Hello user {}", id);
    if let Err(validation_error) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: validation_error,
            }),
        ));
    }

    let task: Task = payload.into();

    match task_repository.create(&TaskModel::from(task)).await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: "Internal server error.".to_string(),
            }),
        )),
    }
}

#[utoipa::path(
    get,
    path = "/api/task/list",
    responses(
        (status = 200, description = "List of tasks retrieved successfully", body = [TaskResponse]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_tasks(
    Extension(task_repository): Extension<Arc<TaskRepository>>,
) -> Result<Json<Vec<TaskResponse>>, (StatusCode, Json<ErrorResponse>)> {
    match task_repository.list().await {
        Ok(models) => {
            let tasks: Vec<Task> = models.into_iter().map(Task::from).collect();
            Ok(Json(TaskResponse::from_tasks(tasks)))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: "Internal server error.".to_string(),
            }),
        )),
    }
}

#[utoipa::path(
    get,
    path = "/api/task/{id}",
    responses(
        (status = 200, description = "List of tasks retrieved successfully", body = [TaskResponse]),
        (status = 404, description = "Task not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Task ID")
    )
)]
pub async fn get_task(
    Path(id): Path<i32>,
    Extension(task_repository): Extension<Arc<TaskRepository>>,
) -> Result<Json<TaskResponse>, (StatusCode, Json<ErrorResponse>)> {
    match task_repository.by_id(id).await {
        Ok(Some(model)) => Ok(Json(TaskResponse::from(Task::from(model)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "Task not found.".to_string(),
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: "Internal server error.".to_string(),
            }),
        )),
    }
}

#[utoipa::path(
    put,
    path = "/api/task/{id}",
    request_body = TaskRequest,
    responses(
        (status = 200, description = "Task updated successfully", body = TaskResponse),
        (status = 400, description = "Validation error", body = ErrorResponse),
        (status = 404, description = "Task not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    ),
    params(
        ("id" = i32, Path, description = "Task ID")
    )
)]
pub async fn update_task(
    Extension(task_repository): Extension<Arc<TaskRepository>>,
    Path(id): Path<i32>,
    Json(payload): Json<TaskRequest>,
) -> Result<Json<TaskResponse>, (StatusCode, Json<ErrorResponse>)> {
    if let Err(validation_error) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: validation_error,
            }),
        ));
    }

    let task: Task = payload.into();

    match task_repository.update(id, &TaskModel::from(task)).await {
        Ok(Some(model)) => Ok(Json(TaskResponse::from(Task::from(model)))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "Task not found.".to_string(),
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: "Internal server error.".to_string(),
            }),
        )),
    }
}

#[utoipa::path(
    delete,
    path = "/api/task/{id}",
    responses(
        (status = 200, description = "Task deleted successfully"),
        (status = 404, description = "Task not found", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "Task ID")
    )
)]
pub async fn delete_task(
    Extension(task_repository): Extension<Arc<TaskRepository>>,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    match task_repository.delete(id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(sqlx::Error::RowNotFound) => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                message: "Task not found.".to_string(),
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: "Internal server error.".to_string(),
            }),
        )),
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/", post(create_task))
        .route("/list", get(get_tasks))
        .route("/{id}", get(get_task))
        .route("/{id}", put(update_task))
        .route("/{id}", delete(delete_task))
}
