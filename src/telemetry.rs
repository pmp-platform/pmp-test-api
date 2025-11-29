use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace::TracerProvider, Resource};
use std::env;
use tracing::info;

/// OpenTelemetry protocol type
#[derive(Debug, Clone, PartialEq)]
pub enum OtelProtocol {
    Grpc,
    Http,
}

/// OpenTelemetry configuration from environment variables
#[derive(Debug, Clone)]
pub struct OtelConfig {
    /// Whether OpenTelemetry is enabled
    pub enabled: bool,
    /// Collector endpoint URL
    pub endpoint: String,
    /// Protocol type (grpc or http)
    pub protocol: OtelProtocol,
    /// Service name
    pub service_name: String,
}

impl OtelConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        let enabled = env::var("OTEL_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        let endpoint = env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:4317".to_string());

        let protocol = env::var("OTEL_EXPORTER_OTLP_PROTOCOL")
            .unwrap_or_else(|_| "grpc".to_string())
            .to_lowercase();

        let protocol = match protocol.as_str() {
            "http" => OtelProtocol::Http,
            _ => OtelProtocol::Grpc,
        };

        let service_name =
            env::var("OTEL_SERVICE_NAME").unwrap_or_else(|_| "pmp-test-api".to_string());

        Self {
            enabled,
            endpoint,
            protocol,
            service_name,
        }
    }
}

/// Initialize OpenTelemetry tracer provider
pub fn init_tracer(config: &OtelConfig) -> Result<TracerProvider, Box<dyn std::error::Error>> {
    info!(
        service_name = %config.service_name,
        endpoint = %config.endpoint,
        protocol = ?config.protocol,
        "Initializing OpenTelemetry tracer"
    );

    let resource = Resource::new(vec![KeyValue::new(
        "service.name",
        config.service_name.clone(),
    )]);

    let tracer_provider = match config.protocol {
        OtelProtocol::Grpc => {
            let exporter = opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint(&config.endpoint)
                .build()?;

            TracerProvider::builder()
                .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
                .with_resource(resource)
                .build()
        }
        OtelProtocol::Http => {
            let exporter = opentelemetry_otlp::SpanExporter::builder()
                .with_http()
                .with_endpoint(&config.endpoint)
                .build()?;

            TracerProvider::builder()
                .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
                .with_resource(resource)
                .build()
        }
    };

    info!("OpenTelemetry tracer initialized successfully");

    Ok(tracer_provider)
}

/// Shutdown the tracer provider gracefully
pub fn shutdown_tracer(provider: TracerProvider) {
    if let Err(err) = provider.shutdown() {
        eprintln!("Error shutting down tracer provider: {:?}", err);
    }
}
