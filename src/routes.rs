use crate::handlers::{health_handler, info_handler, ui_handler};
use axum::{Router, routing::get};

/// Configure and return the application router
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(ui_handler))
        .route("/_/health", get(health_handler))
        .route("/_/info", get(info_handler))
}
