use clap::Parser;
use dream_ontology_mcp::domain::RepositoryFactory;
use dream_ontology_mcp::infrastructure::memory_repository::MemoryRepositoryFactory;
use dream_ontology_mcp::logging::setup_logging;
use dream_ontology_mcp::mcp::service::SymbolService;
use rmcp::transport::sse_server::SseServer;
use tracing::info;

/// Command line arguments for the MCP server
#[derive(Parser, Debug)]
struct Args {
    /// Port to bind the server to
    #[clap(short, long, default_value = "3002")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Setup enhanced logging
    setup_logging().expect("Failed to set up logging");

    // Initialize repository
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    // Start MCP server
    let bind_address = format!("127.0.0.1:{}", args.port);

    info!("Starting MCP server on {}", bind_address);

    let server = SseServer::serve(bind_address.parse()?)
        .await?
        .with_service(move || SymbolService::new(repository.clone()));

    // Keep the server running until Ctrl+C
    tokio::signal::ctrl_c().await?;
    server.cancel();

    Ok(())
}
