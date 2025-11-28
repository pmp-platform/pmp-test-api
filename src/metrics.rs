use axum_prometheus::PrometheusMetricLayerBuilder;
use axum::{
    response::IntoResponse,
};
use metrics_exporter_prometheus::PrometheusHandle;

/// Metrics state that holds the prometheus metric handle
#[derive(Clone)]
pub struct MetricsState {
    pub handle: PrometheusHandle,
}

/// Create the prometheus metric layer with custom configuration
pub fn create_metric_layer() -> (axum_prometheus::PrometheusMetricLayer<'static>, MetricsState) {
    let (metric_layer, handle) = PrometheusMetricLayerBuilder::new()
        .with_default_metrics()
        .build_pair();

    let state = MetricsState { handle };

    (metric_layer, state)
}

/// Handler for the /metrics endpoint
pub async fn metrics_handler(
    axum::extract::State(state): axum::extract::State<MetricsState>,
) -> impl IntoResponse {
    state.handle.render()
}
