mod check;
mod env_parser;
mod handlers;
mod models;
mod routes;

use routes::create_router;
use std::env;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if it exists (useful for local development)
    let _ = dotenvy::dotenv();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,pmp_test_api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting pmp-test-api");

    // Get port from environment or use default
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    let addr = format!("0.0.0.0:{}", port);

    info!("Binding to {}", addr);

    // Create router with tracing layer
    let app = create_router().layer(TraceLayer::new_for_http());

    // Create TCP listener
    let listener = TcpListener::bind(&addr).await?;

    info!("Server listening on {}", addr);
    info!("Dashboard: http://{}/", addr);
    info!("Health endpoint: http://{}/_/health", addr);
    info!("Info endpoint: http://{}/_/info", addr);

    // Start the server
    axum::serve(listener, app).await?;

    Ok(())
}
