use std::net::SocketAddr;

use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod domain;
mod llm;
mod mcp;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "dream_ontology_mcp=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Dream Ontology MCP server v{}", utils::version());

    // For now, we'll use a simple API router without the MCP server
    // since we're having issues with the RMCP library
    debug!("Initialized with get_symbols handler");

    // Create router with API routes
    let app = Router::new()
        .route("/", get(root_handler))
        .merge(api::router())
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

// Basic health check handler
async fn root_handler() -> &'static str {
    "Dream Ontology MCP Server - OK"
}
