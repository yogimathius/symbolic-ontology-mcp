use log::info;
use rmcp::{Error as RmcpError, ServerHandler, model::*, server::MCPServer, tool};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

#[cfg(feature = "local")]
use ontology_core::db::pool::DbError;
#[cfg(feature = "local")]
use ontology_core::db::repository::{
    PgRepositoryFactory, RepositoryError, SymbolRepository, SymbolSetRepository,
};
#[cfg(feature = "local")]
use sqlx::PgPool;

#[cfg(not(feature = "local"))]
use reqwest::Client;

use crate::mcp::schema::{CategorySymbolsParams, GetSymbolsParams, SearchSymbolsParams, SymbolDTO};

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

// API client structure
#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse<T> {
    status: String,
    data: T,
}

// Symbol ontology MCP service supporting both local and remote modes
pub struct SymbolService {
    #[cfg(feature = "local")]
    symbol_repository: Arc<dyn SymbolRepository>,

    #[cfg(not(feature = "local"))]
    client: Client,

    #[cfg(not(feature = "local"))]
    api_endpoint: String,

    #[cfg(not(feature = "local"))]
    api_key: Option<String>,
}

impl SymbolService {
    // Create a new service with local repository
    #[cfg(feature = "local")]
    pub fn new(symbol_repository: Arc<dyn SymbolRepository>) -> Self {
        Self { symbol_repository }
    }

    // Create a new service with HTTP client for remote API
    #[cfg(not(feature = "local"))]
    pub fn new_with_client(client: Client, api_endpoint: String, api_key: Option<String>) -> Self {
        Self {
            client,
            api_endpoint,
            api_key,
        }
    }
}

impl ServerHandler for SymbolService {
    fn schema(&self) -> Schema {
        let mut schema = Schema::default();

        // Get symbols method
        schema.add_method(
            MethodDef::new("get_symbols")
                .description("Get a list of symbols from the ontology")
                .param::<GetSymbolsParams>()
                .return_type(),
        );

        // Search symbols method
        schema.add_method(
            MethodDef::new("search_symbols")
                .description("Search for symbols by text query")
                .param::<SearchSymbolsParams>()
                .return_type(),
        );

        // Filter by category method
        schema.add_method(
            MethodDef::new("filter_by_category")
                .description("Filter symbols by category")
                .param::<CategorySymbolsParams>()
                .return_type(),
        );

        schema
    }

    #[cfg(feature = "local")]
    async fn handle_call(&self, call: Call) -> Result<Content, RmcpError> {
        info!("Handling MCP call: {} ({})", call.method, call.id);

        // Log parameters (if not sensitive)
        if !call.method.contains("key") && !call.method.contains("password") {
            info!("Parameters: {}", pretty_print_result(&call.params));
        }

        // Handle the method
        let result = match call.method.as_str() {
            "get_symbols" => {
                let params: GetSymbolsParams = match serde_json::from_value(
                    call.params.as_object().unwrap_or_default().clone(),
                ) {
                    Ok(p) => p,
                    Err(e) => {
                        info!("Error parsing parameters: {}", e);
                        return Err(RmcpError::invalid_params());
                    }
                };

                info!("Getting all symbols (limit: {})", params.limit);
                let symbols = match self.symbol_repository.get_all(params.limit).await {
                    Ok(s) => s,
                    Err(e) => {
                        info!("Repository error: {}", e);
                        return Err(RmcpError::internal_error());
                    }
                };

                info!("Found {} symbols", symbols.len());

                // Convert to DTOs
                let dtos: Vec<SymbolDTO> = symbols
                    .iter()
                    .map(|s| SymbolDTO {
                        id: s.id.clone(),
                        name: s.name.clone(),
                        category: s.category.clone(),
                        description: s.description.clone(),
                        related_symbols: s.related_symbols.clone(),
                    })
                    .collect();

                serde_json::to_value(crate::mcp::schema::GetSymbolsResponse {
                    symbols: dtos,
                    total_count: symbols.len(),
                })
                .unwrap_or_default()
            }
            "search_symbols" => {
                let params: SearchSymbolsParams = match serde_json::from_value(
                    call.params.as_object().unwrap_or_default().clone(),
                ) {
                    Ok(p) => p,
                    Err(e) => {
                        info!("Error parsing parameters: {}", e);
                        return Err(RmcpError::invalid_params());
                    }
                };

                info!(
                    "Searching symbols with query: {} (limit: {})",
                    params.query, params.limit
                );
                let symbols = match self.symbol_repository.search_symbols(&params.query).await {
                    Ok(s) => s,
                    Err(e) => {
                        info!("Repository error: {}", e);
                        return Err(RmcpError::internal_error());
                    }
                };

                info!("Found {} matching symbols", symbols.len());

                // Convert to DTOs
                let dtos: Vec<SymbolDTO> = symbols
                    .iter()
                    .take(params.limit)
                    .map(|s| SymbolDTO {
                        id: s.id.clone(),
                        name: s.name.clone(),
                        category: s.category.clone(),
                        description: s.description.clone(),
                        related_symbols: s.related_symbols.clone(),
                    })
                    .collect();

                serde_json::to_value(crate::mcp::schema::GetSymbolsResponse {
                    symbols: dtos,
                    total_count: dtos.len(),
                })
                .unwrap_or_default()
            }
            "filter_by_category" => {
                let params: CategorySymbolsParams = match serde_json::from_value(
                    call.params.as_object().unwrap_or_default().clone(),
                ) {
                    Ok(p) => p,
                    Err(e) => {
                        info!("Error parsing parameters: {}", e);
                        return Err(RmcpError::invalid_params());
                    }
                };

                info!(
                    "Filtering symbols by category: {} (limit: {})",
                    params.category, params.limit
                );
                let symbols = match self
                    .symbol_repository
                    .filter_by_category(&params.category)
                    .await
                {
                    Ok(s) => s,
                    Err(e) => {
                        info!("Repository error: {}", e);
                        return Err(RmcpError::internal_error());
                    }
                };

                info!(
                    "Found {} symbols in category {}",
                    symbols.len(),
                    params.category
                );

                // Convert to DTOs
                let dtos: Vec<SymbolDTO> = symbols
                    .iter()
                    .take(params.limit)
                    .map(|s| SymbolDTO {
                        id: s.id.clone(),
                        name: s.name.clone(),
                        category: s.category.clone(),
                        description: s.description.clone(),
                        related_symbols: s.related_symbols.clone(),
                    })
                    .collect();

                serde_json::to_value(crate::mcp::schema::GetSymbolsResponse {
                    symbols: dtos,
                    total_count: dtos.len(),
                })
                .unwrap_or_default()
            }
            _ => {
                info!("Method not supported: {}", call.method);
                return Err(RmcpError::method_not_found());
            }
        };

        // Log result (if not too verbose)
        info!("Result for {}: (result omitted for brevity)", call.method);

        Ok(Content::from_json(result))
    }

