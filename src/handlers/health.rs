use axum::http::StatusCode;

/// Health check endpoint handler
/// Returns 200 OK to indicate the service is running
pub async fn health_handler() -> StatusCode {
    StatusCode::OK
}
