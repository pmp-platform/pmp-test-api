use crate::models::{HttpCheckResult, HttpConfig};
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info, instrument};

/// Check an HTTP API endpoint
#[instrument(skip(config), fields(identifier = %config.identifier, url = %config.url, method = %config.method))]
pub async fn check_http(config: HttpConfig) -> HttpCheckResult {
    info!("Checking HTTP API: {}", config.identifier);

    // Create HTTP client with timeout
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap_or_else(|_| Client::new());

    debug!("Making {} request to {}", config.method, config.url);

    // Build the request based on method
    let request_builder = match config.method.as_str() {
        "GET" => client.get(&config.url),
        "POST" => client.post(&config.url),
        "PUT" => client.put(&config.url),
        "DELETE" => client.delete(&config.url),
        "PATCH" => client.patch(&config.url),
        "HEAD" => client.head(&config.url),
        method => {
            error!("Unsupported HTTP method: {}", method);
            return HttpCheckResult {
                success: false,
                url: config.url,
                method: method.to_string(),
                status_code: None,
                response_headers: None,
                response_body: None,
                error: Some(format!("Unsupported HTTP method: {}", method)),
            };
        }
    };

    // Add custom headers
    let mut request_builder = request_builder;
    for (key, value) in &config.headers {
        request_builder = request_builder.header(key, value);
    }

    // Send the request
    let response_result = request_builder.send().await;

    match response_result {
        Ok(response) => {
            let status_code = response.status().as_u16();
            debug!("Received response with status code: {}", status_code);

            // Extract response headers
            let mut response_headers = HashMap::new();
            for (key, value) in response.headers() {
                if let Ok(value_str) = value.to_str() {
                    response_headers.insert(key.to_string(), value_str.to_string());
                }
            }

            // Get response body
            let body_result = response.text().await;

            match body_result {
                Ok(body) => {
                    info!("Successfully completed HTTP request");

                    HttpCheckResult {
                        success: true,
                        url: config.url,
                        method: config.method,
                        status_code: Some(status_code),
                        response_headers: Some(response_headers),
                        response_body: Some(body),
                        error: None,
                    }
                }
                Err(e) => {
                    error!("Failed to read response body: {}", e);
                    HttpCheckResult {
                        success: false,
                        url: config.url,
                        method: config.method,
                        status_code: Some(status_code),
                        response_headers: Some(response_headers),
                        response_body: None,
                        error: Some(format!("Failed to read response body: {}", e)),
                    }
                }
            }
        }
        Err(e) => {
            error!("HTTP request failed: {}", e);
            HttpCheckResult {
                success: false,
                url: config.url,
                method: config.method,
                status_code: None,
                response_headers: None,
                response_body: None,
                error: Some(format!("Request failed: {}", e)),
            }
        }
    }
}
