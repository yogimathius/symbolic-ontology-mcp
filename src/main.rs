use std::net::SocketAddr;
use std::sync::Arc;

use axum::serve;
use tokio::net::TcpListener;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::domain::RepositoryFactory;
use crate::infrastructure::memory_repository::MemoryRepositoryFactory;

mod api;
mod domain;
mod infrastructure;
mod llm; // TODO: This should be removed as it belongs to the Dream Interpretation Backend
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

    info!(
        "Starting Dream Ontology Symbolic MCP Server v{}",
        utils::version()
    );

    // Initialize repositories with test data
    let repo_factory = MemoryRepositoryFactory::new().with_test_data();
    let symbol_repository = repo_factory.create_symbol_repository();

    // Create the API router with repository dependency
    let app = api::router(symbol_repository.clone());

    // TODO: Initialize MCP server with method handlers
    // using the same repository
    debug!("API Server initialized with symbol repository");

    // Start HTTP server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("API Server listening on {}", addr);

    // Create a TCP listener and serve with Axum
    let listener = TcpListener::bind(&addr).await?;
    serve(listener, app.into_make_service()).await?;

    info!("Server shutting down");

    Ok(())
}
