use crate::handlers::{execute_http_request, health_handler, info_handler, ui_handler};
use crate::metrics::{metrics_handler, MetricsState};
use axum::{
    Router,
    routing::{get, post},
};

/// Configure and return the application router
pub fn create_router(metrics_state: MetricsState) -> Router {
    Router::new()
        .route("/", get(ui_handler))
        .route("/api/http-client", post(execute_http_request))
        .route("/_/health", get(health_handler))
        .route("/_/info", get(info_handler))
        .route("/metrics", get(metrics_handler))
        .with_state(metrics_state)
}
