use crate::models::{MemoryDBCheckResult, MemoryDBConfig};
use aws_config::BehaviorVersion;
use aws_sdk_memorydb::config::Region;
use aws_sdk_memorydb::Client;
use tracing::{debug, error, info, instrument};

/// Check a MemoryDB cluster and retrieve information
#[instrument(skip(config), fields(identifier = %config.identifier, cluster = %config.cluster))]
pub async fn check_memorydb(config: MemoryDBConfig) -> MemoryDBCheckResult {
    info!("Checking MemoryDB cluster: {}", config.identifier);

    // Clone region for use in results
    let region_str = config.region.clone();

    // Set up AWS configuration
    let mut aws_config_builder = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region_str.clone()));

    // Use custom credentials if provided
    if let (Some(access_key_id), Some(secret_access_key)) =
        (&config.access_key_id, &config.secret_access_key) {
        debug!("Using custom AWS credentials");
        aws_config_builder = aws_config_builder.credentials_provider(
            aws_sdk_memorydb::config::Credentials::new(
                access_key_id,
                secret_access_key,
                None,
                None,
                "env",
            )
        );
    }

    let aws_config = aws_config_builder.load().await;
    let client = Client::new(&aws_config);

    debug!("Attempting to describe MemoryDB cluster");

    // Try to describe the cluster
    match client
        .describe_clusters()
        .cluster_name(&config.cluster)
        .send()
        .await
    {
        Ok(output) => {
            let clusters = output.clusters();
            if !clusters.is_empty() {
                if let Some(cluster) = clusters.first() {
                    let status = cluster.status().unwrap_or("unknown").to_string();
                    let endpoint = cluster
                        .cluster_endpoint()
                        .and_then(|e| e.address())
                        .map(|s| s.to_string());
                    let node_count = cluster.number_of_shards().unwrap_or(0) as usize;

                    info!(
                        "Successfully checked MemoryDB cluster with status: {}",
                        status
                    );

                    MemoryDBCheckResult {
                        success: true,
                        region: config.region,
                        cluster: config.cluster,
                        endpoint,
                        status: Some(status),
                        node_count: Some(node_count),
                        error: None,
                    }
                } else {
                    error!("Cluster not found in response");
                    MemoryDBCheckResult {
                        success: false,
                        region: config.region,
                        cluster: config.cluster,
                        endpoint: None,
                        status: None,
                        node_count: None,
                        error: Some("Cluster not found in response".to_string()),
                    }
                }
            } else {
                error!("No clusters returned in response");
                MemoryDBCheckResult {
                    success: false,
                    region: config.region,
                    cluster: config.cluster,
                    endpoint: None,
                    status: None,
                    node_count: None,
                    error: Some("No clusters returned in response".to_string()),
                }
            }
        }
        Err(e) => {
            error!("Failed to describe MemoryDB cluster: {}", e);
            MemoryDBCheckResult {
                success: false,
                region: config.region,
                cluster: config.cluster,
                endpoint: None,
                status: None,
                node_count: None,
                error: Some(format!("Failed to describe cluster: {}", e)),
            }
        }
    }
}
