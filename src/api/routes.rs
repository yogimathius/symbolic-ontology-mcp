use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use super::handlers;
use crate::domain::SymbolRepository;
use crate::mcp::service::SymbolService;
use rmcp::transport::sse_server::SseServer;

/// Builds the main application router with all API routes
pub fn router(symbol_repository: Arc<dyn SymbolRepository>) -> Router {
    let api_router = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/symbols", get(handlers::list_symbols))
        .route("/symbols/{id}", get(handlers::get_symbol))
        .with_state(symbol_repository.clone());

    // For now, just return the API router
    // TODO: Add MCP server support once we resolve the integration issues
    api_router
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::RepositoryFactory;
    use crate::infrastructure::memory_repository::MemoryRepositoryFactory;

    #[test]
    fn test_router_creation() {
        let factory = MemoryRepositoryFactory::new();
        let repository = factory.create_symbol_repository();

        let _router = router(repository);
        // Just testing that it builds - actual routes tested in integration tests
    }
}
