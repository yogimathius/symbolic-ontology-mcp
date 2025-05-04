use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

use super::handlers;
use crate::domain::SymbolRepository;

/// Builds the main application router with all API routes
pub fn router(symbol_repository: Arc<dyn SymbolRepository>) -> Router {
    Router::new()
        .route("/health", get(handlers::health_check))
        .route("/symbols", get(handlers::list_symbols))
        .route("/symbols/{id}", get(handlers::get_symbol))
        .route("/symbols/{id}/related", get(handlers::get_related_symbols))
        .route("/categories", get(handlers::get_categories))
        .route("/interpret", post(handlers::interpret_symbol))
        .with_state(symbol_repository)
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
    }
}
