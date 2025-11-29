mod check;
mod env_parser;
mod handlers;
mod metrics;
mod models;
mod routes;
mod telemetry;

use metrics::create_metric_layer;
use opentelemetry::trace::TracerProvider;
use routes::create_router;
use std::env;
use telemetry::{init_tracer, OtelConfig};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if it exists (useful for local development)
    let _ = dotenvy::dotenv();

    // Load OpenTelemetry configuration
    let otel_config = OtelConfig::from_env();

    // Initialize OpenTelemetry if enabled
    let _tracer_provider = if otel_config.enabled {
        match init_tracer(&otel_config) {
            Ok(provider) => {
                // Set up tracing subscriber with OpenTelemetry layer
                let tracer = provider.tracer("pmp-test-api");
                let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

                tracing_subscriber::registry()
                    .with(
                        tracing_subscriber::EnvFilter::try_from_default_env()
                            .unwrap_or_else(|_| "info,pmp_test_api=debug".into()),
                    )
                    .with(tracing_subscriber::fmt::layer())
                    .with(telemetry_layer)
                    .init();

                Some(provider)
            }
            Err(e) => {
                eprintln!("Failed to initialize OpenTelemetry: {}", e);
                // Fall back to standard tracing
                tracing_subscriber::registry()
                    .with(
                        tracing_subscriber::EnvFilter::try_from_default_env()
                            .unwrap_or_else(|_| "info,pmp_test_api=debug".into()),
                    )
                    .with(tracing_subscriber::fmt::layer())
                    .init();
                None
            }
        }
    } else {
        // Standard tracing without OpenTelemetry
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info,pmp_test_api=debug".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
        None
    };

    info!("Starting pmp-test-api");

    if otel_config.enabled {
        info!(
            otel_endpoint = %otel_config.endpoint,
            otel_protocol = ?otel_config.protocol,
            "OpenTelemetry enabled"
        );
    }

    // Get port from environment or use default
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    let addr = format!("0.0.0.0:{}", port);

    info!("Binding to {}", addr);

    // Create prometheus metrics layer and state
    let (metric_layer, metrics_state) = create_metric_layer();

    // Create router with metrics and tracing layers
    let app = create_router(metrics_state)
        .layer(metric_layer)
        .layer(TraceLayer::new_for_http());

    // Create TCP listener
    let listener = TcpListener::bind(&addr).await?;

    info!("Server listening on {}", addr);
    info!("Dashboard: http://{}/", addr);
    info!("Health endpoint: http://{}/_/health", addr);
    info!("Info endpoint: http://{}/_/info", addr);
    info!("Metrics endpoint: http://{}/metrics", addr);

    // Start the server
    axum::serve(listener, app).await?;

    Ok(())
}
