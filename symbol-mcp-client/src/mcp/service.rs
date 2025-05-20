use rmcp::model::*;
use rmcp::service::RequestContext;
use rmcp::tool;
use rmcp::RoleServer;
use rmcp::ServerHandler;
use std::sync::Arc;
use tracing::{debug, error, info};

#[cfg(feature = "local")]
use crate::mcp::test_utils::{InMemorySymbolRepository, InMemorySymbolSetRepository};
use ontology_core::db::repository::{SymbolRepository, SymbolSetRepository};

use crate::mcp::methods::{
    filter_by_category::filter_by_category,
    get_symbols::{get_symbols, Handler, MethodCall, RmcpError},
    search_symbols::search_symbols,
};
use crate::mcp::schema::{CategorySymbolsParams, GetSymbolsParams, SearchSymbolsParams};

// Helper function for pretty-printing results
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

// Symbol Service for MCP with direct database access
#[derive(Clone)]
pub struct SymbolService {
    pub symbol_repository: Arc<dyn SymbolRepository>,
    pub symbol_set_repository: Arc<dyn SymbolSetRepository>,
}

impl SymbolService {
    /// Creates a new SymbolService with in-memory repositories for testing
    #[cfg(feature = "local")]
    pub fn new() -> Self {
        Self {
            symbol_repository: Arc::new(InMemorySymbolRepository::new()),
            symbol_set_repository: Arc::new(InMemorySymbolSetRepository::new()),
        }
    }
}

#[tool(tool_box)]
impl SymbolService {
    #[tool(description = "List all symbols (without filtering)")]
    async fn get_symbols(
        &self,
        #[tool(aggr)] params: GetSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: get_symbols");

        // Convert params to MethodCall for our handler
        let handler = get_symbols(Arc::clone(&self.symbol_repository));
        let method_call = MethodCall {
            id: "get_symbols".to_string(),
            method: "get_symbols".to_string(),
            params: serde_json::to_value(params).unwrap_or(serde_json::Value::Null),
        };

        // Process using our handler
        match handler.handle(method_call).await {
            Ok(json_result) => match Content::json(json_result) {
                Ok(content) => {
                    info!("Result preview:\n{}", pretty_print_result(&content));
                    Ok(CallToolResult::success(vec![content]))
                }
                Err(e) => {
                    error!("Failed to create content: {}", e);
                    Err(e)
                }
            },
            Err(e) => {
                error!("Error in get_symbols: {}", e);
                match e {
                    RmcpError::ParseError(msg) => Err(rmcp::Error::invalid_params(msg, None)),
                    RmcpError::RepositoryError(msg) => Err(rmcp::Error::internal_error(msg, None)),
                    RmcpError::Other(msg) => Err(rmcp::Error::internal_error(msg, None)),
                }
            }
        }
    }

    #[tool(description = "Search symbols by text query - use this for all text searches")]
    async fn search_symbols(
        &self,
        #[tool(aggr)] params: SearchSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: search_symbols");

        // Convert params to MethodCall for our handler
        let handler = search_symbols(Arc::clone(&self.symbol_repository));
        let method_call = MethodCall {
            id: "search_symbols".to_string(),
            method: "search_symbols".to_string(),
            params: serde_json::to_value(params).unwrap_or(serde_json::Value::Null),
        };

        // Process using our handler
        match handler.handle(method_call).await {
            Ok(json_result) => match Content::json(json_result) {
                Ok(content) => {
                    info!("Result preview:\n{}", pretty_print_result(&content));
                    Ok(CallToolResult::success(vec![content]))
                }
                Err(e) => {
                    error!("Failed to create content: {}", e);
                    Err(e)
                }
            },
            Err(e) => {
                error!("Error in search_symbols: {}", e);
                match e {
                    RmcpError::ParseError(msg) => Err(rmcp::Error::invalid_params(msg, None)),
                    RmcpError::RepositoryError(msg) => Err(rmcp::Error::internal_error(msg, None)),
                    RmcpError::Other(msg) => Err(rmcp::Error::internal_error(msg, None)),
                }
            }
        }
    }

