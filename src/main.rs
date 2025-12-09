mod check;
mod env_parser;
mod handlers;
mod metrics;
mod models;
mod routes;
mod telemetry;

use metrics::create_metric_layer;
use opentelemetry::trace::TracerProvider;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use routes::create_router;
use std::env;
use telemetry::{init_telemetry, ExporterType, OtelConfig, OtelProviders};
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

    // Initialize OpenTelemetry if not disabled and any exporter is enabled
    let _otel_providers = init_otel_and_tracing(&otel_config);

    info!("Starting pmp-test-api");
    log_otel_status(&otel_config);

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

fn init_otel_and_tracing(config: &OtelConfig) -> Option<OtelProviders> {
    if config.disabled || !config.is_any_enabled() {
        init_standard_tracing();
        return None;
    }

    match init_telemetry(config) {
        Ok(providers) => {
            init_tracing_with_otel(config.service_name.clone(), &providers);
            Some(providers)
        }
        Err(e) => {
            eprintln!("Failed to initialize OpenTelemetry: {}", e);
            init_standard_tracing();
            None
        }
    }
}

fn init_tracing_with_otel(service_name: String, providers: &OtelProviders) {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "info,pmp_test_api=debug".into());

    let fmt_layer = tracing_subscriber::fmt::layer();
    let registry = tracing_subscriber::registry().with(env_filter).with(fmt_layer);

    match (&providers.tracer_provider, &providers.logger_provider) {
        (Some(tracer_provider), Some(logger_provider)) => {
            let tracer = tracer_provider.tracer(service_name);
            let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
            let logs_layer = OpenTelemetryTracingBridge::new(logger_provider);
            registry.with(telemetry_layer).with(logs_layer).init();
        }
        (Some(tracer_provider), None) => {
            let tracer = tracer_provider.tracer(service_name);
            let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);
            registry.with(telemetry_layer).init();
        }
        (None, Some(logger_provider)) => {
            let logs_layer = OpenTelemetryTracingBridge::new(logger_provider);
            registry.with(logs_layer).init();
        }
        (None, None) => {
            registry.init();
        }
    }
}

fn init_standard_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,pmp_test_api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn log_otel_status(config: &OtelConfig) {
    if config.disabled {
        info!("OpenTelemetry SDK disabled");
        return;
    }

    if !config.is_any_enabled() {
        info!("OpenTelemetry: no exporters enabled");
        return;
    }

    let traces = format_exporter_status(&config.traces_exporter);
    let metrics = format_exporter_status(&config.metrics_exporter);
    let logs = format_exporter_status(&config.logs_exporter);

    info!(
        endpoint = %config.endpoint,
        protocol = ?config.protocol,
        traces = %traces,
        metrics = %metrics,
        logs = %logs,
        "OpenTelemetry enabled"
    );
}

fn format_exporter_status(exporter: &ExporterType) -> &'static str {
    match exporter {
        ExporterType::Otlp => "otlp",
        ExporterType::Console => "console",
        ExporterType::None => "none",
    }
}
