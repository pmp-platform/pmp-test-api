use crate::handlers::{health_handler, info_handler};
use axum::{routing::get, Router};

/// Configure and return the application router
pub fn create_router() -> Router {
    Router::new()
        .route("/_/health", get(health_handler))
        .route("/_/info", get(info_handler))
}
