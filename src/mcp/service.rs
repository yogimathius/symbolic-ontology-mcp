use log::info;
use rmcp::{ServerHandler, model::*, tool};
use sqlx::PgPool;
use std::fmt::Debug;
use std::sync::Arc;

use crate::db::pool::DbError;
use crate::db::repository::{
    PgRepositoryFactory, RepositoryError, SymbolRepository, SymbolSetRepository,
};
use crate::mcp::schema::{CategorySymbolsParams, GetSymbolsParams, SearchSymbolsParams};

fn pretty_print_result(content: &Content) -> String {
    if let Some(text) = content.as_text() {
        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text.text) {
            if let Ok(pretty) = serde_json::to_string_pretty(&value) {
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

fn pretty_print_params<T: serde::Serialize + Debug>(params: &T) -> String {
    match serde_json::to_string_pretty(params) {
        Ok(pretty) => pretty,
        Err(_) => format!("{:?}", params),
    }
}

#[derive(Clone)]
pub struct SymbolService {
    symbol_repository: Arc<dyn SymbolRepository>,
    symbol_set_repository: Arc<dyn SymbolSetRepository>,
}

#[tool(tool_box)]
impl SymbolService {
    #[allow(dead_code)]
    pub fn new_with_db(db_pool: PgPool) -> Self {
        let factory = PgRepositoryFactory::new(db_pool);
        let symbol_repository = factory.create_symbol_repository();
        let symbol_set_repository = factory.create_symbol_set_repository();

        Self {
            symbol_repository,
            symbol_set_repository,
        }
    }

    #[tool(description = "List all symbols (without filtering)")]
    async fn get_symbols(
        &self,
        #[tool(aggr)] params: GetSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: get_symbols");
        info!("Parameters: {}", pretty_print_params(&params));

        let symbols = self.symbol_repository.list_symbols(None).await?;

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

        info!("Result preview:\n{}", pretty_print_result(&content));

        let result = CallToolResult::success(vec![content]);
        Ok(result)
    }

    #[tool(description = "Search symbols by text query - use this for all text searches")]
    async fn search_symbols(
        &self,
        #[tool(aggr)] params: SearchSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: search_symbols");
        info!("Parameters: {}", pretty_print_params(&params));

        let symbols = self.symbol_repository.search_symbols(&params.query).await?;

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

        info!("Result preview:\n{}", pretty_print_result(&content));

        let result = CallToolResult::success(vec![content]);
        Ok(result)
    }

    #[tool(description = "Get symbols by category - use this to filter by category")]
    async fn filter_by_category(
        &self,
        #[tool(aggr)] params: CategorySymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: filter_by_category");
        info!("Parameters: {}", pretty_print_params(&params));

        let symbols = self
            .symbol_repository
            .list_symbols(Some(&params.category))
            .await?;

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

        info!("Result preview:\n{}", pretty_print_result(&content));

        let result = CallToolResult::success(vec![content]);
        Ok(result)
    }

    #[tool(description = "Get all available symbol categories")]
    async fn get_categories(&self) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: get_categories");

        let symbols = self.symbol_repository.list_symbols(None).await?;

        let mut categories = std::collections::HashSet::new();
        for symbol in &symbols {
            categories.insert(symbol.category.clone());
        }

        let mut categories: Vec<String> = categories.into_iter().collect();
        categories.sort();

        let content = Content::json(serde_json::json!({
            "categories": categories,
            "total_count": categories.len()
        }))?;

        info!("Found {} categories", categories.len());
        info!("Result preview:\n{}", pretty_print_result(&content));

        let result = CallToolResult::success(vec![content]);
        Ok(result)
    }

    #[tool(description = "List symbol sets - collections of related symbols")]
    async fn get_symbol_sets(
        &self,
        #[tool(aggr)] params: GetSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: get_symbol_sets");
        info!("Parameters: {}", pretty_print_params(&params));

        let symbol_sets = self.symbol_set_repository.list_symbol_sets(None).await?;

        let total_count = symbol_sets.len();
        let symbol_sets = symbol_sets
            .iter()
            .take(params.limit)
            .map(|s| {
                serde_json::json!({
                    "id": s.id,
                    "name": s.name,
                    "category": s.category,
                    "description": s.description,
                    "symbol_count": s.symbols.len(),
                })
            })
            .collect::<Vec<_>>();

        let content = Content::json(serde_json::json!({
            "symbol_sets": symbol_sets,
            "total_count": total_count
        }))?;

        info!(
            "Returning {} symbol sets (from total of {})",
            symbol_sets.len(),
            total_count
        );

        info!("Result preview:\n{}", pretty_print_result(&content));

        let result = CallToolResult::success(vec![content]);
        Ok(result)
    }

    #[tool(description = "Search for symbol sets by name or description")]
    async fn search_symbol_sets(
        &self,
        #[tool(aggr)] params: SearchSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: search_symbol_sets");
        info!("Parameters: {}", pretty_print_params(&params));

        let symbol_sets = self
            .symbol_set_repository
            .search_symbol_sets(&params.query)
            .await?;

        let total_count = symbol_sets.len();
        let symbol_sets = symbol_sets
            .iter()
            .take(params.limit)
            .map(|s| {
                serde_json::json!({
                    "id": s.id,
                    "name": s.name,
                    "category": s.category,
                    "description": s.description,
                    "symbol_count": s.symbols.len(),
                })
            })
            .collect::<Vec<_>>();

        let content = Content::json(serde_json::json!({
            "symbol_sets": symbol_sets,
            "total_count": total_count
        }))?;

        info!(
            "Found {} symbol sets matching query '{}' (from total of {})",
            symbol_sets.len(),
            params.query,
            total_count
        );

        info!("Result preview:\n{}", pretty_print_result(&content));

        let result = CallToolResult::success(vec![content]);
        Ok(result)
    }
}

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

impl From<DbError> for rmcp::Error {
    fn from(err: DbError) -> Self {
        match err {
            DbError::NotFound => rmcp::Error::resource_not_found("Symbol not found", None),
            DbError::Conflict(msg) => {
                let error_msg = format!("Conflict: {}", msg);
                rmcp::Error::invalid_request(error_msg, None)
            }
            DbError::Connection(msg) => {
                let error_msg = format!("Database connection error: {}", msg);
                rmcp::Error::internal_error(error_msg, None)
            }
            DbError::Sqlx(e) => {
                let error_msg = format!("Database error: {}", e);
                rmcp::Error::internal_error(error_msg, None)
            }
        }
    }
}

impl From<RepositoryError> for rmcp::Error {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound(msg) => {
                let error_msg = format!("Not found: {}", msg);
                rmcp::Error::resource_not_found(error_msg, None)
            }
            RepositoryError::Conflict(msg) => {
                let error_msg = format!("Conflict: {}", msg);
                rmcp::Error::invalid_request(error_msg, None)
            }
            RepositoryError::Internal(msg) => {
                let error_msg = format!("Internal error: {}", msg);
                rmcp::Error::internal_error(error_msg, None)
            }
            RepositoryError::Validation(msg) => {
                let error_msg = format!("Validation error: {}", msg);
                rmcp::Error::invalid_request(error_msg, None)
            }
            RepositoryError::NotImplemented(msg) => {
                let error_msg = format!("Not implemented: {}", msg);
                rmcp::Error::internal_error(error_msg, None)
            }
        }
    }
}
