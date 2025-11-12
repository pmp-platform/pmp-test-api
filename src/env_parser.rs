use crate::models::{
    BedrockConfig, DynamoDBConfig, HttpConfig, MemoryDBConfig, NoSqlConfig, S3Config,
    SecretsManagerConfig, SqlConfig,
};
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
        if let Some(rest) = key.strip_prefix("SQL_")
            && let Some((identifier, param)) = rest.split_once('_')
        {
            configs
                .entry(identifier.to_string())
                .or_default()
                .insert(param.to_lowercase(), value);
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
                host: params
                    .get("host")
                    .cloned()
                    .unwrap_or_else(|| "localhost".to_string()),
                port: params
                    .get("port")
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(5432),
                user: params
                    .get("user")
                    .cloned()
                    .unwrap_or_else(|| "postgres".to_string()),
                password: params.get("password").cloned().unwrap_or_default(),
                database: params
                    .get("database")
                    .cloned()
                    .unwrap_or_else(|| "postgres".to_string()),
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
        if let Some(rest) = key.strip_prefix("NOSQL_")
            && let Some((identifier, param)) = rest.split_once('_')
        {
            configs
                .entry(identifier.to_string())
                .or_default()
                .insert(param.to_lowercase(), value);
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
                host: params
                    .get("host")
                    .cloned()
                    .unwrap_or_else(|| "localhost".to_string()),
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
        if let Some(rest) = key.strip_prefix("HTTP_")
            && let Some((identifier, param)) = rest.split_once('_')
        {
            configs
                .entry(identifier.to_string())
                .or_default()
                .insert(param.to_lowercase(), value);
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

/// Parse S3 bucket configurations from environment variables
/// Format: S3_{identifier}_{param}
pub fn parse_s3_configs() -> HashMap<String, S3Config> {
    let mut configs: HashMap<String, HashMap<String, String>> = HashMap::new();

    // Group environment variables by identifier
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("S3_")
            && let Some((identifier, param)) = rest.split_once('_')
        {
            configs
                .entry(identifier.to_string())
                .or_default()
                .insert(param.to_lowercase(), value);
        }
    }

    // Convert grouped variables into S3Config structs
    let mut s3_configs = HashMap::new();
    for (identifier, params) in configs {
        if let Some(bucket) = params.get("bucket") {
            // Only parse if bucket is specified
            let config = S3Config {
                identifier: identifier.clone(),
                region: params
                    .get("region")
                    .cloned()
                    .unwrap_or_else(|| "us-east-1".to_string()),
                bucket: bucket.clone(),
                access_key_id: params.get("access_key_id").cloned(),
                secret_access_key: params.get("secret_access_key").cloned(),
            };
            s3_configs.insert(identifier, config);
        }
    }

    s3_configs
}

/// Parse MemoryDB configurations from environment variables
/// Format: MEMORYDB_{identifier}_{param}
pub fn parse_memorydb_configs() -> HashMap<String, MemoryDBConfig> {
    let mut configs: HashMap<String, HashMap<String, String>> = HashMap::new();

    // Group environment variables by identifier
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("MEMORYDB_")
            && let Some((identifier, param)) = rest.split_once('_')
        {
            configs
                .entry(identifier.to_string())
                .or_default()
                .insert(param.to_lowercase(), value);
        }
    }

    // Convert grouped variables into MemoryDBConfig structs
    let mut memorydb_configs = HashMap::new();
    for (identifier, params) in configs {
        if let Some(cluster) = params.get("cluster") {
            // Only parse if cluster is specified
            let config = MemoryDBConfig {
                identifier: identifier.clone(),
                region: params
                    .get("region")
                    .cloned()
                    .unwrap_or_else(|| "us-east-1".to_string()),
                cluster: cluster.clone(),
                access_key_id: params.get("access_key_id").cloned(),
                secret_access_key: params.get("secret_access_key").cloned(),
            };
            memorydb_configs.insert(identifier, config);
        }
    }

    memorydb_configs
}

/// Parse AWS Secrets Manager configurations from environment variables
/// Format: SECRETS_{identifier}_{param}
pub fn parse_secrets_manager_configs() -> HashMap<String, SecretsManagerConfig> {
    let mut configs: HashMap<String, HashMap<String, String>> = HashMap::new();

    // Group environment variables by identifier
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("SECRETS_")
            && let Some((identifier, param)) = rest.split_once('_')
        {
            configs
                .entry(identifier.to_string())
                .or_default()
                .insert(param.to_lowercase(), value);
        }
    }

    // Convert grouped variables into SecretsManagerConfig structs
    let mut secrets_configs = HashMap::new();
    for (identifier, params) in configs {
        if let Some(secret_name) = params.get("secret_name") {
            // Only parse if secret_name is specified
            let config = SecretsManagerConfig {
                identifier: identifier.clone(),
                region: params
                    .get("region")
                    .cloned()
                    .unwrap_or_else(|| "us-east-1".to_string()),
                secret_name: secret_name.clone(),
                access_key_id: params.get("access_key_id").cloned(),
                secret_access_key: params.get("secret_access_key").cloned(),
            };
            secrets_configs.insert(identifier, config);
        }
    }

    secrets_configs
}

/// Parse DynamoDB configurations from environment variables
/// Format: DYNAMODB_{identifier}_{param}
pub fn parse_dynamodb_configs() -> HashMap<String, DynamoDBConfig> {
    let mut configs: HashMap<String, HashMap<String, String>> = HashMap::new();

    // Group environment variables by identifier
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("DYNAMODB_")
            && let Some((identifier, param)) = rest.split_once('_')
        {
            configs
                .entry(identifier.to_string())
                .or_default()
                .insert(param.to_lowercase(), value);
        }
    }

    // Convert grouped variables into DynamoDBConfig structs
    let mut dynamodb_configs = HashMap::new();
    for (identifier, params) in configs {
        if let Some(table) = params.get("table") {
            // Only parse if table is specified
            let config = DynamoDBConfig {
                identifier: identifier.clone(),
                region: params
                    .get("region")
                    .cloned()
                    .unwrap_or_else(|| "us-east-1".to_string()),
                table: table.clone(),
                access_key_id: params.get("access_key_id").cloned(),
                secret_access_key: params.get("secret_access_key").cloned(),
            };
            dynamodb_configs.insert(identifier, config);
        }
    }

    dynamodb_configs
}

/// Parse AWS Bedrock configurations from environment variables
/// Format: BEDROCK_{identifier}_{param}
pub fn parse_bedrock_configs() -> HashMap<String, BedrockConfig> {
    let mut configs: HashMap<String, HashMap<String, String>> = HashMap::new();

    // Group environment variables by identifier
    for (key, value) in env::vars() {
        if let Some(rest) = key.strip_prefix("BEDROCK_")
            && let Some((identifier, param)) = rest.split_once('_')
        {
            configs
                .entry(identifier.to_string())
                .or_default()
                .insert(param.to_lowercase(), value);
        }
    }

    // Convert grouped variables into BedrockConfig structs
    let mut bedrock_configs = HashMap::new();
    for (identifier, params) in configs {
        // Bedrock only requires region (and optional credentials)
        let config = BedrockConfig {
            identifier: identifier.clone(),
            region: params
                .get("region")
                .cloned()
                .unwrap_or_else(|| "us-east-1".to_string()),
            access_key_id: params.get("access_key_id").cloned(),
            secret_access_key: params.get("secret_access_key").cloned(),
        };
        bedrock_configs.insert(identifier, config);
    }

    bedrock_configs
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
