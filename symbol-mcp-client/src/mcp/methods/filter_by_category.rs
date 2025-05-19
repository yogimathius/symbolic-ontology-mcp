use async_trait::async_trait;
use serde_json;
use std::sync::Arc;

#[cfg(feature = "local")]
use ontology_core::db::repository::SymbolRepository;
#[cfg(feature = "local")]
use ontology_core::domain::symbols::Symbol;

use crate::mcp::methods::get_symbols::{Handler, MethodCall, RmcpError};
use crate::mcp::schema::{CategorySymbolsParams, GetSymbolsResponse, SymbolDTO};

#[cfg(feature = "local")]
pub struct FilterByCategoryHandler {
    symbol_repository: Arc<dyn SymbolRepository>,
}

#[cfg(feature = "local")]
impl FilterByCategoryHandler {
    pub fn new(symbol_repository: Arc<dyn SymbolRepository>) -> Self {
        FilterByCategoryHandler { symbol_repository }
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
impl Handler for FilterByCategoryHandler {
    fn method_name(&self) -> &str {
        "filter_by_category"
    }

    async fn handle(&self, call: MethodCall) -> Result<serde_json::Value, RmcpError> {
        let params: CategorySymbolsParams = call.parse_params()?;

        let symbols = self
            .symbol_repository
            .filter_by_category(&params.category)
            .await?;

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
pub fn filter_by_category(symbol_repository: Arc<dyn SymbolRepository>) -> FilterByCategoryHandler {
    FilterByCategoryHandler::new(symbol_repository)
}

#[cfg(not(feature = "local"))]
pub fn filter_by_category() {
    // Stub for when local feature is not enabled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "local")]
    #[tokio::test]
    async fn test_filter_by_category_handler() {
        // TODO: Add proper tests with mock
    }

    #[cfg(feature = "local")]
    #[test]
    fn test_method_name() {
        // TODO: Add proper tests with mock
    }
}
