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

    /// Method to call (get_symbols, search_symbols, filter_by_category, get_categories)
    #[clap(short, long)]
    method: Option<String>,
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

    // Determine which method to call based on arguments
    let (method_name, params) = if let Some(method) = args.method.as_deref() {
        match method {
            "get_categories" => ("get_categories", json!({})),
            "search_symbols" => {
                if let Some(query) = &args.query {
                    (
                        "search_symbols",
                        json!({
                            "query": query,
                            "limit": args.limit
                        }),
                    )
                } else {
                    return Err(anyhow::anyhow!("search_symbols requires a query parameter"));
                }
            }
            "filter_by_category" => {
                if let Some(category) = &args.category {
                    (
                        "filter_by_category",
                        json!({
                            "category": category,
                            "limit": args.limit
                        }),
                    )
                } else {
                    return Err(anyhow::anyhow!(
                        "filter_by_category requires a category parameter"
                    ));
                }
            }
            _ => {
                // Default to get_symbols
                let mut params = json!({
                    "limit": args.limit
                });

                if let Some(category) = &args.category {
                    params["category"] = json!(category);
                }

                ("get_symbols", params)
            }
        }
    } else if args.query.is_some() {
        // If query is provided, use search_symbols
        (
            "search_symbols",
            json!({
                "query": args.query.as_ref().unwrap(),
                "limit": args.limit
            }),
        )
    } else if args.category.is_some() {
        // If category is provided, use filter_by_category
        (
            "filter_by_category",
            json!({
                "category": args.category.as_ref().unwrap(),
                "limit": args.limit
            }),
        )
    } else {
        // Default to get_symbols
        (
            "get_symbols",
            json!({
                "limit": args.limit
            }),
        )
    };

    println!("Calling method: {} with params: {}", method_name, params);

    // Call the tool with the parameters
    let tool_result = client
        .call_tool(CallToolRequestParam {
            name: method_name.into(),
            arguments: params.as_object().cloned(),
        })
        .await?;

    println!("Tool result: {tool_result:#?}");

    // Properly close the client connection
    client.cancel().await?;

    Ok(())
}
