use crate::models::{SecretsManagerCheckResult, SecretsManagerConfig};
use aws_config::BehaviorVersion;
use aws_sdk_secretsmanager::Client;
use aws_sdk_secretsmanager::config::Region;
use tracing::{debug, error, info, instrument};

/// Check AWS Secrets Manager secret and retrieve information
#[instrument(skip(config), fields(identifier = %config.identifier, secret_name = %config.secret_name))]
pub async fn check_secrets_manager(config: SecretsManagerConfig) -> SecretsManagerCheckResult {
    info!("Checking Secrets Manager secret: {}", config.identifier);

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
        aws_config_builder = aws_config_builder.credentials_provider(
            aws_sdk_secretsmanager::config::Credentials::new(
                access_key_id,
                secret_access_key,
                None,
                None,
                "env",
            ),
        );
    }

    let aws_config = aws_config_builder.load().await;
    let client = Client::new(&aws_config);

    debug!("Attempting to describe secret");

    // Try to describe the secret
    match client
        .describe_secret()
        .secret_id(&config.secret_name)
        .send()
        .await
    {
        Ok(output) => {
            let version_id = output
                .version_ids_to_stages()
                .and_then(|versions| versions.keys().next().map(|s| s.to_string()));

            info!("Successfully checked Secrets Manager secret");

            SecretsManagerCheckResult {
                success: true,
                region: config.region,
                secret_name: config.secret_name,
                exists: Some(true),
                version_id,
                error: None,
            }
        }
        Err(e) => {
            error!("Failed to describe secret: {}", e);
            SecretsManagerCheckResult {
                success: false,
                region: config.region,
                secret_name: config.secret_name,
                exists: Some(false),
                version_id: None,
                error: Some(format!("Failed to describe secret: {}", e)),
            }
        }
    }
}
