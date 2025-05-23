use async_trait::async_trait;
use serde_json;
use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

use ontology_core::db::repository::{RepositoryError, SymbolRepository};
use ontology_core::domain::Symbol;

use crate::mcp::schema::{GetSymbolsParams, GetSymbolsResponse, SymbolDTO};

use super::utils::repository_error_to_rmcp_error;

/// Handler trait definition
#[async_trait]
pub trait Handler: Send + Sync {
    fn method_name(&self) -> &str;

    async fn handle(&self, call: MethodCall) -> Result<serde_json::Value, RmcpError>;
}

/// MethodCall structure
pub struct MethodCall {
    pub id: String,
    pub method: String,
    pub params: serde_json::Value,
}

impl MethodCall {
    pub fn parse_params<T: serde::de::DeserializeOwned>(&self) -> Result<T, RmcpError> {
        serde_json::from_value(self.params.clone())
            .map_err(|e| RmcpError::ParseError(e.to_string()))
    }
}

/// Error codes for MCP protocol
///
/// These error codes follow the JSON-RPC 2.0 specification, with additional MCP-specific error codes.
///
/// References:
/// - JSON-RPC 2.0 Specification: https://www.jsonrpc.org/specification#error_object
/// - MCP Specification: https://modelcontextprotocol.io
/// - RMCP (Rust MCP) SDK: https://github.com/4t145/rmcp
///
/// JSON-RPC 2.0 specifies error codes in these ranges:
/// - -32700 to -32600: Reserved for pre-defined errors
/// - -32000 to -32099: Reserved for implementation-defined server errors
/// - Client-defined codes may be used for custom errors (not used here)
#[derive(Debug, Clone, Copy)]
pub enum RmcpErrorCode {
    /// -32600: Invalid request - The JSON sent is not a valid Request object
    /// as defined in the JSON-RPC 2.0 specification
    InvalidRequest,

    /// -32601: Method not found - The method does not exist / is not available
    MethodNotFound,

    /// -32602: Invalid params - Invalid method parameter(s)
    InvalidParams,

    /// -32603: Internal error - Internal JSON-RPC error
    InternalError,

    /// -32000: Server error - Generic server-side error
    /// This is an implementation-defined server error
    ServerError,

    /// -32001: Not found - Requested resource not found
    /// This is an MCP-specific error code
    NotFound,

    /// -32002: Conflict - Resource conflict (e.g., duplicate ID)
    /// This is an MCP-specific error code
    Conflict,
}

impl RmcpErrorCode {
    /// Get the numeric code for this error
    pub fn code(&self) -> i32 {
        match self {
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::InvalidParams => -32602,
            Self::InternalError => -32603,
            Self::ServerError => -32000,
            Self::NotFound => -32001,
            Self::Conflict => -32002,
        }
    }

    /// Get a string representation of this error code
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::MethodNotFound => "Method not found",
            Self::InvalidParams => "Invalid params",
            Self::InternalError => "Internal error",
            Self::ServerError => "Server error",
            Self::NotFound => "Not found",
            Self::Conflict => "Conflict",
        }
    }
}

/// Enhanced MCP error type
#[derive(Debug)]
pub enum RmcpError {
    ParseError(String),
    RepositoryError(String),
    #[allow(dead_code)]
    Other(String),
}

impl fmt::Display for RmcpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::RepositoryError(msg) => write!(f, "Repository error: {}", msg),
            Self::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl StdError for RmcpError {}

impl RmcpError {
    /// Get the error code for this error
    pub fn error_code(&self) -> RmcpErrorCode {
        match self {
            Self::ParseError(_) => RmcpErrorCode::InvalidParams,
            Self::RepositoryError(_) => RmcpErrorCode::ServerError,
            Self::Other(_) => RmcpErrorCode::InternalError,
        }
    }

    /// Convert to a JSON-RPC error response
    ///
    /// This creates a standard JSON-RPC 2.0 error response following the specification:
    /// https://www.jsonrpc.org/specification#error_object
    pub fn to_jsonrpc_error(&self, id: &str) -> serde_json::Value {
        let code = self.error_code();
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {
                "code": code.code(),
                "message": code.as_str(),
                "data": self.to_string()
            }
        })
    }
}

impl From<serde_json::Error> for RmcpError {
    fn from(err: serde_json::Error) -> Self {
        RmcpError::ParseError(err.to_string())
    }
}

impl From<RepositoryError> for RmcpError {
    fn from(err: RepositoryError) -> Self {
        repository_error_to_rmcp_error(err)
    }
}

/// MCP handler for get_symbols method
pub struct GetSymbolsHandler {
    symbol_repository: Arc<dyn SymbolRepository>,
}

impl GetSymbolsHandler {
    pub fn new(symbol_repository: Arc<dyn SymbolRepository>) -> Self {
        GetSymbolsHandler { symbol_repository }
    }

    fn to_dto(symbol: &Symbol) -> SymbolDTO {
        SymbolDTO {
            id: symbol.id.clone(),
            name: symbol.name.clone(),
            category: symbol.category.clone(),
            description: symbol.description.to_string().clone(),
            related_symbols: symbol.related_symbols.clone(),
        }
    }
}

#[async_trait]
impl Handler for GetSymbolsHandler {
    fn method_name(&self) -> &str {
        "get_symbols"
    }

    async fn handle(&self, call: MethodCall) -> Result<serde_json::Value, RmcpError> {
        // Parse parameters
        let params: GetSymbolsParams = call.parse_params()?;

        // Validate category if provided
        if let Some(cat) = &params.category {
            if cat.trim().is_empty() {
                return Err(RmcpError::ParseError(
                    "Category cannot be empty".to_string(),
                ));
            }
        }

        // Fetch symbols with optional category filter
        let symbols = self
            .symbol_repository
            .list_symbols(params.category.as_deref())
            .await
            .map_err(repository_error_to_rmcp_error)?;

        // Apply limit and convert to DTOs
        let symbol_dtos = symbols
            .iter()
            .take(params.limit)
            .map(Self::to_dto)
            .collect::<Vec<_>>();

        let total_count = symbols.len();

        // Create and return response
        Ok(serde_json::to_value(GetSymbolsResponse {
            symbols: symbol_dtos,
            total_count,
        })?)
    }
}

pub fn get_symbols(symbol_repository: Arc<dyn SymbolRepository>) -> GetSymbolsHandler {
    GetSymbolsHandler::new(symbol_repository)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[cfg(feature = "local")]
    use crate::mcp::test_utils::InMemorySymbolRepository;

    #[cfg(feature = "local")]
    use std::collections::HashMap;

    #[cfg(feature = "local")]
    #[tokio::test]
    async fn test_get_symbols_handler() {
        // TODO: Create mock repository and test the handler
    }

    #[cfg(feature = "local")]
    #[test]
    fn test_method_name() {
        let handler = get_symbols(Arc::new(InMemorySymbolRepository::new()));
        assert_eq!(handler.method_name(), "get_symbols");
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(RmcpErrorCode::InvalidRequest.code(), -32600);
        assert_eq!(RmcpErrorCode::MethodNotFound.code(), -32601);
        assert_eq!(RmcpErrorCode::InvalidParams.code(), -32602);
    }
}
