use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

// We need to import directly from the crate
use dream_ontology_mcp::domain::RepositoryFactory;
use dream_ontology_mcp::domain::SymbolRepository;
use dream_ontology_mcp::infrastructure::memory_repository::MemoryRepositoryFactory;

// Add a dependency on the correct modules instead of using API modules
#[tokio::test]
async fn test_list_symbols_integration() {
    // Set up repository with test data
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    // For now, we won't test the API directly, but just verify repository works
    let symbols = repository.list_symbols(None).await.unwrap();

    // Validate response
    assert!(!symbols.is_empty());
}

#[tokio::test]
async fn test_get_symbol_integration() {
    // Set up repository with test data
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    // Get the first symbol from the repo to test with
    let symbols = repository.list_symbols(None).await.unwrap();
    let first_symbol = symbols.first().unwrap();

    // Test direct repository call
    let result = repository.get_symbol(&first_symbol.id).await;

    // Validate response
    assert!(result.is_ok());
    let symbol = result.unwrap();
    assert_eq!(symbol.id, first_symbol.id);
}

#[tokio::test]
async fn test_search_symbols_integration() {
    // Set up repository with test data
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    // Get the first symbol from the repo to test with
    let symbols = repository.list_symbols(None).await.unwrap();
    let first_symbol = symbols.first().unwrap();

    // Search symbols using query from the first symbol (should find it)
    let query = &first_symbol.name[0..3]; // Use first few characters of name
    let search_results = repository.search_symbols(query).await.unwrap();

    // Validate response
    assert!(!search_results.is_empty());
    assert!(search_results.iter().any(|s| s.id == first_symbol.id));
}

#[tokio::test]
async fn test_error_handling_symbol_not_found() {
    // Set up repository with test data
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    // Call repository with nonexistent ID
    let result = repository.get_symbol("nonexistent-id").await;

    // Validate error response
    assert!(result.is_err());
    match result {
        Err(err) => {
            let error_string = err.to_string();
            assert!(error_string.contains("Not found"));
        }
        _ => panic!("Expected error response"),
    }
}
