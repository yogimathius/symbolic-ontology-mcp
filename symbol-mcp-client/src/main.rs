// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::Duration;

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

// TODO: SSE Timeout Alternatives
//
// There are several better approaches to handling SSE timeouts:
//
// 1. Fork the RMCP library to add built-in heartbeat support
//    - Add a heartbeat_interval option to SseServerConfig
//    - Modify the SseServer implementation to send periodic comments
//
// 2. Use a different MCP transport that supports WebSockets
//    - WebSockets have built-in ping/pong frames for keepalive
//    - The MCP spec supports WebSocket transport
//
// 3. Modify the proxy configuration (for fly.io)
//    - Set HTTP_PROXY_CONNECT_TIMEOUT and HTTP_PROXY_IDLE_TIMEOUT environment variables
//    - Configure TCP keepalive at the operating system level
//
// 4. Use a custom middleware approach to wrap the SSE stream
//    - Implement a wrapper around SseServer that injects heartbeat events
//    - This is the most complex but least invasive option

#[derive(Parser, Debug)]
#[clap(name = "symbol-mcp", about = "Symbol Ontology MCP Client", version)]
struct Args {
    /// Port to listen on for local server
    #[arg(short, long, env("PORT"), default_value = "3000")]
    port: u16,

    /// Environment variable for MCP port, overrides port argument
    #[arg(long, env("MCP_PORT"))]
    mcp_port: Option<u16>,

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

    /// Heartbeat interval in seconds
    #[arg(long, env("HEARTBEAT_INTERVAL"), default_value = "30")]
    heartbeat_interval: u64,
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

    // Use MCP_PORT if available, fall back to PORT argument
    let port = args.mcp_port.unwrap_or(args.port);

    // Create socket address - always bind to 0.0.0.0 to listen on all interfaces
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

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
    info!("SSE endpoint: http://0.0.0.0:{}/sse", port);
    info!("Message endpoint: http://0.0.0.0:{}/message", port);
    info!("Heartbeat interval: {} seconds", args.heartbeat_interval);
    info!("==============================");

    // Configure heartbeat interval
    let heartbeat_interval = Duration::from_secs(args.heartbeat_interval);

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
