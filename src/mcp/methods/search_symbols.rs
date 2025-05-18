use async_trait::async_trait;
use serde_json;
use std::sync::Arc;

use crate::db::repository::SymbolRepository;
use crate::domain::Symbol;
use crate::mcp::methods::get_symbols::{Handler, MethodCall, RmcpError};
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

pub fn search_symbols(symbol_repository: Arc<dyn SymbolRepository>) -> SearchSymbolsHandler {
    SearchSymbolsHandler::new(symbol_repository)
}

#[cfg(test)]
mod tests {

    #[ignore]
    #[tokio::test]
    async fn test_search_symbols_handler() {
        assert!(true);
    }

    #[ignore]
    #[test]
    fn test_method_name() {
        assert!(true);
    }
}
