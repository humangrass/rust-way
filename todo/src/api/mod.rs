use std::sync::Arc;
use axum::Router;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};
use crate::app::AppState;

pub mod task;

#[derive(OpenApi)]
#[openapi(
    paths(
        task::create_task
    ),
    tags(
        (name = "TODO application", description = "API для управления задачами"),
    )
)]
struct ApiDoc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_router = Router::new()
        .nest("/api/tasks", task::router());

    Router::new()
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .merge(api_router)
        .layer(axum::Extension(app_state))
}