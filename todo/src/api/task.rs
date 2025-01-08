use crate::app::AppState;
use crate::errors::ErrorResponse;
use crate::models::task::{CreateTaskRequest, Task, TaskResponse};
use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/tasks",
    request_body = CreateTaskRequest,
    responses(
        (status = 201, description = "Task created successfully"),
        (status = 400, description = "Invalid data", body = ErrorResponse),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_task(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    if let Err(validation_error) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                message: validation_error,
            }),
        ));
    }

    let task: Task = payload.into();

    match state.task_repository.create(&task).await {
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
    path = "/api/tasks",
    responses(
        (status = 200, description = "List of tasks retrieved successfully", body = [TaskResponse]),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_tasks(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<TaskResponse>>, (StatusCode, Json<ErrorResponse>)> {
    match state.task_repository.list().await {
        Ok(tasks) => Ok(Json(TaskResponse::from_tasks(tasks))),
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
        .route("/", get(get_tasks))
    // .route("/:id", get(get_by_id))
    // .route("/:id", put(update_task))
    // .route("/:id", delete(delete_task))
}
