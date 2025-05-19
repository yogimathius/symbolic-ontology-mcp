// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use clap::Parser;
use symbol_ontology_mcp::config::Config;
use symbol_ontology_mcp::db::pool::create_pool;
use symbol_ontology_mcp::db::repository::PgRepositoryFactory;
use symbol_ontology_mcp::logging::setup_logging;
use symbol_ontology_mcp::mcp::service::SymbolService;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use std::collections::HashMap;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "3002")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    setup_logging().expect("Failed to set up logging");

    let _config = Config::from_env();

    // Use DATABASE_URL from environment variable
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/symbol_ontology".to_string());
    info!("Connecting to database at {}", database_url);

    let db_pool = match create_pool(&database_url).await {
        Ok(pool) => {
            info!("Successfully connected to PostgreSQL database");
            pool
        }
        Err(e) => {
            error!("Failed to connect to PostgreSQL database: {:?}", e);
            return Err(anyhow::anyhow!("Failed to connect to database: {}", e));
        }
    };

    match symbol_ontology_mcp::db::pool::init_database(&db_pool).await {
        Ok(_) => info!("Database schema initialized"),
        Err(e) => {
            error!("Failed to initialize database schema: {:?}", e);
            return Err(anyhow::anyhow!(
                "Failed to initialize database schema: {}",
                e
            ));
        }
    }

    let factory = PgRepositoryFactory::new(db_pool.clone());
    let symbol_repository = factory.create_symbol_repository();

    // Query for symbols count and categories
    let symbols = match symbol_repository.list_symbols(None).await {
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

    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(args.port);

    let bind_address = format!("0.0.0.0:{}", port);

    info!("=== Symbol Ontology MCP Server ===");
    info!("Version: {}", env!("CARGO_PKG_VERSION"));
    info!("Loaded {} total symbols", symbols.len());

    let categories: Vec<String> = category_counts
        .iter()
        .map(|(cat, count)| format!("{}: {}", cat, count))
        .collect();
    info!("Categories: {}", categories.join(", "));

    info!("Starting server on {}", bind_address);
    info!("SSE endpoint: http://localhost:{}/sse", port);
    info!("Message endpoint: http://localhost:{}/message", port);
    info!("==============================");

    let config = SseServerConfig {
        bind: bind_address.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: CancellationToken::new(),
    };

    let db_pool_clone = db_pool.clone();
    let server = SseServer::serve_with_config(config)
        .await?
        .with_service(move || {
            info!("New client connected, creating SymbolService instance");
            SymbolService::new_with_db(db_pool_clone.clone())
        });

    info!("Server ready to accept connections");

    tokio::signal::ctrl_c().await?;
    server.cancel();
    info!("Server shutting down");

    Ok(())
}
