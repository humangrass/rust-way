use crate::app::AppState;
use axum::Router;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

pub mod task;

#[derive(OpenApi)]
#[openapi(
    paths(
        task::create_task,
        task::get_tasks,
        task::get_task,
        task::update_task,
        task::delete_task,
    ),
    tags(
        (name = "TODO application", description = "API для управления задачами"),
    )
)]
struct ApiDoc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_router = Router::new().nest("/api/task", task::router());

    Router::new()
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .merge(api_router)
        .layer(axum::Extension(app_state.task_repository.clone()))
        .layer(axum::Extension(app_state.token_manager.clone()))
}
