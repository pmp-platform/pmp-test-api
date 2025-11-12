use crate::models::{BedrockCheckResult, BedrockConfig};
use aws_config::BehaviorVersion;
use aws_sdk_bedrock::Client;
use aws_sdk_bedrock::config::Region;
use tracing::{debug, error, info, instrument};

/// Check AWS Bedrock availability and list foundation models
#[instrument(skip(config), fields(identifier = %config.identifier))]
pub async fn check_bedrock(config: BedrockConfig) -> BedrockCheckResult {
    info!("Checking Bedrock availability: {}", config.identifier);

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
            aws_config_builder.credentials_provider(aws_sdk_bedrock::config::Credentials::new(
                access_key_id,
                secret_access_key,
                None,
                None,
                "env",
            ));
    }

    let aws_config = aws_config_builder.load().await;
    let client = Client::new(&aws_config);

    debug!("Attempting to list foundation models");

    // Try to list foundation models
    match client.list_foundation_models().send().await {
        Ok(output) => {
            let models: Vec<String> = output
                .model_summaries()
                .iter()
                .map(|model| model.model_id().to_string())
                .collect();

            let model_count = models.len();
            info!(
                "Successfully listed {} Bedrock foundation models",
                model_count
            );

            BedrockCheckResult {
                success: true,
                region: config.region,
                model_count: Some(model_count),
                models: Some(models),
                error: None,
            }
        }
        Err(e) => {
            error!("Failed to list Bedrock foundation models: {}", e);
            BedrockCheckResult {
                success: false,
                region: config.region,
                model_count: None,
                models: None,
                error: Some(format!("Failed to list foundation models: {}", e)),
            }
        }
    }
}
