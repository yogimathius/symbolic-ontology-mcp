use async_trait::async_trait;
use serde_json;
use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

use crate::domain::{RepositoryError, Symbol, SymbolRepository};
use crate::mcp::schema::{GetSymbolsParams, GetSymbolsResponse, SymbolDTO};

/// Handler trait definition
#[async_trait]
#[allow(dead_code)]
pub trait Handler: Send + Sync {
    fn method_name(&self) -> &str;

    async fn handle(&self, call: MethodCall) -> Result<serde_json::Value, RmcpError>;
}

/// MethodCall structure
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
    /// Parse error when deserializing request
    ParseError(String),
    /// Error from the repository layer
    RepositoryError(String),
    /// Other errors
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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
        match err {
            RepositoryError::NotFound(msg) => {
                RmcpError::RepositoryError(format!("Not found: {}", msg))
            }
            RepositoryError::Conflict(msg) => {
                RmcpError::RepositoryError(format!("Conflict: {}", msg))
            }
            RepositoryError::Internal(msg) => {
                RmcpError::RepositoryError(format!("Internal: {}", msg))
            }
            RepositoryError::Validation(msg) => {
                RmcpError::RepositoryError(format!("Validation: {}", msg))
            }
        }
    }
}

/// MCP handler for get_symbols method
pub struct GetSymbolsHandler {
    // Repository for fetching symbols
    symbol_repository: Arc<dyn SymbolRepository>,
}

impl GetSymbolsHandler {
    /// Create a new handler with the provided repository
    #[allow(dead_code)]
    pub fn new(symbol_repository: Arc<dyn SymbolRepository>) -> Self {
        GetSymbolsHandler { symbol_repository }
    }

    /// Convert a domain Symbol to a DTO for the API
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
impl Handler for GetSymbolsHandler {
    /// The method name this handler responds to
    fn method_name(&self) -> &str {
        "get_symbols"
    }

    /// Handle the MCP method call
    async fn handle(&self, call: MethodCall) -> Result<serde_json::Value, RmcpError> {
        let params: GetSymbolsParams = call.parse_params()?;

        // Fetch symbols from the repository
        let symbols = match (params.category.as_deref(), params.query.as_deref()) {
            // If we have a query, search for it
            (_, Some(query)) => self.symbol_repository.search_symbols(query).await?,
            // If we have just a category, filter by it
            (Some(category), None) => self.symbol_repository.list_symbols(Some(category)).await?,
            // No filters, list all symbols
            (None, None) => self.symbol_repository.list_symbols(None).await?,
        };

        // Apply limit
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

/// Factory function to create the get_symbols handler with provided repository
#[allow(dead_code)]
pub fn get_symbols(symbol_repository: Arc<dyn SymbolRepository>) -> GetSymbolsHandler {
    GetSymbolsHandler::new(symbol_repository)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        domain::RepositoryFactory, infrastructure::memory_repository::MemoryRepositoryFactory,
    };
    use serde_json::json;

    #[tokio::test]
    async fn test_get_symbols_handler() {
        // Create a real repository with test data
        let factory = MemoryRepositoryFactory::new().with_test_data();
        let repository = factory.create_symbol_repository();

        let handler = GetSymbolsHandler::new(repository);

        // Create a method call with empty params
        let call = MethodCall {
            id: "test-call".to_string(),
            method: "get_symbols".to_string(),
            params: json!({}),
        };

        let result = handler.handle(call).await;

        assert!(result.is_ok());
        let response = result.unwrap();

        // Validate response structure
        assert!(response.is_object());
        assert!(response.as_object().unwrap().contains_key("symbols"));
        assert!(response.as_object().unwrap().contains_key("total_count"));

        // We should have some symbols from the test data
        assert!(response["total_count"].as_u64().unwrap() > 0);
    }

    #[test]
    fn test_method_name() {
        let factory = MemoryRepositoryFactory::new();
        let repository = factory.create_symbol_repository();

        let handler = GetSymbolsHandler::new(repository);
        assert_eq!(handler.method_name(), "get_symbols");
    }

    #[test]
    fn test_error_codes() {
        let error = RmcpError::ParseError("test error".to_string());
        assert_eq!(error.error_code().code(), -32602); // InvalidParams

        let error = RmcpError::RepositoryError("test error".to_string());
        assert_eq!(error.error_code().code(), -32000); // ServerError

        let error = RmcpError::Other("test error".to_string());
        assert_eq!(error.error_code().code(), -32603); // InternalError
    }
}