    #[tool(description = "Get symbols by category - use this to filter by category")]
    async fn filter_by_category(
        &self,
        #[tool(aggr)] params: CategorySymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: filter_by_category");

        // Convert params to MethodCall for our handler
        let handler = filter_by_category(Arc::clone(&self.symbol_repository));
        let method_call = MethodCall {
            id: "filter_by_category".to_string(),
            method: "filter_by_category".to_string(),
            params: serde_json::to_value(params).unwrap_or(serde_json::Value::Null),
        };

        // Process using our handler
        match handler.handle(method_call).await {
            Ok(json_result) => match Content::json(json_result) {
                Ok(content) => {
                    info!("Result preview:\n{}", pretty_print_result(&content));
                    Ok(CallToolResult::success(vec![content]))
                }
                Err(e) => {
                    error!("Failed to create content: {}", e);
                    Err(e)
                }
            },
            Err(e) => {
                error!("Error in filter_by_category: {}", e);
                match e {
                    RmcpError::ParseError(msg) => Err(rmcp::Error::invalid_params(msg, None)),
                    RmcpError::RepositoryError(msg) => Err(rmcp::Error::internal_error(msg, None)),
                    RmcpError::Other(msg) => Err(rmcp::Error::internal_error(msg, None)),
                }
            }
        }
    }

    #[tool(description = "Get a list of symbol sets available in the ontology")]
    async fn get_symbol_sets(&self) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: get_symbol_sets");

