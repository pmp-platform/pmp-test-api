use crate::models::{HttpClientRequest, HttpClientResponse};
use axum::Json;
use std::collections::HashMap;

/// API endpoint to execute HTTP requests from the UI
pub async fn execute_http_request(
    Json(request): Json<HttpClientRequest>,
) -> Json<HttpClientResponse> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(false)
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .unwrap();

    let method = match request.method.to_uppercase().as_str() {
        "GET" => reqwest::Method::GET,
        "POST" => reqwest::Method::POST,
        "PUT" => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        "PATCH" => reqwest::Method::PATCH,
        "OPTIONS" => reqwest::Method::OPTIONS,
        _ => reqwest::Method::GET,
    };

    let mut req_builder = client.request(method, &request.url);

    // Add headers
    for (key, value) in &request.headers {
        req_builder = req_builder.header(key, value);
    }

    // Add body if present
    if let Some(body) = &request.body
        && !body.is_empty()
    {
        req_builder = req_builder.body(body.clone());
    }

    match req_builder.send().await {
        Ok(response) => {
            let status_code = response.status().as_u16();
            let mut headers = HashMap::new();

            // Extract headers
            for (key, value) in response.headers() {
                if let Ok(value_str) = value.to_str() {
                    headers.insert(key.to_string(), value_str.to_string());
                }
            }

            // Get response body
            let body = (response.text().await).ok();

            Json(HttpClientResponse {
                success: true,
                status_code: Some(status_code),
                headers,
                body,
                error: None,
            })
        }
        Err(err) => Json(HttpClientResponse {
            success: false,
            status_code: None,
            headers: HashMap::new(),
            body: None,
            error: Some(err.to_string()),
        }),
    }
}
