use crate::app::AppState;
use axum::Router;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

#[derive(OpenApi)]
#[openapi(
    paths(),
    tags(
        (name = "Bartender", description = "Authentication service"),
    )
)]
struct ApiDoc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_router = Router::new();

    Router::new()
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
        .merge(api_router)
        .layer(axum::Extension(app_state))
}
