use crate::models::{DynamoDBCheckResult, DynamoDBConfig};
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::config::Region;
use tracing::{debug, error, info, instrument};

/// Check a DynamoDB table and retrieve information
#[instrument(skip(config), fields(identifier = %config.identifier, table = %config.table))]
pub async fn check_dynamodb(config: DynamoDBConfig) -> DynamoDBCheckResult {
    info!("Checking DynamoDB table: {}", config.identifier);

    // Clone region for use in results
    let region_str = config.region.clone();

    // Set up AWS configuration
    let mut aws_config_builder =
        aws_config::defaults(BehaviorVersion::latest()).region(Region::new(region_str.clone()));

    // Use custom credentials if provided
    if let (Some(access_key_id), Some(secret_access_key)) =
        (&config.access_key_id, &config.secret_access_key)
    {
        debug!("Using custom AWS credentials");
        aws_config_builder =
            aws_config_builder.credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
                access_key_id,
                secret_access_key,
                None,
                None,
                "env",
            ));
    }

    let aws_config = aws_config_builder.load().await;
    let client = Client::new(&aws_config);

    debug!("Attempting to describe DynamoDB table");

    // Try to describe the table
    match client
        .describe_table()
        .table_name(&config.table)
        .send()
        .await
    {
        Ok(output) => {
            if let Some(table) = output.table() {
                let status = table.table_status().map(|s| s.as_str().to_string());
                let item_count = table.item_count();
                let table_size_bytes = table.table_size_bytes();

                info!(
                    "Successfully checked DynamoDB table with status: {:?}",
                    status
                );

                DynamoDBCheckResult {
                    success: true,
                    region: config.region,
                    table: config.table,
                    status,
                    item_count,
                    table_size_bytes,
                    error: None,
                }
            } else {
                error!("Table not found in response");
                DynamoDBCheckResult {
                    success: false,
                    region: config.region,
                    table: config.table,
                    status: None,
                    item_count: None,
                    table_size_bytes: None,
                    error: Some("Table not found in response".to_string()),
                }
            }
        }
        Err(e) => {
            error!("Failed to describe DynamoDB table: {}", e);
            DynamoDBCheckResult {
                success: false,
                region: config.region,
                table: config.table,
                status: None,
                item_count: None,
                table_size_bytes: None,
                error: Some(format!("Failed to describe table: {}", e)),
            }
        }
    }
}