        match self.symbol_set_repository.list_symbol_sets(None).await {
            Ok(symbol_sets) => {
                let result = serde_json::json!({
                    "symbol_sets": symbol_sets.iter().map(|set| {
                        serde_json::json!({
                            "id": set.id,
                            "name": set.name,
                            "category": set.category,
                            "description": set.description,
                            "symbol_count": set.symbols.len()
                        })
                    }).collect::<Vec<_>>(),
                    "total_count": symbol_sets.len()
                });

                match Content::json(result) {
                    Ok(content) => {
                        info!("Result preview:\n{}", pretty_print_result(&content));
                        Ok(CallToolResult::success(vec![content]))
                    }
                    Err(e) => {
                        error!("Failed to create content: {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Error fetching symbol sets: {}", e);
                Err(rmcp::Error::internal_error(
                    format!("Repository error: {}", e),
                    None,
                ))
            }
        }
    }

    #[tool(description = "Get all available symbol categories")]
    async fn get_categories(&self) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: get_categories");

        match self.symbol_repository.list_symbols(None).await {
            Ok(symbols) => {
                let mut categories = std::collections::HashSet::new();
                for symbol in &symbols {
                    categories.insert(symbol.category.clone());
                }

                let mut categories: Vec<String> = categories.into_iter().collect();
                categories.sort();

                let result = serde_json::json!({
                    "categories": categories,
                    "count": categories.len()
                });

                match Content::json(result) {
                    Ok(content) => {
                        info!("Result preview:\n{}", pretty_print_result(&content));
                        Ok(CallToolResult::success(vec![content]))
                    }
                    Err(e) => {
                        error!("Failed to create content: {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Error fetching categories: {}", e);
                Err(rmcp::Error::internal_error(
                    format!("Repository error: {}", e),
                    None,
                ))
            }
        }
    }

    #[tool(description = "Search for symbol sets by name or description")]
    async fn search_symbol_sets(
        &self,
        #[tool(aggr)] params: SearchSymbolsParams,
    ) -> Result<CallToolResult, rmcp::Error> {
        info!("Tool call: search_symbol_sets");

        match self
            .symbol_set_repository
            .search_symbol_sets(&params.query)
            .await
        {
            Ok(symbol_sets) => {
                let result = serde_json::json!({
                    "symbol_sets": symbol_sets.iter().take(params.limit).map(|set| {
                        serde_json::json!({
                            "id": set.id,
                            "name": set.name,
                            "category": set.category,
                            "description": set.description,
                            "symbol_count": set.symbols.len()
                        })
                    }).collect::<Vec<_>>(),
                    "total_count": symbol_sets.len()
                });

                match Content::json(result) {
                    Ok(content) => {
                        info!("Result preview:\n{}", pretty_print_result(&content));
                        Ok(CallToolResult::success(vec![content]))
                    }
                    Err(e) => {
                        error!("Failed to create content: {}", e);
                        Err(e)
                    }
                }
            }
            Err(e) => {
                error!("Error searching symbol sets: {}", e);
                Err(rmcp::Error::internal_error(
                    format!("Repository error: {}", e),
                    None,
                ))
            }
        }
    }
}

// ServerHandler implementation with proper MCP method dispatching
impl ServerHandler for SymbolService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "Symbol Ontology MCP Client".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            instructions: Some("Symbol Ontology provides symbolic reasoning tools. Available methods: get_symbols, search_symbols, filter_by_category, get_categories, get_symbol_sets, search_symbol_sets.".to_string()),
        }
    }

    async fn call_tool(
        &self,
        method: CallToolRequestParam,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, rmcp::Error> {
        let method_name = &method.name;
        debug!("MCP tool call received: {}", method_name);

        let arguments = match method.arguments {
            Some(ref map) => serde_json::Value::Object(map.clone()),
            None => serde_json::Value::Null,
        };

        let method_call = crate::mcp::methods::get_symbols::MethodCall {
            id: "mcp-call".to_string(), // ID is not really used in our implementation
            method: method_name.to_string(),
            params: arguments,
        };

        // Create appropriate handlers based on method
        let result = match method_name.as_ref() {
            "get_symbols" => {
                let handler = get_symbols(Arc::clone(&self.symbol_repository));
                handler.handle(method_call).await
            }
            "search_symbols" => {
                let handler = search_symbols(Arc::clone(&self.symbol_repository));
                handler.handle(method_call).await
            }
            "filter_by_category" => {
                let handler = filter_by_category(Arc::clone(&self.symbol_repository));
                handler.handle(method_call).await
            }
            "get_categories" => {
                // Get a list of all unique categories
                match self.symbol_repository.list_symbols(None).await {
                    Ok(symbols) => {
                        let mut categories = std::collections::HashSet::new();
                        for symbol in &symbols {
                            categories.insert(symbol.category.clone());
                        }

                        let mut categories: Vec<String> = categories.into_iter().collect();
                        categories.sort();

                        Ok(serde_json::json!({
                            "categories": categories,
                            "count": categories.len()
                        }))
                    }
                    Err(e) => Err(RmcpError::RepositoryError(e.to_string())),
                }
            }
            "get_symbol_sets" => {
                // Return a list of all symbol sets
                match self.symbol_set_repository.list_symbol_sets(None).await {
                    Ok(symbol_sets) => Ok(serde_json::json!({
                        "symbol_sets": symbol_sets.iter().map(|set| {
                            serde_json::json!({
                                "id": set.id,
                                "name": set.name,
                                "category": set.category,
                                "description": set.description,
                                "symbol_count": set.symbols.len()
                            })
                        }).collect::<Vec<_>>(),
                        "total_count": symbol_sets.len()
                    })),
                    Err(e) => Err(RmcpError::RepositoryError(e.to_string())),
                }
            }
            "search_symbol_sets" => {
                // Parse the query parameter from method_call.params
                let params: SearchSymbolsParams = match serde_json::from_value(method_call.params) {
                    Ok(params) => params,
                    Err(e) => return Err(rmcp::Error::invalid_params(e.to_string(), None)),
                };

                match self
                    .symbol_set_repository
                    .search_symbol_sets(&params.query)
                    .await
                {
                    Ok(symbol_sets) => Ok(serde_json::json!({
                        "symbol_sets": symbol_sets.iter().take(params.limit).map(|set| {
                            serde_json::json!({
                                "id": set.id,
                                "name": set.name,
                                "category": set.category,
                                "description": set.description,
                                "symbol_count": set.symbols.len()
                            })
                        }).collect::<Vec<_>>(),
                        "total_count": symbol_sets.len()
                    })),
                    Err(e) => Err(RmcpError::RepositoryError(e.to_string())),
                }
            }
            _ => Err(RmcpError::Other(format!("Unknown method: {}", method_name))),
        };

        // Process the result, or return an error
        match result {
            Ok(json_result) => match Content::json(json_result) {
                Ok(content) => Ok(CallToolResult::success(vec![content])),
                Err(e) => Err(e),
            },
            Err(e) => match e {
                RmcpError::ParseError(msg) => Err(rmcp::Error::invalid_params(msg, None)),
                RmcpError::RepositoryError(msg) => Err(rmcp::Error::internal_error(msg, None)),
                RmcpError::Other(msg) => Err(rmcp::Error::internal_error(msg, None)),
            },
        }
    }

    fn list_tools(
        &self,
        _param: Option<PaginatedRequestParamInner>,
        _ctx: RequestContext<RoleServer>,
    ) -> impl std::future::Future<Output = Result<ListToolsResult, rmcp::Error>> + Send + '_ {
        async move {
            // Schema for get_symbols
            let schema1 = serde_json::json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of symbols to return",
                        "default": 50
                    }
                }
            });

            // Schema for search_symbols
            let schema2 = serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search text"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of symbols to return",
                        "default": 50
                    }
                },
                "required": ["query"]
            });

            // Schema for filter_by_category
            let schema3 = serde_json::json!({
                "type": "object",
                "properties": {
                    "category": {
                        "type": "string",
                        "description": "Category name to filter by"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of symbols to return",
                        "default": 50
                    }
                },
                "required": ["category"]
            });

            // Schema for get_categories (no parameters needed)
            let schema4 = serde_json::json!({
                "type": "object",
                "properties": {}
            });

            // Schema for get_symbol_sets
            let schema5 = serde_json::json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of symbol sets to return",
                        "default": 50
                    }
                }
            });

            // Schema for search_symbol_sets
            let schema6 = serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "Search text for symbol sets"
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum number of symbol sets to return",
                        "default": 50
                    }
                },
                "required": ["query"]
            });

            // Convert schemas to Arc<Map<String, Value>> as expected by rmcp
            let schema1_map = match serde_json::to_value(schema1) {
                Ok(serde_json::Value::Object(map)) => Arc::new(map),
                _ => {
                    return Err(rmcp::Error::internal_error(
                        "Failed to create schema1",
                        None,
                    ))
                }
            };

            let schema2_map = match serde_json::to_value(schema2) {
                Ok(serde_json::Value::Object(map)) => Arc::new(map),
                _ => {
                    return Err(rmcp::Error::internal_error(
                        "Failed to create schema2",
                        None,
                    ))
                }
            };

            let schema3_map = match serde_json::to_value(schema3) {
                Ok(serde_json::Value::Object(map)) => Arc::new(map),
                _ => {
                    return Err(rmcp::Error::internal_error(
                        "Failed to create schema3",
                        None,
                    ))
                }
            };

            let schema4_map = match serde_json::to_value(schema4) {
                Ok(serde_json::Value::Object(map)) => Arc::new(map),
                _ => {
                    return Err(rmcp::Error::internal_error(
                        "Failed to create schema4",
                        None,
                    ))
                }
            };

            let schema5_map = match serde_json::to_value(schema5) {
                Ok(serde_json::Value::Object(map)) => Arc::new(map),
                _ => {
                    return Err(rmcp::Error::internal_error(
                        "Failed to create schema5",
                        None,
                    ))
                }
            };

            let schema6_map = match serde_json::to_value(schema6) {
                Ok(serde_json::Value::Object(map)) => Arc::new(map),
                _ => {
                    return Err(rmcp::Error::internal_error(
                        "Failed to create schema6",
                        None,
                    ))
                }
            };

            Ok(ListToolsResult {
                tools: vec![
                    Tool {
                        name: "get_symbols".into(),
                        input_schema: schema1_map,
                        description: "List all symbols (with optional limit)".into(),
                    },
                    Tool {
                        name: "search_symbols".into(),
                        input_schema: schema2_map,
                        description:
                            "Search symbols by text query - use this for all text searches".into(),
                    },
                    Tool {
                        name: "filter_by_category".into(),
                        input_schema: schema3_map,
                        description: "Get symbols by category - use this to filter by category"
                            .into(),
                    },
                    Tool {
                        name: "get_categories".into(),
                        input_schema: schema4_map,
                        description: "Get all available symbol categories".into(),
                    },
                    Tool {
                        name: "get_symbol_sets".into(),
                        input_schema: schema5_map,
                        description: "List symbol sets - collections of related symbols".into(),
                    },
                    Tool {
                        name: "search_symbol_sets".into(),
                        input_schema: schema6_map,
                        description: "Search for symbol sets by name or description".into(),
                    },
                ],
                next_cursor: None,
            })
        }
    }
}
