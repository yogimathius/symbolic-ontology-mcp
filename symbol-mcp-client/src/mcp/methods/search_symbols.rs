use async_trait::async_trait;
use serde_json;
use std::sync::Arc;

#[cfg(feature = "local")]
use ontology_core::db::repository::SymbolRepository;
#[cfg(feature = "local")]
use ontology_core::domain::symbols::Symbol;

use crate::mcp::methods::get_symbols::{Handler, MethodCall, RmcpError};
use crate::mcp::schema::{GetSymbolsResponse, SearchSymbolsParams, SymbolDTO};

#[cfg(feature = "local")]
pub struct SearchSymbolsHandler {
    symbol_repository: Arc<dyn SymbolRepository>,
}

#[cfg(feature = "local")]
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

#[cfg(feature = "local")]
#[async_trait]
impl Handler for SearchSymbolsHandler {
    fn method_name(&self) -> &str {
        "search_symbols"
    }

    async fn handle(&self, call: MethodCall) -> Result<serde_json::Value, RmcpError> {
        let params: SearchSymbolsParams = call.parse_params()?;

        let symbols = self.symbol_repository.search_symbols(&params.query).await?;

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

#[cfg(feature = "local")]
pub fn search_symbols(symbol_repository: Arc<dyn SymbolRepository>) -> SearchSymbolsHandler {
    SearchSymbolsHandler::new(symbol_repository)
}

#[cfg(not(feature = "local"))]
pub fn search_symbols() {
    // Stub for when local feature is not enabled
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
