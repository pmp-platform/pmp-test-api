use crate::check::{check_http, check_nosql, check_sql};
use crate::env_parser::{get_all_env_vars, parse_http_configs, parse_nosql_configs, parse_sql_configs};
use crate::models::InfoResponse;
use axum::Json;
use std::collections::HashMap;
use tracing::{info, instrument};

/// Info endpoint handler
/// Returns comprehensive information about the system and all configured checks
#[instrument]
pub async fn info_handler() -> Json<InfoResponse> {
    info!("Processing info request");

    // Get all environment variables
    let environments = get_all_env_vars();

    // Parse configurations from environment
    let sql_configs = parse_sql_configs();
    let nosql_configs = parse_nosql_configs();
    let http_configs = parse_http_configs();

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

    info!("Info request completed");

    Json(InfoResponse {
        environments,
        sql: sql_results,
        nosql: nosql_results,
        http: http_results,
    })
}
