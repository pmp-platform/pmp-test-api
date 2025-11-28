use crate::check::{
    check_bedrock, check_dynamodb, check_http, check_memorydb, check_nosql, check_s3,
    check_secrets_manager, check_sql,
};
use crate::env_parser::{
    get_all_env_vars, parse_bedrock_configs, parse_dynamodb_configs, parse_http_configs,
    parse_memorydb_configs, parse_nosql_configs, parse_s3_configs, parse_secrets_manager_configs,
    parse_sql_configs,
};
use crate::models::InfoResponse;
use axum::Json;
use std::collections::HashMap;
use tracing::{info, instrument};

/// Info endpoint handler
/// Returns comprehensive information about the system and all configured checks
#[instrument]
pub async fn info_handler() -> Json<InfoResponse> {
    info!(
        event = "info_request_started",
        "Processing info request"
    );

    // Get all environment variables
    let environments = get_all_env_vars();

    // Parse configurations from environment
    let sql_configs = parse_sql_configs();
    let nosql_configs = parse_nosql_configs();
    let http_configs = parse_http_configs();
    let s3_configs = parse_s3_configs();
    let memorydb_configs = parse_memorydb_configs();
    let secrets_manager_configs = parse_secrets_manager_configs();
    let dynamodb_configs = parse_dynamodb_configs();
    let bedrock_configs = parse_bedrock_configs();

    // Run all SQL checks concurrently
    let sql_results = if !sql_configs.is_empty() {
        let mut tasks = Vec::new();

        for (identifier, config) in sql_configs {
            tasks.push(async move {
                let result = check_sql(config).await;
                (identifier, result)
            });
        }

        let results = futures::future::join_all(tasks).await;
        Some(results.into_iter().collect::<HashMap<_, _>>())
    } else {
        None
    };

    // Run all NoSQL checks concurrently
    let nosql_results = if !nosql_configs.is_empty() {
        let mut tasks = Vec::new();

        for (identifier, config) in nosql_configs {
            tasks.push(async move {
                let result = check_nosql(config).await;
                (identifier, result)
            });
        }

        let results = futures::future::join_all(tasks).await;
        Some(results.into_iter().collect::<HashMap<_, _>>())
    } else {
        None
    };

    // Run all HTTP checks concurrently
    let http_results = if !http_configs.is_empty() {
        let mut tasks = Vec::new();

        for (identifier, config) in http_configs {
            tasks.push(async move {
                let result = check_http(config).await;
                (identifier, result)
            });
        }

        let results = futures::future::join_all(tasks).await;
        Some(results.into_iter().collect::<HashMap<_, _>>())
    } else {
        None
    };

    // Run all S3 checks concurrently
    let s3_results = if !s3_configs.is_empty() {
        let mut tasks = Vec::new();

        for (identifier, config) in s3_configs {
            tasks.push(async move {
                let result = check_s3(config).await;
                (identifier, result)
            });
        }

        let results = futures::future::join_all(tasks).await;
        Some(results.into_iter().collect::<HashMap<_, _>>())
    } else {
        None
    };

    // Run all MemoryDB checks concurrently
    let memorydb_results = if !memorydb_configs.is_empty() {
        let mut tasks = Vec::new();

        for (identifier, config) in memorydb_configs {
            tasks.push(async move {
                let result = check_memorydb(config).await;
                (identifier, result)
            });
        }

        let results = futures::future::join_all(tasks).await;
        Some(results.into_iter().collect::<HashMap<_, _>>())
    } else {
        None
    };

    // Run all Secrets Manager checks concurrently
    let secrets_manager_results = if !secrets_manager_configs.is_empty() {
        let mut tasks = Vec::new();

        for (identifier, config) in secrets_manager_configs {
            tasks.push(async move {
                let result = check_secrets_manager(config).await;
                (identifier, result)
            });
        }

        let results = futures::future::join_all(tasks).await;
        Some(results.into_iter().collect::<HashMap<_, _>>())
    } else {
        None
    };

    // Run all DynamoDB checks concurrently
    let dynamodb_results = if !dynamodb_configs.is_empty() {
        let mut tasks = Vec::new();

        for (identifier, config) in dynamodb_configs {
            tasks.push(async move {
                let result = check_dynamodb(config).await;
                (identifier, result)
            });
        }

        let results = futures::future::join_all(tasks).await;
        Some(results.into_iter().collect::<HashMap<_, _>>())
    } else {
        None
    };

    // Run all Bedrock checks concurrently
    let bedrock_results = if !bedrock_configs.is_empty() {
        let mut tasks = Vec::new();

        for (identifier, config) in bedrock_configs {
            tasks.push(async move {
                let result = check_bedrock(config).await;
                (identifier, result)
            });
        }

        let results = futures::future::join_all(tasks).await;
        Some(results.into_iter().collect::<HashMap<_, _>>())
    } else {
        None
    };

    // Count checks performed
    let sql_count = sql_results.as_ref().map(|r| r.len()).unwrap_or(0);
    let nosql_count = nosql_results.as_ref().map(|r| r.len()).unwrap_or(0);
    let http_count = http_results.as_ref().map(|r| r.len()).unwrap_or(0);
    let s3_count = s3_results.as_ref().map(|r| r.len()).unwrap_or(0);
    let memorydb_count = memorydb_results.as_ref().map(|r| r.len()).unwrap_or(0);
    let secrets_manager_count = secrets_manager_results.as_ref().map(|r| r.len()).unwrap_or(0);
    let dynamodb_count = dynamodb_results.as_ref().map(|r| r.len()).unwrap_or(0);
    let bedrock_count = bedrock_results.as_ref().map(|r| r.len()).unwrap_or(0);
    let total_checks = sql_count + nosql_count + http_count + s3_count + memorydb_count
        + secrets_manager_count + dynamodb_count + bedrock_count;

    info!(
        event = "info_request_completed",
        sql_checks = sql_count,
        nosql_checks = nosql_count,
        http_checks = http_count,
        s3_checks = s3_count,
        memorydb_checks = memorydb_count,
        secrets_manager_checks = secrets_manager_count,
        dynamodb_checks = dynamodb_count,
        bedrock_checks = bedrock_count,
        total_checks = total_checks,
        "Info request completed successfully"
    );

    Json(InfoResponse {
        environments,
        sql: sql_results,
        nosql: nosql_results,
        http: http_results,
        s3: s3_results,
        memorydb: memorydb_results,
        secrets_manager: secrets_manager_results,
        dynamodb: dynamodb_results,
        bedrock: bedrock_results,
    })
}
