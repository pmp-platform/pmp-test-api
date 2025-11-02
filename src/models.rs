use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main response structure for the /_/info endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    /// All environment variables found
    pub environments: HashMap<String, String>,

    /// SQL database check results, keyed by identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<HashMap<String, SqlCheckResult>>,

    /// NoSQL database check results, keyed by identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nosql: Option<HashMap<String, NoSqlCheckResult>>,

    /// HTTP API check results, keyed by identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http: Option<HashMap<String, HttpCheckResult>>,
}

/// Result of checking a SQL database connection
#[derive(Debug, Serialize, Deserialize)]
pub struct SqlCheckResult {
    /// Whether the connection was successful
    pub success: bool,

    /// The database driver used (e.g., "postgres")
    pub driver: String,

    /// Connection host
    pub host: String,

    /// Connection port
    pub port: u16,

    /// Database name
    pub database: String,

    /// List of tables found (only if connection successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<String>>,

    /// Error message if connection failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Result of checking a NoSQL database connection
#[derive(Debug, Serialize, Deserialize)]
pub struct NoSqlCheckResult {
    /// Whether the connection was successful
    pub success: bool,

    /// The database driver used (e.g., "redis")
    pub driver: String,

    /// Connection host
    pub host: String,

    /// Connection port
    pub port: u16,

    /// Server information from the database
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<HashMap<String, String>>,

    /// Error message if connection failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Result of checking an HTTP API
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpCheckResult {
    /// Whether the request was successful
    pub success: bool,

    /// The URL that was requested
    pub url: String,

    /// HTTP method used
    pub method: String,

    /// HTTP status code received
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<u16>,

    /// Response headers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<HashMap<String, String>>,

    /// Response body (as string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body: Option<String>,

    /// Error message if request failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Parsed SQL database configuration from environment variables
#[derive(Debug, Clone)]
pub struct SqlConfig {
    pub identifier: String,
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

/// Parsed NoSQL database configuration from environment variables
#[derive(Debug, Clone)]
pub struct NoSqlConfig {
    pub identifier: String,
    pub driver: String,
    pub host: String,
    pub port: u16,
    pub password: Option<String>,
}

/// Parsed HTTP API configuration from environment variables
#[derive(Debug, Clone)]
pub struct HttpConfig {
    pub identifier: String,
    pub url: String,
    pub method: String,
    pub headers: HashMap<String, String>,
}
