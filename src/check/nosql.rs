use crate::models::{NoSqlCheckResult, NoSqlConfig};
use redis::aio::ConnectionManager;
use redis::{Client, RedisError};
use std::collections::HashMap;
use tracing::{debug, error, info, instrument};

/// Check a NoSQL database connection and retrieve information
#[instrument(skip(config), fields(identifier = %config.identifier, driver = %config.driver))]
pub async fn check_nosql(config: NoSqlConfig) -> NoSqlCheckResult {
    info!("Checking NoSQL database: {}", config.identifier);

    match config.driver.as_str() {
        "redis" => check_redis(config).await,
        driver => {
            error!("Unsupported NoSQL driver: {}", driver);
            NoSqlCheckResult {
                success: false,
                driver: driver.to_string(),
                host: config.host,
                port: config.port,
                info: None,
                error: Some(format!("Unsupported NoSQL driver: {}", driver)),
            }
        }
    }
}

/// Check a Redis database connection
#[instrument(skip(config))]
async fn check_redis(config: NoSqlConfig) -> NoSqlCheckResult {
    // Build Redis connection string
    let connection_string = if let Some(password) = &config.password {
        format!("redis://:{}@{}:{}/", password, config.host, config.port)
    } else {
        format!("redis://{}:{}/", config.host, config.port)
    };

    debug!("Attempting to connect to Redis");

    // Create Redis client
    let client_result = Client::open(connection_string);

    match client_result {
        Ok(client) => {
            // Try to get a connection manager
            let manager_result = ConnectionManager::new(client).await;

            match manager_result {
                Ok(mut manager) => {
                    debug!("Successfully connected to Redis");

                    // Try to ping the server
                    let ping_result: Result<String, RedisError> =
                        redis::cmd("PING").query_async(&mut manager).await;

                    match ping_result {
                        Ok(pong) => {
                            debug!("Redis PING successful: {}", pong);

                            // Try to get server info
                            let info_result: Result<String, RedisError> =
                                redis::cmd("INFO").query_async(&mut manager).await;

                            let info_map = match info_result {
                                Ok(info_str) => parse_redis_info(&info_str),
                                Err(e) => {
                                    error!("Failed to get Redis INFO: {}", e);
                                    HashMap::new()
                                }
                            };

                            info!("Successfully connected to Redis and retrieved info");

                            NoSqlCheckResult {
                                success: true,
                                driver: "redis".to_string(),
                                host: config.host,
                                port: config.port,
                                info: Some(info_map),
                                error: None,
                            }
                        }
                        Err(e) => {
                            error!("Redis PING failed: {}", e);
                            NoSqlCheckResult {
                                success: false,
                                driver: "redis".to_string(),
                                host: config.host,
                                port: config.port,
                                info: None,
                                error: Some(format!("PING failed: {}", e)),
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to create Redis connection manager: {}", e);
                    NoSqlCheckResult {
                        success: false,
                        driver: "redis".to_string(),
                        host: config.host,
                        port: config.port,
                        info: None,
                        error: Some(format!("Connection manager creation failed: {}", e)),
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to create Redis client: {}", e);
            NoSqlCheckResult {
                success: false,
                driver: "redis".to_string(),
                host: config.host,
                port: config.port,
                info: None,
                error: Some(format!("Client creation failed: {}", e)),
            }
        }
    }
}

/// Parse Redis INFO command output into a HashMap
fn parse_redis_info(info_str: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for line in info_str.lines() {
        let line = line.trim();

        // Skip empty lines and section headers
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse key:value pairs
        if let Some((key, value)) = line.split_once(':') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_redis_info() {
        let info = r#"# Server
redis_version:7.0.0
redis_mode:standalone

# Clients
connected_clients:1

# Memory
used_memory:1234567
"#;

        let result = parse_redis_info(info);

        assert_eq!(result.get("redis_version"), Some(&"7.0.0".to_string()));
        assert_eq!(result.get("redis_mode"), Some(&"standalone".to_string()));
        assert_eq!(result.get("connected_clients"), Some(&"1".to_string()));
        assert_eq!(result.get("used_memory"), Some(&"1234567".to_string()));
    }
}
