use std::net::SocketAddr;
use std::sync::Arc;

use axum::serve;
use tokio::net::TcpListener;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::domain::RepositoryFactory;
use crate::infrastructure::memory_repository::MemoryRepositoryFactory;
use crate::logging::{init_tracing, trace_layer};

/// API module containing HTTP endpoints and request handlers
mod api;

/// Configuration module for loading and managing application settings
mod config;

/// Domain module containing core business logic and models
mod domain;

/// Infrastructure module for external integrations and persistence
mod infrastructure;

// TODO: This module belongs to the Dream Interpretation Backend, not the Symbolic Ontology
// It will be removed from this repository and implemented in a separate MCP client service
// that consumes data from this service for dream interpretation.
/// LLM integration module for language model interaction
mod llm;

/// Logging configuration and utilities
mod logging;

/// MCP implementation for protocol-compliant symbolic reasoning
mod mcp;

/// Utility functions and shared helpers
mod utils;

/// Main application entry point for the Dream Ontology Symbolic MCP Server.
///
/// This starts an Axum-based HTTP server that provides:
/// 1. REST API endpoints for symbol and ontology management
/// 2. MCP protocol endpoints for semantic reasoning
///
/// The server uses a repository pattern to abstract data access, with
/// in-memory implementation available for development and testing.
///
/// Note: This server does NOT handle LLM integration directly - that functionality
/// will be implemented in a separate MCP client service. This service focuses solely
/// on providing accurate symbolic data as an MCP server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment
    let config = Config::from_env();

    // Initialize tracing with config
    init_tracing(&config);

    info!(
        "Starting Dream Ontology Symbolic MCP Server v{}",
        utils::version()
    );

    debug!(?config, "Loaded configuration");

    // Initialize repositories based on configuration
    let repo_factory = if config.use_memory_repository {
        info!("Using in-memory repository with test data");
        MemoryRepositoryFactory::new().with_test_data()
    } else {
        // In the future, we could add other repository implementations here
        info!("Using in-memory repository (default)");
        MemoryRepositoryFactory::new().with_test_data()
    };

    let symbol_repository = repo_factory.create_symbol_repository();

    // Create the API router with repository dependency and logging middleware
    let app = api::router(symbol_repository.clone()).layer(trace_layer());

    // TODO: Initialize MCP server with method handlers
    // using the same repository
    debug!("API Server initialized with symbol repository and logging middleware");

    // Start HTTP server
    info!("API Server listening on {}", config.server_addr);

    // Create a TCP listener and serve with Axum
    let listener = TcpListener::bind(&config.server_addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    info!("Server shutting down");

    Ok(())
}
