// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use std::collections::HashMap;
use std::net::SocketAddr;

use anyhow::Result;
use clap::Parser;
use rmcp::server::MCPServer;
use rmcp::{errors::ManagedError, mcp, server::SSETransport, transport::MappedTransportError};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tracing::{error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[clap(name = "symbol-mcp", about = "Symbol Ontology MCP Client", version)]
struct Args {
    /// Port to listen on
    #[clap(short, long, env = "PORT", default_value = "3000")]
    port: u16,

    /// Enable verbose logging
    #[clap(short, long, env = "VERBOSE", action = clap::ArgAction::Count)]
    verbose: u8,

    /// API key for authentication
    #[clap(long, env = "SYMBOL_MCP_API_KEY")]
    api_key: Option<String>,

    /// API endpoint to connect to
    #[clap(
        long,
        env = "SYMBOL_MCP_API_URL",
        default_value = "http://localhost:8080/api/v1"
    )]
    api_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Symbol {
    id: String,
    name: String,
    category: String,
    description: String,
    interpretations: HashMap<String, String>,
    related_symbols: Vec<String>,
    properties: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    status: String,
    data: T,
}

// MCP service that connects to the API server
#[derive(Clone)]
struct SymbolMcpService {
    api_key: Option<String>,
    api_url: String,
    client: reqwest::Client,
}

impl SymbolMcpService {
    fn new(api_key: Option<String>, api_url: String) -> Self {
        Self {
            api_key,
            api_url,
            client: reqwest::Client::new(),
        }
    }

    // Get symbols from the API server
    async fn get_symbols(
        &self,
        query: Option<&str>,
        category: Option<&str>,
    ) -> mcp::ManagedResult<Vec<Symbol>> {
        let mut url = format!("{}/symbols", self.api_url);

        // Add query parameters if provided
        if query.is_some() || category.is_some() {
            url.push('?');
            if let Some(q) = query {
                url.push_str(&format!("query={}", q));
                if category.is_some() {
                    url.push('&');
                }
            }
            if let Some(c) = category {
                url.push_str(&format!("category={}", c));
            }
        }

        // Build the request with authentication if available
        let mut req = self.client.get(&url);
        if let Some(api_key) = &self.api_key {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }

        // Execute the request
        let response = req.send().await.map_err(|e| {
            ManagedError::new_with_message("API_ERROR", &format!("Failed to call API: {}", e))
        })?;

        // Check for success and parse the response
        if response.status().is_success() {
            let api_response: ApiResponse<Vec<Symbol>> = response.json().await.map_err(|e| {
                ManagedError::new_with_message(
                    "API_ERROR",
                    &format!("Failed to parse API response: {}", e),
                )
            })?;
            Ok(api_response.data)
        } else {
            Err(ManagedError::new_with_message(
                "API_ERROR",
                &format!("API returned error status: {}", response.status()),
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Setup logging
    setup_logging(args.verbose)?;

    // Create MCP service
    let service = SymbolMcpService::new(args.api_key, args.api_url.clone());

    // Log startup information
    info!("Symbol Ontology MCP Client v{}", env!("CARGO_PKG_VERSION"));
    info!("Connecting to API at: {}", args.api_url);
    info!("Starting MCP server on port {}", args.port);

    // Setup server
    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    let listener = TcpListener::bind(addr).await?;

    // Create MCP server
    let mut mcp_server = MCPServer::new();

    // Register MCP methods
    let service_clone = service.clone();
    mcp_server.register_method("get_symbols", move |params: mcp::Parameters| {
        let service = service_clone.clone();
        let query = params.get("query").and_then(|q| q.as_str());
        let category = params.get("category").and_then(|c| c.as_str());
        Box::pin(async move { service.get_symbols(query, category).await })
    });

    info!("MCP server is ready to accept connections");

    // Start handling connections
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                info!("Accepted new connection");
                let transport = SSETransport::new(stream);
                let server = mcp_server.clone();
                tokio::spawn(async move {
                    if let Err(e) = server.serve(transport).await {
                        match e {
                            MappedTransportError::Disconnect => info!("Client disconnected"),
                            _ => error!("Error serving client: {:?}", e),
                        }
                    }
                });
            }
            Err(e) => {
                error!("Error accepting connection: {}", e);
            }
        }
    }
}

fn setup_logging(verbose: u8) -> Result<()> {
    let level = match verbose {
        0 => tracing::Level::INFO,
        1 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env().add_directive(level.into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    Ok(())
}
