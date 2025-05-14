use clap::Parser;
use dream_ontology_mcp::domain::{RepositoryFactory, Symbol};
use dream_ontology_mcp::infrastructure::memory_repository::MemoryRepositoryFactory;
use dream_ontology_mcp::infrastructure::postgres_repository::PostgresRepositoryFactory;
use dream_ontology_mcp::logging::setup_logging;
use dream_ontology_mcp::mcp::service::SymbolService;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

/// Command line arguments for the MCP server
#[derive(Parser, Debug)]
struct Args {
    /// Port to bind the server to
    #[clap(short, long, default_value = "3002")]
    port: u16,

    /// Use memory repository instead of PostgreSQL
    #[clap(long)]
    memory: bool,

    /// Database URL for PostgreSQL repository
    #[clap(long)]
    database_url: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Setup enhanced logging
    setup_logging().expect("Failed to set up logging");

    // Get repository
    let repository = if args.memory {
        info!("Using in-memory repository");

        // Initialize memory repository
        let factory = MemoryRepositoryFactory::new();

        // Load myths from JSON file
        let json_path = Path::new("data/myth-symbol-seed.json");
        info!("Attempting to load symbols from: {}", json_path.display());

        let factory = if json_path.exists() {
            factory.with_json_data(json_path)
        } else {
            warn!("JSON file not found: {}", json_path.display());
            factory
        };

        factory.create_symbol_repository()
    } else {
        // Initialize the PostgreSQL repository factory
        let database_url = args
            .database_url
            .or_else(|| std::env::var("DATABASE_URL").ok())
            .unwrap_or_else(|| {
                "postgres://postgres:postgres@localhost/symbol_ontology".to_string()
            });

        info!("Connecting to database at {}", database_url);

        match PostgresRepositoryFactory::new(&database_url).await {
            Ok(factory) => {
                info!("Successfully connected to PostgreSQL database");
                let repo = factory.create_symbol_repository();

                // Test query to check connection
                match repo.list_symbols(None).await {
                    Ok(symbols) => {
                        info!(
                            "Successfully queried {} symbols from database",
                            symbols.len()
                        );
                    }
                    Err(e) => {
                        error!("Failed to query symbols from database: {:?}", e);
                    }
                }

                repo
            }
            Err(e) => {
                error!("Failed to connect to PostgreSQL database: {:?}", e);
                error!("Falling back to in-memory repository");

                // Fall back to memory repository
                let factory = MemoryRepositoryFactory::new();
                let json_path = Path::new("data/myth-symbol-seed.json");

                let factory = if json_path.exists() {
                    factory.with_json_data(json_path)
                } else {
                    warn!("JSON file not found: {}", json_path.display());
                    factory
                };

                factory.create_symbol_repository()
            }
        }
    };

    // Get symbols and categories for summary
    let symbols = repository.list_symbols(None).await.unwrap_or_else(|e| {
        error!("Failed to list symbols: {:?}", e);
        Vec::new()
    });

    // Count symbols by category
    let mut category_counts = HashMap::new();
    for symbol in &symbols {
        *category_counts.entry(symbol.category.clone()).or_insert(0) += 1;
    }

    // Start MCP server
    let port = std::env::var("PORT").unwrap_or_else(|_| args.port.to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    // Print server information in a cleaner format
    info!("=== Symbol Ontology MCP Server ===");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("Loaded {} total symbols", symbols.len());

    // Print categories in a more compact format
    let categories: Vec<String> = category_counts
        .iter()
        .map(|(cat, count)| format!("{}: {}", cat, count))
        .collect();
    info!("Categories: {}", categories.join(", "));

    info!("Starting server on {}", bind_address);
    info!("SSE endpoint: http://localhost:{}/sse", port);
    info!("Message endpoint: http://localhost:{}/message", port);
    info!("==============================");

    // Create a server configuration with custom paths
    let config = SseServerConfig {
        bind: bind_address.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: CancellationToken::new(),
    };

    // Create the core SSE server with enhanced logging
    let server = SseServer::serve_with_config(config)
        .await?
        .with_service(move || {
            info!("New client connected, creating SymbolService instance");
            SymbolService::new(repository.clone())
        });

    info!("Server ready to accept connections");

    // Keep the server running until Ctrl+C
    tokio::signal::ctrl_c().await?;
    server.cancel();
    info!("Server shutting down");

    Ok(())
}
