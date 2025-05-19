// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use anyhow::Result;
use clap::Parser;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use serde_json::json;
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[clap(name = "dream-mcp", about = "Dream Ontology MCP Client", version)]
struct Args {
    /// Port to listen on
    #[clap(short, long, env = "PORT", default_value = "3002")]
    port: u16,

    /// API key for authentication
    #[clap(long, env = "DREAM_MCP_API_KEY")]
    api_key: Option<String>,

    /// API endpoint to connect to
    #[clap(
        long,
        env = "DREAM_MCP_API_URL",
        default_value = "https://api.dreamontology.example.com"
    )]
    api_url: String,

    /// Enable verbose logging
    #[clap(short, long, env = "VERBOSE")]
    verbose: bool,
}

struct DreamMcpService {
    api_key: Option<String>,
    api_url: String,
    client: reqwest::Client,
}

impl DreamMcpService {
    fn new(api_key: Option<String>, api_url: String) -> Self {
        Self {
            api_key,
            api_url,
            client: reqwest::Client::new(),
        }
    }

    // This will be expanded when we implement the full API integration
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    setup_logging(args.verbose)?;

    // Create the service
    let service = DreamMcpService::new(args.api_key, args.api_url.clone());

    // Start the MCP server
    let bind_address = format!("0.0.0.0:{}", args.port);

    info!("Dream Ontology MCP Client v{}", env!("CARGO_PKG_VERSION"));
    info!("Connecting to API: {}", args.api_url);
    info!("Starting local MCP server on {}", bind_address);

    let config = SseServerConfig {
        bind: bind_address.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: CancellationToken::new(),
    };

    // Start the server with our service
    // We'll implement the actual service later
    let server = SseServer::serve_with_config(config).await?;

    // TODO: Add the service implementation

    info!("Server started. Press Ctrl+C to exit...");

    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await?;
    server.cancel();
    info!("Server shutting down...");

    Ok(())
}

fn setup_logging(verbose: bool) -> Result<()> {
    let level = if verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env().add_directive(level.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}
