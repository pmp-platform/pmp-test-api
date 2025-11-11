use crate::models::{S3CheckResult, S3Config};
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Region;
use tracing::{debug, error, info, instrument};

/// Check an S3 bucket and retrieve information
#[instrument(skip(config), fields(identifier = %config.identifier, bucket = %config.bucket))]
pub async fn check_s3(config: S3Config) -> S3CheckResult {
    info!("Checking S3 bucket: {}", config.identifier);

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
            aws_config_builder.credentials_provider(aws_sdk_s3::config::Credentials::new(
                access_key_id,
                secret_access_key,
                None,
                None,
                "env",
            ));
    }

    let aws_config = aws_config_builder.load().await;
    let client = Client::new(&aws_config);

    debug!("Attempting to check S3 bucket existence");

    // Try to check if bucket exists by calling head_bucket
    match client.head_bucket().bucket(&config.bucket).send().await {
        Ok(_) => {
            debug!("Successfully accessed S3 bucket");

            // Try to list objects to get count
            match client
                .list_objects_v2()
                .bucket(&config.bucket)
                .max_keys(1000)
                .send()
                .await
            {
                Ok(output) => {
                    let object_count = output.key_count().unwrap_or(0) as usize;
                    info!(
                        "Successfully checked S3 bucket with {} objects",
                        object_count
                    );

                    S3CheckResult {
                        success: true,
                        region: config.region,
                        bucket: config.bucket,
                        exists: Some(true),
                        object_count: Some(object_count),
                        error: None,
                    }
                }
                Err(e) => {
                    error!("Failed to list objects: {}", e);
                    S3CheckResult {
                        success: false,
                        region: config.region,
                        bucket: config.bucket,
                        exists: Some(true),
                        object_count: None,
                        error: Some(format!("Failed to list objects: {}", e)),
                    }
                }
            }
        }
        Err(e) => {
            error!("Failed to access S3 bucket: {}", e);
            S3CheckResult {
                success: false,
                region: config.region,
                bucket: config.bucket,
                exists: Some(false),
                object_count: None,
                error: Some(format!("Bucket access failed: {}", e)),
            }
        }
    }
}
