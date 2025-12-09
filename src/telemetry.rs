use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    logs::LoggerProvider,
    metrics::SdkMeterProvider,
    trace::TracerProvider,
    Resource,
};
use std::env;
use tracing::info;

/// OpenTelemetry protocol type
#[derive(Debug, Clone, PartialEq)]
pub enum OtelProtocol {
    Grpc,
    Http,
}

/// Exporter type for each signal
#[derive(Debug, Clone, PartialEq)]
pub enum ExporterType {
    Otlp,
    Console,
    None,
}

/// OpenTelemetry configuration from environment variables
#[derive(Debug, Clone)]
pub struct OtelConfig {
    /// Whether OpenTelemetry SDK is disabled (OTEL_SDK_DISABLED)
    pub disabled: bool,
    /// Traces exporter type (OTEL_TRACES_EXPORTER)
    pub traces_exporter: ExporterType,
    /// Metrics exporter type (OTEL_METRICS_EXPORTER)
    pub metrics_exporter: ExporterType,
    /// Logs exporter type (OTEL_LOGS_EXPORTER)
    pub logs_exporter: ExporterType,
    /// Collector endpoint URL (OTEL_EXPORTER_OTLP_ENDPOINT)
    pub endpoint: String,
    /// Protocol type (OTEL_EXPORTER_OTLP_PROTOCOL)
    pub protocol: OtelProtocol,
    /// Service name (OTEL_SERVICE_NAME)
    pub service_name: String,
}

/// Container for all OpenTelemetry providers
pub struct OtelProviders {
    pub tracer_provider: Option<TracerProvider>,
    #[allow(dead_code)]
    pub meter_provider: Option<SdkMeterProvider>,
    pub logger_provider: Option<LoggerProvider>,
}

fn parse_exporter_type(env_var: &str, default: ExporterType) -> ExporterType {
    env::var(env_var)
        .map(|v| match v.to_lowercase().as_str() {
            "otlp" => ExporterType::Otlp,
            "console" => ExporterType::Console,
            "none" => ExporterType::None,
            _ => default.clone(),
        })
        .unwrap_or(default)
}

fn parse_protocol() -> OtelProtocol {
    env::var("OTEL_EXPORTER_OTLP_PROTOCOL")
        .map(|v| match v.to_lowercase().as_str() {
            "http" | "http/protobuf" => OtelProtocol::Http,
            _ => OtelProtocol::Grpc,
        })
        .unwrap_or(OtelProtocol::Grpc)
}

fn parse_bool_env(env_var: &str, default: bool) -> bool {
    env::var(env_var)
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(default)
}

impl OtelConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Self {
        Self {
            disabled: parse_bool_env("OTEL_SDK_DISABLED", false),
            traces_exporter: parse_exporter_type("OTEL_TRACES_EXPORTER", ExporterType::Otlp),
            metrics_exporter: parse_exporter_type("OTEL_METRICS_EXPORTER", ExporterType::None),
            logs_exporter: parse_exporter_type("OTEL_LOGS_EXPORTER", ExporterType::None),
            endpoint: env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:4317".to_string()),
            protocol: parse_protocol(),
            service_name: env::var("OTEL_SERVICE_NAME")
                .unwrap_or_else(|_| "pmp-test-api".to_string()),
        }
    }

    /// Check if any exporter is enabled
    pub fn is_any_enabled(&self) -> bool {
        self.traces_exporter != ExporterType::None
            || self.metrics_exporter != ExporterType::None
            || self.logs_exporter != ExporterType::None
    }
}

fn create_resource(service_name: &str) -> Resource {
    Resource::new(vec![KeyValue::new("service.name", service_name.to_string())])
}

