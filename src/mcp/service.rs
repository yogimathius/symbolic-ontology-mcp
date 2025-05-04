use rmcp::{ServerHandler, model::*, tool};
use std::sync::Arc;

use crate::domain::SymbolRepository;
use crate::mcp::schema::{CategorySymbolsParams, GetSymbolsParams, SearchSymbolsParams};

#[derive(Clone)]
pub struct SymbolService {
    repository: Arc<dyn SymbolRepository>,
}

#[tool(tool_box)]
impl SymbolService {
    #[allow(dead_code)]
    pub fn new(repository: Arc<dyn SymbolRepository>) -> Self {
        Self { repository }
    }

    #[tool(
        description = "List all symbols or filter by category (use search_symbols for text search)"
    )]
    async fn get_symbols(
        &self,
        #[tool(aggr)] params: GetSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        // Only use this method when not performing text search
        // For text search, use search_symbols instead
        let symbols = match params.category.as_deref() {
            Some(category) => self.repository.list_symbols(Some(category)).await?,
            None => self.repository.list_symbols(None).await?,
        };

        let total_count = symbols.len();
        let symbols = symbols
            .iter()
            .take(params.limit)
            .map(|s| crate::mcp::schema::SymbolDTO {
                id: s.id.clone(),
                name: s.name.clone(),
                category: s.category.clone(),
                description: s.description.clone(),
                related_symbols: s.related_symbols.clone(),
            })
            .collect::<Vec<_>>();

        let content = Content::json(serde_json::json!({
            "symbols": symbols,
            "total_count": total_count
        }));

        let result = CallToolResult::success(vec![content?]);
        Ok(result)
    }

    #[tool(description = "Search symbols by text query - use this for all text searches")]
    async fn search_symbols(
        &self,
        #[tool(aggr)] params: SearchSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        // This is the recommended method for text search
        let symbols = self.repository.search_symbols(&params.query).await?;

        let total_count = symbols.len();
        let symbols = symbols
            .iter()
            .take(params.limit)
            .map(|s| crate::mcp::schema::SymbolDTO {
                id: s.id.clone(),
                name: s.name.clone(),
                category: s.category.clone(),
                description: s.description.clone(),
                related_symbols: s.related_symbols.clone(),
            })
            .collect::<Vec<_>>();

        let content = Content::json(serde_json::json!({
            "symbols": symbols,
            "total_count": total_count
        }));

        let result = CallToolResult::success(vec![content?]);
        Ok(result)
    }

    #[tool(description = "Get symbols by category - use this to filter by category")]
    async fn filter_by_category(
        &self,
        #[tool(aggr)] params: CategorySymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        // This is the recommended method for category filtering
        let symbols = self.repository.list_symbols(Some(&params.category)).await?;

        let total_count = symbols.len();
        let symbols = symbols
            .iter()
            .take(params.limit)
            .map(|s| crate::mcp::schema::SymbolDTO {
                id: s.id.clone(),
                name: s.name.clone(),
                category: s.category.clone(),
                description: s.description.clone(),
                related_symbols: s.related_symbols.clone(),
            })
            .collect::<Vec<_>>();

        let content = Content::json(serde_json::json!({
            "symbols": symbols,
            "total_count": total_count
        }));

        let result = CallToolResult::success(vec![content?]);
        Ok(result)
    }

    #[tool(description = "Get all available symbol categories")]
    async fn get_categories(&self) -> Result<CallToolResult, rmcp::Error> {
        // Get all symbols and extract unique categories
        let symbols = self.repository.list_symbols(None).await?;

        // Extract unique categories using a HashSet
        let mut categories = std::collections::HashSet::new();
        for symbol in &symbols {
            categories.insert(symbol.category.clone());
        }

        // Convert to a sorted Vec
        let mut categories: Vec<String> = categories.into_iter().collect();
        categories.sort();

        let content = Content::json(serde_json::json!({
            "categories": categories,
            "count": categories.len()
        }));

        let result = CallToolResult::success(vec![content?]);
        Ok(result)
    }
}

// Implement ServerHandler trait
#[tool(tool_box)]
impl ServerHandler for SymbolService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "Dream Ontology MCP Server".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some("Get dream symbols from the ontology. For searching symbols, use search_symbols. For filtering by category, use filter_by_category.".to_string()),
        }
    }
}

// Implement error conversion
impl From<crate::domain::RepositoryError> for rmcp::Error {
    fn from(err: crate::domain::RepositoryError) -> Self {
        match err {
            crate::domain::RepositoryError::NotFound(_) => {
                rmcp::Error::resource_not_found("Symbol not found", None)
            }
            crate::domain::RepositoryError::Conflict(_) => {
                rmcp::Error::invalid_request("Symbol already exists", None)
            }
            _ => rmcp::Error::internal_error("Repository error", None),
        }
    }
}
