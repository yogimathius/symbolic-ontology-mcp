use dream_ontology_mcp::domain::{RepositoryFactory, SymbolRepository};
use dream_ontology_mcp::infrastructure::memory_repository::MemoryRepositoryFactory;
use dream_ontology_mcp::mcp::service::SymbolService;
use rmcp::transport::sse_server::SseServer;
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize repository
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    // Start MCP server
    let bind_address = "127.0.0.1:3001";

    println!("Starting MCP server on {}", bind_address);

    let server = SseServer::serve(bind_address.parse()?)
        .await?
        .with_service(move || SymbolService::new(repository.clone()));

    // Keep the server running until Ctrl+C
    tokio::signal::ctrl_c().await?;
    server.cancel();

    Ok(())
}
