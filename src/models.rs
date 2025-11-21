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

    /// S3 bucket check results, keyed by identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<HashMap<String, S3CheckResult>>,

    /// MemoryDB check results, keyed by identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memorydb: Option<HashMap<String, MemoryDBCheckResult>>,

    /// AWS Secrets Manager check results, keyed by identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager: Option<HashMap<String, SecretsManagerCheckResult>>,

    /// DynamoDB check results, keyed by identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb: Option<HashMap<String, DynamoDBCheckResult>>,

    /// Bedrock check results, keyed by identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrock: Option<HashMap<String, BedrockCheckResult>>,
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

/// Result of checking an S3 bucket
#[derive(Debug, Serialize, Deserialize)]
pub struct S3CheckResult {
    /// Whether the connection was successful
    pub success: bool,

    /// AWS region
    pub region: String,

    /// Bucket name
    pub bucket: String,

    /// Whether the bucket exists and is accessible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,

    /// Number of objects in the bucket (if accessible)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_count: Option<usize>,

    /// Error message if check failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Result of checking a MemoryDB connection
#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryDBCheckResult {
    /// Whether the connection was successful
    pub success: bool,

    /// AWS region
    pub region: String,

    /// Cluster name
    pub cluster: String,

    /// Cluster endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,

    /// Cluster status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Number of nodes in the cluster
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_count: Option<usize>,

    /// Error message if check failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Parsed S3 bucket configuration from environment variables
#[derive(Debug, Clone)]
pub struct S3Config {
    pub identifier: String,
    pub region: String,
    pub bucket: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

/// Parsed MemoryDB configuration from environment variables
#[derive(Debug, Clone)]
pub struct MemoryDBConfig {
    pub identifier: String,
    pub region: String,
    pub cluster: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

/// Result of checking AWS Secrets Manager
#[derive(Debug, Serialize, Deserialize)]
pub struct SecretsManagerCheckResult {
    /// Whether the connection was successful
    pub success: bool,

    /// AWS region
    pub region: String,

    /// Secret name
    pub secret_name: String,

    /// Whether the secret exists and is accessible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,

    /// Secret version ID (if accessible)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,

    /// Error message if check failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Result of checking a DynamoDB table
#[derive(Debug, Serialize, Deserialize)]
pub struct DynamoDBCheckResult {
    /// Whether the connection was successful
    pub success: bool,

    /// AWS region
    pub region: String,

    /// Table name
    pub table: String,

    /// Table status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// Item count (approximate)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,

    /// Table size in bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_size_bytes: Option<i64>,

    /// Error message if check failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Result of checking AWS Bedrock
#[derive(Debug, Serialize, Deserialize)]
pub struct BedrockCheckResult {
    /// Whether the connection was successful
    pub success: bool,

    /// AWS region
    pub region: String,

    /// Number of foundation models available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_count: Option<usize>,

    /// List of available model IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,

    /// Error message if check failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Parsed AWS Secrets Manager configuration from environment variables
#[derive(Debug, Clone)]
pub struct SecretsManagerConfig {
    pub identifier: String,
    pub region: String,
    pub secret_name: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

/// Parsed DynamoDB configuration from environment variables
#[derive(Debug, Clone)]
pub struct DynamoDBConfig {
    pub identifier: String,
    pub region: String,
    pub table: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

/// Parsed AWS Bedrock configuration from environment variables
#[derive(Debug, Clone)]
pub struct BedrockConfig {
    pub identifier: String,
    pub region: String,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
}

/// Request model for HTTP client UI
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpClientRequest {
    /// The URL to request
    pub url: String,

    /// HTTP method (GET, POST, PUT, DELETE, PATCH, OPTIONS)
    pub method: String,

    /// Request headers
    #[serde(default)]
    pub headers: HashMap<String, String>,

    /// Optional request body
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

/// Response model for HTTP client UI
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpClientResponse {
    /// Whether the request was successful
    pub success: bool,

    /// HTTP status code received
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<u16>,

    /// Response headers
    pub headers: HashMap<String, String>,

    /// Response body (as string)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,

    /// Error message if request failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