    #[cfg(not(feature = "local"))]
    async fn handle_call(&self, call: Call) -> Result<Content, RmcpError> {
        info!("Forwarding MCP call to API: {} ({})", call.method, call.id);

        // Build the API endpoint
        let endpoint = format!("{}/api/v1/{}", self.api_endpoint, call.method);

        // Create request
        let mut request = self
            .client
            .post(&endpoint)
            .json(&call.params.as_object().unwrap_or_default());

        // Add API key if provided
        if let Some(api_key) = &self.api_key {
            request = request.header("X-API-Key", api_key);
        }

        // Send request
        let response = match request.send().await {
            Ok(r) => r,
            Err(e) => {
                info!("API request error: {}", e);
                return Err(RmcpError::internal_error());
            }
        };

        // Check status
        if !response.status().is_success() {
            info!("API error: {}", response.status());
            return Err(RmcpError::internal_error());
        }

        // Parse response
        let json = match response.json::<serde_json::Value>().await {
            Ok(j) => j,
            Err(e) => {
                info!("Error parsing API response: {}", e);
                return Err(RmcpError::internal_error());
            }
        };

        Ok(Content::from_json(json))
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

impl From<DbError> for RmcpError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::NotFound => RmcpError::resource_not_found("Symbol not found", None),
            DbError::Conflict(msg) => {
                let error_msg = format!("Conflict: {}", msg);
                RmcpError::invalid_request(error_msg, None)
            }
            DbError::Connection(msg) => {
                let error_msg = format!("Database connection error: {}", msg);
                RmcpError::internal_error(error_msg, None)
            }
            DbError::Sqlx(e) => {
                let error_msg = format!("Database error: {}", e);
                RmcpError::internal_error(error_msg, None)
            }
        }
    }
}

impl From<RepositoryError> for RmcpError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound(msg) => {
                let error_msg = format!("Not found: {}", msg);
                RmcpError::resource_not_found(error_msg, None)
            }
            RepositoryError::Conflict(msg) => {
                let error_msg = format!("Conflict: {}", msg);
                RmcpError::invalid_request(error_msg, None)
            }
            RepositoryError::Internal(msg) => {
                let error_msg = format!("Internal error: {}", msg);
                RmcpError::internal_error(error_msg, None)
            }
            RepositoryError::Validation(msg) => {
                let error_msg = format!("Validation error: {}", msg);
                RmcpError::invalid_request(error_msg, None)
            }
            RepositoryError::NotImplemented(msg) => {
                let error_msg = format!("Not implemented: {}", msg);
                RmcpError::internal_error(error_msg, None)
            }
        }
    }
}
