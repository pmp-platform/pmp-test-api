use crate::handlers::{execute_http_request, health_handler, info_handler, ui_handler};
use axum::{
    Router,
    routing::{get, post},
};

/// Configure and return the application router
pub fn create_router() -> Router {
    Router::new()
        .route("/", get(ui_handler))
        .route("/api/http-client", post(execute_http_request))
        .route("/_/health", get(health_handler))
        .route("/_/info", get(info_handler))
}
