use log::info;
use rmcp::{ServerHandler, model::*, tool};
use std::fmt::Debug;
use std::sync::Arc;

use crate::domain::SymbolRepository;
use crate::mcp::schema::{CategorySymbolsParams, GetSymbolsParams, SearchSymbolsParams};

/// Helper function to pretty-print the JSON result for logging
fn pretty_print_result(content: &Content) -> String {
    if let Some(text) = content.as_text() {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text.text) {
            if let Ok(pretty) = serde_json::to_string_pretty(&value) {
                // Return a shortened version with just a few symbols
                if pretty.len() > 500 {
                    let mut shortened = pretty[..400].to_string();
                    shortened.push_str("\n... (truncated) ...\n}");
                    return shortened;
                }
                return pretty;
            }
        }
        return text.text.clone();
    }
    format!("{:?}", content)
}

/// Helper function to pretty-print request parameters
fn pretty_print_params<T: serde::Serialize + Debug>(params: &T) -> String {
    match serde_json::to_string_pretty(params) {
        Ok(pretty) => pretty,
        Err(_) => format!("{:?}", params),
    }
}

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
        // Log request parameters in a cleaner format
        info!("Tool call: get_symbols");
        info!("Parameters: {}", pretty_print_params(&params));

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
        }))?;

        info!(
            "Returning {} symbols (from total of {})",
            symbols.len(),
            total_count
        );

        // Log the prettified content
        info!("Result preview:\n{}", pretty_print_result(&content));

        let result = CallToolResult::success(vec![content]);
        Ok(result)
    }

    #[tool(description = "Search symbols by text query - use this for all text searches")]
    async fn search_symbols(
        &self,
        #[tool(aggr)] params: SearchSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        // Log request parameters in a cleaner format
        info!("Tool call: search_symbols");
        info!("Parameters: {}", pretty_print_params(&params));

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
        }))?;

        info!(
            "Found {} symbols matching query '{}' (from total of {})",
            symbols.len(),
            params.query,
            total_count
        );

        // Log the prettified content
        info!("Result preview:\n{}", pretty_print_result(&content));

        let result = CallToolResult::success(vec![content]);
        Ok(result)
    }

    #[tool(description = "Get symbols by category - use this to filter by category")]
    async fn filter_by_category(
        &self,
        #[tool(aggr)] params: CategorySymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        // Log request parameters in a cleaner format
        info!("Tool call: filter_by_category");
        info!("Parameters: {}", pretty_print_params(&params));

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
        }))?;

        info!(
            "Found {} symbols in category '{}' (from total of {})",
            symbols.len(),
            params.category,
            total_count
        );

        // Log the prettified content
        info!("Result preview:\n{}", pretty_print_result(&content));

        let result = CallToolResult::success(vec![content]);
        Ok(result)
    }

    #[tool(description = "Get all available symbol categories")]
    async fn get_categories(&self) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: get_categories");

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
        }))?;

        info!("Found {} categories", categories.len());

        // Log the prettified content
        info!("Result preview:\n{}", pretty_print_result(&content));

        let result = CallToolResult::success(vec![content]);
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
