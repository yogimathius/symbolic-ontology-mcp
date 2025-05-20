use async_trait::async_trait;
use serde_json;
use std::sync::Arc;

use ontology_core::db::repository::SymbolRepository;
use ontology_core::domain::Symbol;

use crate::mcp::methods::{
    get_symbols::{Handler, MethodCall, RmcpError},
    utils::repository_error_to_rmcp_error,
};
use crate::mcp::schema::{GetSymbolsResponse, SearchSymbolsParams, SymbolDTO};

pub struct SearchSymbolsHandler {
    symbol_repository: Arc<dyn SymbolRepository>,
}

impl SearchSymbolsHandler {
    pub fn new(symbol_repository: Arc<dyn SymbolRepository>) -> Self {
        SearchSymbolsHandler { symbol_repository }
    }

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
    fn method_name(&self) -> &str {
        "search_symbols"
    }

    async fn handle(&self, call: MethodCall) -> Result<serde_json::Value, RmcpError> {
        let params: SearchSymbolsParams = call.parse_params()?;

        // Validate query
        if params.query.trim().is_empty() {
            return Err(RmcpError::ParseError(
                "Search query cannot be empty".to_string(),
            ));
        }

        // Normalize query
        let normalized_query = params.query.trim().to_lowercase();

        // Perform search
        let symbols = self
            .symbol_repository
            .search_symbols(&normalized_query)
            .await
            .map_err(repository_error_to_rmcp_error)?;

        // Apply limit and convert to DTOs
        let symbol_dtos = symbols
            .iter()
            .take(params.limit)
            .map(Self::to_dto)
            .collect::<Vec<_>>();

        let total_count = symbols.len();

        Ok(serde_json::to_value(GetSymbolsResponse {
            symbols: symbol_dtos,
            total_count,
        })?)
    }
}

pub fn search_symbols(symbol_repository: Arc<dyn SymbolRepository>) -> SearchSymbolsHandler {
    SearchSymbolsHandler::new(symbol_repository)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "local")]
    #[tokio::test]
    async fn test_search_symbols_handler() {
        // TODO: Add proper tests with mock
    }

    #[cfg(feature = "local")]
    #[test]
    fn test_method_name() {
        // TODO: Add proper tests with mock
    }
}
