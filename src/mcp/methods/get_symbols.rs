use async_trait::async_trait;
use serde_json;
use std::sync::Arc;

use crate::domain::{Symbol, SymbolSet};
use crate::mcp::schema::{GetSymbolsParams, GetSymbolsResponse, SymbolDTO};

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

/// Simple error type
#[derive(Debug)]
pub enum RmcpError {
    ParseError(String),
    Other(String),
}

impl From<serde_json::Error> for RmcpError {
    fn from(err: serde_json::Error) -> Self {
        RmcpError::ParseError(err.to_string())
    }
}

/// MCP handler for get_symbols method
pub struct GetSymbolsHandler {
    // This will hold our symbol sets
    // For now, we'll use a placeholder
    _symbol_sets: Vec<SymbolSet>,
}

impl GetSymbolsHandler {
    /// Create a new handler
    pub fn new() -> Self {
        GetSymbolsHandler {
            _symbol_sets: Vec::new(),
        }
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
        let _params: GetSymbolsParams = call.parse_params()?;

        // For now, return a placeholder response
        // Later we'll implement actual symbol retrieval from storage
        let symbols = Vec::new();

        Ok(serde_json::to_value(GetSymbolsResponse {
            symbols,
            total_count: 0,
        })?)
    }
}

/// Factory function to create the get_symbols handler
pub fn get_symbols() -> GetSymbolsHandler {
    GetSymbolsHandler::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_symbols_handler() {
        let handler = GetSymbolsHandler::new();

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

        // For now we expect empty results
        assert_eq!(response["symbols"], json!([]));
        assert_eq!(response["total_count"], json!(0));
    }

    #[test]
    fn test_method_name() {
        let handler = GetSymbolsHandler::new();
        assert_eq!(handler.method_name(), "get_symbols");
    }
}
