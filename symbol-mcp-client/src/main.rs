// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;
use reqwest::Client;
use rmcp::server::MCPServer;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{error, info, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod mcp;

#[derive(Parser, Debug)]
#[clap(name = "symbol-mcp", about = "Symbol Ontology MCP Client", version)]
struct Args {
    /// Port to listen on for local server
    #[arg(short, long, env("PORT"), default_value = "3000")]
    port: u16,

    /// Verbosity level (can be used multiple times)
    #[arg(short, long, env("VERBOSE"), action = clap::ArgAction::Count)]
    verbose: u8,

    /// API key for accessing the Symbol Ontology API
    #[arg(long, env("SYMBOL_MCP_API_KEY"))]
    api_key: Option<String>,

    /// API endpoint for the Symbol Ontology API
    #[arg(long, env("SYMBOL_MCP_API_ENDPOINT"))]
    api_endpoint: Option<String>,

    #[cfg(feature = "local")]
    /// Database URL when using local database
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

    // Create MCP server
    let mcp_server = create_mcp_server(&args).await?;

    // Create socket address
    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    let listener = TcpListener::bind(addr).await?;
    info!("MCP server listening on http://{}", addr);

    // Run MCP server
    mcp_server.serve(listener).await?;

    Ok(())
}

async fn create_mcp_server(args: &Args) -> Result<MCPServer> {
    // Create MCP server with symbol service
    #[cfg(feature = "local")]
    {
        use sqlx::postgres::PgPoolOptions;
        use std::sync::Arc;

        // Create database connection pool
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&args.database_url)
            .await?;

        // Create repository factory
        let factory = ontology_core::db::repository::PgRepositoryFactory::new(pool);

        // Create repositories
        let symbol_repository = factory.create_symbol_repository();

        // Create symbol service
        let symbol_service = mcp::service::SymbolService::new(symbol_repository);

        // Create MCP server with local database service
        Ok(MCPServer::builder()
            .add_handler(Arc::new(symbol_service))
            .build())
    }

    #[cfg(not(feature = "local"))]
    {
        use reqwest::Client;

        // Create HTTP client
        let client = Client::builder().build()?;

        // Create API endpoint
        let api_endpoint = args
            .api_endpoint
            .clone()
            .unwrap_or_else(|| "https://api.symbol-ontology.com".to_string());

        // Create MCP server with HTTP client service (connects to remote API)
        let symbol_service = mcp::service::SymbolService::new_with_client(
            client,
            api_endpoint,
            args.api_key.clone(),
        );

        Ok(MCPServer::builder()
            .add_handler(Arc::new(symbol_service))
            .build())
    }
}
