use tokio::net::TcpListener;
use tracing::{debug, info};

use crate::config::Config;
use crate::domain::RepositoryFactory;
use crate::infrastructure::memory_repository::MemoryRepositoryFactory;
use crate::infrastructure::postgres_repository::PostgresRepositoryFactory;
use crate::logging::{init_tracing, trace_layer};

/// API module containing HTTP endpoints and request handlers
mod api;

/// Configuration module for loading and managing application settings
mod config;

/// Domain module containing core business logic and models
mod domain;

/// Infrastructure module for external integrations and persistence
mod infrastructure;

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
    // Load .env file if present
    let dot_env_result = dotenvy::dotenv();

    // Debug environment variables - Use println for now before tracing is initialized
    let server_addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "not set".to_string());
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "not set".to_string());
    let use_memory =
        std::env::var("USE_MEMORY_REPOSITORY").unwrap_or_else(|_| "not set".to_string());
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "not set".to_string());

    println!("Environment variables before config parsing:");
    println!("SERVER_ADDR={}", server_addr);
    println!("LOG_LEVEL={}", log_level);
    println!("USE_MEMORY_REPOSITORY={}", use_memory);
    println!("DATABASE_URL={}", db_url);

    match dot_env_result {
        Ok(path) => println!("Loaded .env from {}", path.display()),
        Err(e) => println!("Could not load .env file: {}", e),
    }

    // Load configuration from environment
    let config = Config::from_env();

    // Initialize tracing with config
    init_tracing(&config);

    info!(
        "Starting Dream Ontology Symbolic MCP Server v{}",
        utils::version()
    );

    // Log the configuration
    debug!("Parsed configuration: {:?}", config);

    info!("Loaded configuration: {:?}", config);

    // Initialize repositories based on configuration
    let symbol_repository = if !config.use_memory_repository {
        if let Some(db_url) = &config.database_url {
            info!("Using PostgreSQL repository");
            // Connect to PostgreSQL and create repository factory
            let pg_factory = PostgresRepositoryFactory::new(db_url)
                .await?
                .with_test_data()
                .await?;
            pg_factory.create_symbol_repository()
        } else {
            info!("No database URL provided, falling back to in-memory repository with test data");
            let memory_factory = MemoryRepositoryFactory::new().with_test_data();
            memory_factory.create_symbol_repository()
        }
    } else {
        info!("Using in-memory repository with test data");
        let memory_factory = MemoryRepositoryFactory::new().with_test_data();
        memory_factory.create_symbol_repository()
    };

    // Create the API router with repository dependency and logging middleware
    let app = api::router(symbol_repository.clone()).layer(trace_layer());

    debug!("API Server initialized with symbol repository and logging middleware");

    // Start HTTP server
    info!("API Server listening on {}", config.server_addr);

    // Create a TCP listener and serve with Axum
    let listener = TcpListener::bind(&config.server_addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    info!("Server shutting down");

    Ok(())
}
