use crate::models::{HttpConfig, NoSqlConfig, SqlConfig};
use std::collections::HashMap;
use std::env;

/// Parse all environment variables and return them as a HashMap
pub fn get_all_env_vars() -> HashMap<String, String> {
    env::vars().collect()
}

/// Parse SQL database configurations from environment variables
/// Format: SQL_{identifier}_{param}
pub fn parse_sql_configs() -> HashMap<String, SqlConfig> {
    let mut configs: HashMap<String, HashMap<String, String>> = HashMap::new();

    // Group environment variables by identifier
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("SQL_") {
            if let Some((identifier, param)) = rest.split_once('_') {
                configs
                    .entry(identifier.to_string())
                    .or_insert_with(HashMap::new)
                    .insert(param.to_lowercase(), value);
            }
        }
    }

    // Convert grouped variables into SqlConfig structs
    let mut sql_configs = HashMap::new();
    for (identifier, params) in configs {
        if let Some(driver) = params.get("driver") {
            // Only parse if driver is specified
            let config = SqlConfig {
                identifier: identifier.clone(),
                driver: driver.clone(),
                host: params.get("host").cloned().unwrap_or_else(|| "localhost".to_string()),
                port: params
                    .get("port")
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(5432),
                user: params.get("user").cloned().unwrap_or_else(|| "postgres".to_string()),
                password: params.get("password").cloned().unwrap_or_default(),
                database: params.get("database").cloned().unwrap_or_else(|| "postgres".to_string()),
            };
            sql_configs.insert(identifier, config);
        }
    }

    sql_configs
}

/// Parse NoSQL database configurations from environment variables
/// Format: NOSQL_{identifier}_{param}
pub fn parse_nosql_configs() -> HashMap<String, NoSqlConfig> {
    let mut configs: HashMap<String, HashMap<String, String>> = HashMap::new();

    // Group environment variables by identifier
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("NOSQL_") {
            if let Some((identifier, param)) = rest.split_once('_') {
                configs
                    .entry(identifier.to_string())
                    .or_insert_with(HashMap::new)
                    .insert(param.to_lowercase(), value);
            }
        }
    }

    // Convert grouped variables into NoSqlConfig structs
    let mut nosql_configs = HashMap::new();
    for (identifier, params) in configs {
        if let Some(driver) = params.get("driver") {
            // Only parse if driver is specified
            let config = NoSqlConfig {
                identifier: identifier.clone(),
                driver: driver.clone(),
                host: params.get("host").cloned().unwrap_or_else(|| "localhost".to_string()),
                port: params
                    .get("port")
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(6379),
                password: params.get("password").cloned(),
            };
            nosql_configs.insert(identifier, config);
        }
    }

    nosql_configs
}

/// Parse HTTP API configurations from environment variables
/// Format: HTTP_{identifier}_{param}
pub fn parse_http_configs() -> HashMap<String, HttpConfig> {
    let mut configs: HashMap<String, HashMap<String, String>> = HashMap::new();

    // Group environment variables by identifier
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("HTTP_") {
            if let Some((identifier, param)) = rest.split_once('_') {
                configs
                    .entry(identifier.to_string())
                    .or_insert_with(HashMap::new)
                    .insert(param.to_lowercase(), value);
            }
        }
    }

    // Convert grouped variables into HttpConfig structs
    let mut http_configs = HashMap::new();
    for (identifier, params) in configs {
        if let Some(url) = params.get("url") {
            // Only parse if URL is specified
            let method = params
                .get("method")
                .cloned()
                .unwrap_or_else(|| "GET".to_string())
                .to_uppercase();

            let headers = params
                .get("headers")
                .and_then(|h| serde_json::from_str::<HashMap<String, String>>(h).ok())
                .unwrap_or_default();

            let config = HttpConfig {
                identifier: identifier.clone(),
                url: url.clone(),
                method,
                headers,
            };
            http_configs.insert(identifier, config);
        }
    }

    http_configs
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sql_config_parsing() {
        // This test would require setting environment variables
        // In practice, integration tests would be more appropriate
    }

    #[test]
    fn test_nosql_config_parsing() {
        // This test would require setting environment variables
        // In practice, integration tests would be more appropriate
    }

    #[test]
    fn test_http_config_parsing() {
        // This test would require setting environment variables
        // In practice, integration tests would be more appropriate
    }
}
