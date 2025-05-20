use async_trait::async_trait;
use serde_json;
use std::sync::Arc;

use ontology_core::db::repository::SymbolRepository;
use ontology_core::domain::Symbol;

use crate::mcp::methods::{
    get_symbols::{Handler, MethodCall, RmcpError},
    utils::repository_error_to_rmcp_error,
};
use crate::mcp::schema::{CategorySymbolsParams, GetSymbolsResponse, SymbolDTO};

pub struct FilterByCategoryHandler {
    symbol_repository: Arc<dyn SymbolRepository>,
}

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

#[async_trait]
impl Handler for FilterByCategoryHandler {
    fn method_name(&self) -> &str {
        "filter_by_category"
    }

    async fn handle(&self, call: MethodCall) -> Result<serde_json::Value, RmcpError> {
        let params: CategorySymbolsParams = call.parse_params()?;

        // Validate category
        if params.category.trim().is_empty() {
            return Err(RmcpError::ParseError(
                "Category cannot be empty".to_string(),
            ));
        }

        // Fetch symbols by category
        let symbols = self
            .symbol_repository
            .list_symbols(Some(&params.category))
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

pub fn filter_by_category(symbol_repository: Arc<dyn SymbolRepository>) -> FilterByCategoryHandler {
    FilterByCategoryHandler::new(symbol_repository)
}
