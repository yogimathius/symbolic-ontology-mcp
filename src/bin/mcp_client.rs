use anyhow::Result;
use clap::Parser;
use rmcp::{
    ServiceExt,
    model::{CallToolRequestParam, ClientCapabilities, ClientInfo, Implementation},
    transport::SseTransport,
};
use serde_json::json;
use std::fmt::Debug;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Command line arguments for the MCP client
#[derive(Parser, Debug)]
struct Args {
    /// Server address to connect to
    #[clap(short, long, default_value = "http://127.0.0.1:3002/sse")]
    server: String,

    /// Number of symbols to retrieve
    #[clap(short, long, default_value = "10")]
    limit: u32,

    /// Optional query to filter symbols
    #[clap(short, long)]
    query: Option<String>,

    /// Optional category to filter symbols
    #[clap(short, long)]
    category: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("info,{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("Connecting to MCP server at {}", args.server);

    // Start the SSE transport to connect to the server
    let transport = SseTransport::start(&args.server).await?;

    // Create client info
    let client_info = ClientInfo {
        protocol_version: Default::default(),
        capabilities: ClientCapabilities::default(),
        client_info: Implementation {
            name: "Dream Ontology MCP Test Client".to_string(),
            version: "0.1.0".to_string(),
        },
    };

    // Serve the client and handle any errors
    let client = client_info.serve(transport).await.inspect_err(|e| {
        tracing::error!("Client error: {:?}", e);
    })?;

    // Get server info
    let server_info = client.peer_info();
    println!("Connected to server: {server_info:#?}");

    // List available tools
    let tools = client.list_tools(Default::default()).await?;
    println!("Available tools: {tools:#?}");

    // Build get_symbols parameters
    let mut params = json!({
        "limit": args.limit
    });

    if let Some(query) = args.query {
        params["query"] = json!(query);
    }

    if let Some(category) = args.category {
        params["category"] = json!(category);
    }

    // Call the get_symbols tool with the parameters
    let tool_result = client
        .call_tool(CallToolRequestParam {
            name: "get_symbols".into(),
            arguments: params.as_object().cloned(),
        })
        .await?;

    println!("Tool result: {tool_result:#?}");

    // Properly close the client connection
    client.cancel().await?;

    Ok(())
}
