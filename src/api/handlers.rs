use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use super::error::{ApiError, ApiResult};
use crate::domain::{Symbol, SymbolRepository};

/// Health check handler
pub async fn health_check() -> &'static str {
    "Dream Ontology MCP API is healthy"
}

/// Represents a collection of symbols in a response
#[derive(Serialize)]
pub struct SymbolsResponse {
    pub symbols: Vec<Symbol>,
    pub total_count: usize,
}

/// Query parameters for listing symbols
#[derive(Deserialize)]
pub struct ListSymbolsQuery {
    pub category: Option<String>,
    pub query: Option<String>,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

/// Default limit for symbol queries
fn default_limit() -> usize {
    50 // Default to 50 symbols
}

/// List all symbols with optional filtering
pub async fn list_symbols(
    State(repository): State<Arc<dyn SymbolRepository>>,
    Json(params): Json<ListSymbolsQuery>,
) -> ApiResult<Json<SymbolsResponse>> {
    // Validate input parameters
    if let Some(ref query) = params.query {
        if query.trim().is_empty() {
            return Err(ApiError::BadRequest(
                "Search query cannot be empty".to_string(),
            ));
        }
    }

    if let Some(ref category) = params.category {
        if category.trim().is_empty() {
            return Err(ApiError::BadRequest("Category cannot be empty".to_string()));
        }
    }

    // Determine which repository method to call based on parameters
    let symbols = match (params.category.as_deref(), params.query.as_deref()) {
        (_, Some(query)) => repository.search_symbols(query).await?,
        (Some(category), None) => repository.list_symbols(Some(category)).await?,
        (None, None) => repository.list_symbols(None).await?,
    };

    // Apply limit and count
    let total_count = symbols.len();
    let symbols = symbols.into_iter().take(params.limit).collect();

    Ok(Json(SymbolsResponse {
        symbols,
        total_count,
    }))
}

/// Get a specific symbol by ID
pub async fn get_symbol(
    Path(id): Path<String>,
    State(repository): State<Arc<dyn SymbolRepository>>,
) -> ApiResult<Json<Symbol>> {
    // Validate ID
    if id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol ID cannot be empty".to_string(),
        ));
    }

    // Retrieve the symbol from repository
    let symbol = repository.get_symbol(&id).await?;
    Ok(Json(symbol))
}

/// Request body for symbol interpretation
#[derive(Deserialize)]
pub struct InterpretRequest {
    pub symbol_id: String,
    pub context: Option<String>,
    pub query: Option<String>,
}

/// Response for symbol interpretation
#[derive(Serialize)]
pub struct InterpretResponse {
    pub symbol_id: String,
    pub context: Option<String>,
    pub interpretation: String,
}

/// Validate an interpretation request
fn validate_interpret_request(request: &InterpretRequest) -> Result<(), ApiError> {
    if request.symbol_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol ID cannot be empty".to_string(),
        ));
    }

    // If context is provided, it shouldn't be empty
    if let Some(ref context) = request.context {
        if context.trim().is_empty() {
            return Err(ApiError::BadRequest("Context cannot be empty".to_string()));
        }
    }

    // If query is provided, it shouldn't be empty
    if let Some(ref query) = request.query {
        if query.trim().is_empty() {
            return Err(ApiError::BadRequest("Query cannot be empty".to_string()));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::RepositoryFactory;
    use crate::infrastructure::memory_repository::MemoryRepositoryFactory;

    #[tokio::test]
    async fn test_health_check() {
        let response = health_check().await;
        assert_eq!(response, "Dream Ontology MCP API is healthy");
    }

    #[tokio::test]
    async fn test_get_symbol_success() {
        // Create test repository with data
        let factory = MemoryRepositoryFactory::new().with_test_data();
        let repository = factory.create_symbol_repository();

        // Get the first symbol from the repo to test with
        let symbols = repository.list_symbols(None).await.unwrap();
        let first_symbol = symbols.first().unwrap();

        // Test the handler
        let path = Path(first_symbol.id.clone());
        let result = get_symbol(path, State(repository)).await;

        assert!(result.is_ok());
        let symbol = result.unwrap().0; // Extract from Json wrapper
        assert_eq!(symbol.id, first_symbol.id);
    }

    #[tokio::test]
    async fn test_list_symbols() {
        // Create test repository with data
        let factory = MemoryRepositoryFactory::new().with_test_data();
        let repository = factory.create_symbol_repository();

        // Test with empty params (list all)
        let params = ListSymbolsQuery {
            category: None,
            query: None,
            limit: 10,
        };

        let result = list_symbols(State(repository.clone()), Json(params)).await;

        assert!(result.is_ok());
        let response = result.unwrap().0; // Extract from Json wrapper
        assert!(!response.symbols.is_empty());
    }
}
