use rmcp::{ServerHandler, model::*, tool};
use std::sync::Arc;

use crate::domain::{Symbol, SymbolRepository};
use crate::mcp::schema::GetSymbolsParams;

#[derive(Clone)]
pub struct SymbolService {
    repository: Arc<dyn SymbolRepository>,
}

#[tool(tool_box)]
impl SymbolService {
    pub fn new(repository: Arc<dyn SymbolRepository>) -> Self {
        Self { repository }
    }

    #[tool(description = "Get symbols from the ontology with optional filtering")]
    async fn get_symbols(
        &self,
        #[tool(aggr)] params: GetSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        // Reuse your existing logic
        let symbols = match (params.category.as_deref(), params.query.as_deref()) {
            (_, Some(query)) => self.repository.search_symbols(query).await?,
            (Some(category), None) => self.repository.list_symbols(Some(category)).await?,
            (None, None) => self.repository.list_symbols(None).await?,
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

        // Create and return a tool result with the contents unwrapped
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
            instructions: Some("Get dream symbols from the ontology.".to_string()),
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