fn init_tracer_provider(config: &OtelConfig) -> Result<TracerProvider, Box<dyn std::error::Error>> {
    info!(
        endpoint = %config.endpoint,
        protocol = ?config.protocol,
        "Initializing OpenTelemetry tracer"
    );

    let resource = create_resource(&config.service_name);

    let provider = match config.protocol {
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

    Ok(provider)
}

fn init_meter_provider(config: &OtelConfig) -> Result<SdkMeterProvider, Box<dyn std::error::Error>> {
    info!(
        endpoint = %config.endpoint,
        protocol = ?config.protocol,
        "Initializing OpenTelemetry meter"
    );

    let resource = create_resource(&config.service_name);

    let provider = match config.protocol {
        OtelProtocol::Grpc => {
            let exporter = opentelemetry_otlp::MetricExporter::builder()
                .with_tonic()
                .with_endpoint(&config.endpoint)
                .build()?;

            let reader = opentelemetry_sdk::metrics::PeriodicReader::builder(
                exporter,
                opentelemetry_sdk::runtime::Tokio,
            )
            .build();

            SdkMeterProvider::builder()
                .with_reader(reader)
                .with_resource(resource)
                .build()
        }
        OtelProtocol::Http => {
            let exporter = opentelemetry_otlp::MetricExporter::builder()
                .with_http()
                .with_endpoint(&config.endpoint)
                .build()?;

            let reader = opentelemetry_sdk::metrics::PeriodicReader::builder(
                exporter,
                opentelemetry_sdk::runtime::Tokio,
            )
            .build();

            SdkMeterProvider::builder()
                .with_reader(reader)
                .with_resource(resource)
                .build()
        }
    };

    info!("OpenTelemetry meter initialized successfully");

    Ok(provider)
}

fn init_logger_provider(
    config: &OtelConfig,
) -> Result<LoggerProvider, Box<dyn std::error::Error>> {
    info!(
        endpoint = %config.endpoint,
        protocol = ?config.protocol,
        "Initializing OpenTelemetry logger"
    );

    let resource = create_resource(&config.service_name);

    let provider = match config.protocol {
        OtelProtocol::Grpc => {
            let exporter = opentelemetry_otlp::LogExporter::builder()
                .with_tonic()
                .with_endpoint(&config.endpoint)
                .build()?;

            LoggerProvider::builder()
                .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
                .with_resource(resource)
                .build()
        }
        OtelProtocol::Http => {
            let exporter = opentelemetry_otlp::LogExporter::builder()
                .with_http()
                .with_endpoint(&config.endpoint)
                .build()?;

            LoggerProvider::builder()
                .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
                .with_resource(resource)
                .build()
        }
    };

    info!("OpenTelemetry logger initialized successfully");

    Ok(provider)
}

/// Initialize all enabled OpenTelemetry providers
pub fn init_telemetry(config: &OtelConfig) -> Result<OtelProviders, Box<dyn std::error::Error>> {
    info!(
        service_name = %config.service_name,
        traces = ?config.traces_exporter,
        metrics = ?config.metrics_exporter,
        logs = ?config.logs_exporter,
        "Initializing OpenTelemetry"
    );

    let tracer_provider = if config.traces_exporter == ExporterType::Otlp {
        Some(init_tracer_provider(config)?)
    } else {
        None
    };

    let meter_provider = if config.metrics_exporter == ExporterType::Otlp {
        Some(init_meter_provider(config)?)
    } else {
        None
    };

    let logger_provider = if config.logs_exporter == ExporterType::Otlp {
        Some(init_logger_provider(config)?)
    } else {
        None
    };

    Ok(OtelProviders {
        tracer_provider,
        meter_provider,
        logger_provider,
    })
}

/// Shutdown all OpenTelemetry providers gracefully
#[allow(dead_code)]
pub fn shutdown_telemetry(providers: OtelProviders) {
    if let Some(tracer_provider) = providers.tracer_provider
        && let Err(err) = tracer_provider.shutdown()
    {
        eprintln!("Error shutting down tracer provider: {:?}", err);
    }

    if let Some(meter_provider) = providers.meter_provider
        && let Err(err) = meter_provider.shutdown()
    {
        eprintln!("Error shutting down meter provider: {:?}", err);
    }

    if let Some(logger_provider) = providers.logger_provider
        && let Err(err) = logger_provider.shutdown()
    {
        eprintln!("Error shutting down logger provider: {:?}", err);
    }
}
