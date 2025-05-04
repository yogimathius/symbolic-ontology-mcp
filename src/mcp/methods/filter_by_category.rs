use async_trait::async_trait;
use serde_json;
use std::sync::Arc;

use crate::domain::{Symbol, SymbolRepository};
use crate::mcp::methods::get_symbols::{Handler, MethodCall, RmcpError};
use crate::mcp::schema::{CategorySymbolsParams, GetSymbolsResponse, SymbolDTO};

/// MCP handler for filter_by_category method
pub struct FilterByCategoryHandler {
    // Repository for filtering symbols by category
    symbol_repository: Arc<dyn SymbolRepository>,
}

impl FilterByCategoryHandler {
    /// Create a new handler with the provided repository
    pub fn new(symbol_repository: Arc<dyn SymbolRepository>) -> Self {
        FilterByCategoryHandler { symbol_repository }
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
impl Handler for FilterByCategoryHandler {
    /// The method name this handler responds to
    fn method_name(&self) -> &str {
        "filter_by_category"
    }

    /// Handle the MCP method call
    async fn handle(&self, call: MethodCall) -> Result<serde_json::Value, RmcpError> {
        let params: CategorySymbolsParams = call.parse_params()?;

        // Filter symbols by category
        let symbols = self
            .symbol_repository
            .list_symbols(Some(&params.category))
            .await?;

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

/// Factory function to create the filter_by_category handler with provided repository
pub fn filter_by_category(symbol_repository: Arc<dyn SymbolRepository>) -> FilterByCategoryHandler {
    FilterByCategoryHandler::new(symbol_repository)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::RepositoryFactory;
    use crate::infrastructure::memory_repository::MemoryRepositoryFactory;
    use serde_json::json;

    #[tokio::test]
    async fn test_filter_by_category_handler() {
        // Create a real repository with test data
        let factory = MemoryRepositoryFactory::new().with_test_data();
        let repository = factory.create_symbol_repository();

        let handler = FilterByCategoryHandler::new(repository);

        // Create a method call with a category parameter
        let call = MethodCall {
            id: "test-call".to_string(),
            method: "filter_by_category".to_string(),
            params: json!({
                "category": "dream" // Assuming test data has dream category symbols
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

        let handler = FilterByCategoryHandler::new(repository);
        assert_eq!(handler.method_name(), "filter_by_category");
    }
}
