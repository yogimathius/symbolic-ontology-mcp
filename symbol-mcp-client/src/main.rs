// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use std::collections::HashMap;
use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;
use tokio::signal;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import rmcp types
use rmcp::transport::sse_server::{SseServer, SseServerConfig};

mod mcp;
use mcp::service::SymbolService;

#[derive(Parser, Debug)]
#[clap(name = "symbol-mcp", about = "Symbol Ontology MCP Client", version)]
struct Args {
    /// Port to listen on for local server
    #[arg(short, long, env("PORT"), default_value = "3000")]
    port: u16,

    /// Verbosity level (can be used multiple times)
    #[arg(short, long, env("VERBOSE"), action = clap::ArgAction::Count)]
    verbose: u8,

    /// Database URL for local database
    #[clap(
        long,
        env = "DATABASE_URL",
        default_value = "postgres://postgres:postgres@localhost:5432/symbol_ontology"
    )]
    database_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "symbol_mcp_client=info,rmcp=info,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Parse command line arguments
    let args = Args::parse();

    // Set up logging level based on verbosity
    let verbosity = match args.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };
    debug!("Verbosity level: {}", verbosity);

    // Create socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    // Connect to database
    info!("Connecting to database at {}", args.database_url);
    let pool = ontology_core::db::pool::create_pool(&args.database_url)
        .await
        .expect("Failed to create database connection pool");

    // Create repositories
    let symbol_repo = std::sync::Arc::new(ontology_core::db::repository::PgSymbolRepository::new(
        pool.clone(),
    )) as std::sync::Arc<dyn ontology_core::db::repository::SymbolRepository>;
    let symbol_set_repo = std::sync::Arc::new(
        ontology_core::db::repository::PgSymbolSetRepository::new(pool.clone()),
    )
        as std::sync::Arc<dyn ontology_core::db::repository::SymbolSetRepository>;

    // Query for symbols count and categories
    let symbols = match symbol_repo.list_symbols(None).await {
        Ok(symbols) => {
            info!(
                "Successfully queried {} symbols from database",
                symbols.len()
            );
            symbols
        }
        Err(e) => {
            error!("Failed to query symbols from database: {:?}", e);
            Vec::new()
        }
    };

    let mut category_counts = HashMap::new();
    for symbol in &symbols {
        *category_counts.entry(symbol.category.clone()).or_insert(0) += 1;
    }

    // Create service with repositories
    let service = SymbolService {
        symbol_repository: symbol_repo.clone(),
        symbol_set_repository: symbol_set_repo.clone(),
    };

    info!("=== Symbol Ontology MCP Server ===");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("Loaded {} total symbols", symbols.len());

    let categories: Vec<String> = category_counts
        .iter()
        .map(|(cat, count)| format!("{}: {}", cat, count))
        .collect();
    info!("Categories: {}", categories.join(", "));

    info!("Starting server on {}", addr);
    info!("SSE endpoint: http://localhost:{}/sse", args.port);
    info!("Message endpoint: http://localhost:{}/message", args.port);
    info!("==============================");

    // Create a proper transport
    let config = SseServerConfig {
        bind: addr,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: CancellationToken::new(),
    };

    let server_handle = SseServer::serve_with_config(config)
        .await?
        .with_service(move || service.clone());

    // We need to run this in a task so we can also listen for Ctrl+C
    let _server_task = tokio::spawn(async {
        info!("MCP server started");
        // Just keep the server running without trying to await it
    });

    // Wait for Ctrl+C
    match signal::ctrl_c().await {
        Ok(()) => {
            info!("Received Ctrl+C, shutting down gracefully");
        }
        Err(err) => {
            error!("Unable to listen for shutdown signal: {}", err);
        }
    }

    // Cancel the server
    server_handle.cancel();
    info!("MCP server shut down");

    Ok(())
}
