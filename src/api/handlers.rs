use axum::{
    Json,
    extract::{Path, Query, State},
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
#[derive(Deserialize, Default)]
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

/// List all symbols with optional filtering - Query parameter version
pub async fn list_symbols(
    State(repository): State<Arc<dyn SymbolRepository>>,
    Query(params): Query<ListSymbolsQuery>,
) -> ApiResult<Json<SymbolsResponse>> {
    process_list_symbols(repository, params).await
}

/// Process the list_symbols request with the given parameters
async fn process_list_symbols(
    repository: Arc<dyn SymbolRepository>,
    params: ListSymbolsQuery,
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
}

/// Response for symbol interpretation
#[derive(Serialize)]
pub struct InterpretResponse {
    pub symbol: Symbol,
    pub interpretation: String,
}

/// Validate an interpret request
fn validate_interpret_request(request: &InterpretRequest) -> Result<(), ApiError> {
    if request.symbol_id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol ID cannot be empty".to_string(),
        ));
    }
    Ok(())
}

/// Interpret a symbol in a given context
pub async fn interpret_symbol(
    State(repository): State<Arc<dyn SymbolRepository>>,
    Json(request): Json<InterpretRequest>,
) -> ApiResult<Json<InterpretResponse>> {
    // Validate request
    validate_interpret_request(&request)?;

    // Get the symbol from repository
    let symbol = repository.get_symbol(&request.symbol_id).await?;

    // For now, we'll return a simple interpretation - this would be enhanced
    // with the LLM integration in the future
    let interpretation = match &request.context {
        Some(context) => format!(
            "Symbol interpretation for '{}' in context '{}': {}",
            symbol.name, context, symbol.description
        ),
        None => format!(
            "General interpretation for '{}': {}",
            symbol.name, symbol.description
        ),
    };

    Ok(Json(InterpretResponse {
        symbol,
        interpretation,
    }))
}

/// Response for related symbols
#[derive(Serialize)]
pub struct RelatedSymbolsResponse {
    pub symbols: Vec<Symbol>,
    pub total_count: usize,
}

/// Get all related symbols for a specific symbol ID
pub async fn get_related_symbols(
    Path(id): Path<String>,
    State(repository): State<Arc<dyn SymbolRepository>>,
) -> ApiResult<Json<RelatedSymbolsResponse>> {
    // Validate ID
    if id.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "Symbol ID cannot be empty".to_string(),
        ));
    }

    println!("Getting related symbols for ID: {}", id);

    // First, retrieve the base symbol to get its related symbol IDs
    let base_symbol = repository.get_symbol(&id).await?;

    println!("Base symbol retrieved: {}", base_symbol.name);
    println!("Related symbol IDs: {:?}", base_symbol.related_symbols);

    // If there are no related symbols, return an empty list
    if base_symbol.related_symbols.is_empty() {
        println!("No related symbols found");
        return Ok(Json(RelatedSymbolsResponse {
            symbols: Vec::new(),
            total_count: 0,
        }));
    }

    // Fetch all related symbols
    let mut related_symbols = Vec::new();

    for related_id in &base_symbol.related_symbols {
        println!("Fetching related symbol: {}", related_id);
        match repository.get_symbol(related_id).await {
            Ok(symbol) => {
                println!("Successfully fetched related symbol: {}", symbol.name);
                related_symbols.push(symbol);
            }
            Err(err) => {
                // Log the error but continue with other related symbols
                println!("Error fetching related symbol {}: {}", related_id, err);
                eprintln!("Error fetching related symbol {}: {}", related_id, err);
            }
        }
    }

    let total_count = related_symbols.len();
    println!("Total related symbols found: {}", total_count);

    Ok(Json(RelatedSymbolsResponse {
        symbols: related_symbols,
        total_count,
    }))
}

/// Response for categories
#[derive(Serialize)]
pub struct CategoriesResponse {
    pub categories: Vec<String>,
    pub total_count: usize,
}

/// Get all available categories
pub async fn get_categories(
    State(repository): State<Arc<dyn SymbolRepository>>,
) -> ApiResult<Json<CategoriesResponse>> {
    // Get all symbols
    let symbols = repository.list_symbols(None).await?;

    // Extract unique categories
    let mut categories = std::collections::HashSet::new();
    for symbol in &symbols {
        categories.insert(symbol.category.clone());
    }

    // Convert to Vec for the response
    let categories: Vec<String> = categories.into_iter().collect();
    let total_count = categories.len();

    Ok(Json(CategoriesResponse {
        categories,
        total_count,
    }))
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

        let result = list_symbols(State(repository.clone()), Query(params)).await;

        assert!(result.is_ok());
        let response = result.unwrap().0; // Extract from Json wrapper
        assert!(!response.symbols.is_empty());
    }
}
