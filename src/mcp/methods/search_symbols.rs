use async_trait::async_trait;
use serde_json;
use std::sync::Arc;

use crate::domain::{Symbol, SymbolRepository};
use crate::mcp::methods::get_symbols::{Handler, MethodCall, RmcpError};
use crate::mcp::schema::{GetSymbolsResponse, SearchSymbolsParams, SymbolDTO};

/// MCP handler for search_symbols method
pub struct SearchSymbolsHandler {
    // Repository for searching symbols
    symbol_repository: Arc<dyn SymbolRepository>,
}

impl SearchSymbolsHandler {
    /// Create a new handler with the provided repository
    pub fn new(symbol_repository: Arc<dyn SymbolRepository>) -> Self {
        SearchSymbolsHandler { symbol_repository }
    }

    /// Convert a domain Symbol to a DTO for the API
    fn to_dto(symbol: &Symbol) -> SymbolDTO {
        SymbolDTO {
            id: symbol.id.clone(),
            name: symbol.name.clone(),
            category: symbol.category.clone(),
            description: symbol.description.clone(),
            related_symbols: symbol.related_symbols.clone(),
        }
    }
}

#[async_trait]
impl Handler for SearchSymbolsHandler {
    /// The method name this handler responds to
    fn method_name(&self) -> &str {
        "search_symbols"
    }

    /// Handle the MCP method call
    async fn handle(&self, call: MethodCall) -> Result<serde_json::Value, RmcpError> {
        let params: SearchSymbolsParams = call.parse_params()?;

        // Search symbols from the repository using the query string
        let symbols = self.symbol_repository.search_symbols(&params.query).await?;

        // Apply limit
        let symbols = symbols
            .iter()
            .take(params.limit)
            .map(|s| Self::to_dto(s))
            .collect::<Vec<_>>();

        let total_count = symbols.len();

        Ok(serde_json::to_value(GetSymbolsResponse {
            symbols,
            total_count,
        })?)
    }
}

/// Factory function to create the search_symbols handler with provided repository
pub fn search_symbols(symbol_repository: Arc<dyn SymbolRepository>) -> SearchSymbolsHandler {
    SearchSymbolsHandler::new(symbol_repository)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::RepositoryFactory;
    use crate::infrastructure::memory_repository::MemoryRepositoryFactory;
    use serde_json::json;

    #[tokio::test]
    async fn test_search_symbols_handler() {
        // Create a real repository with test data
        let factory = MemoryRepositoryFactory::new().with_test_data();
        let repository = factory.create_symbol_repository();

        let handler = SearchSymbolsHandler::new(repository);

        // Create a method call with a search query
        let call = MethodCall {
            id: "test-call".to_string(),
            method: "search_symbols".to_string(),
            params: json!({
                "query": "water" // Assuming test data has water-related symbols
            }),
        };

        let result = handler.handle(call).await;

        assert!(result.is_ok());
        let response = result.unwrap();

        // Validate response structure
        assert!(response.is_object());
        assert!(response.as_object().unwrap().contains_key("symbols"));
        assert!(response.as_object().unwrap().contains_key("total_count"));
    }

    #[test]
    fn test_method_name() {
        let factory = MemoryRepositoryFactory::new();
        let repository = factory.create_symbol_repository();

        let handler = SearchSymbolsHandler::new(repository);
        assert_eq!(handler.method_name(), "search_symbols");
    }
}
