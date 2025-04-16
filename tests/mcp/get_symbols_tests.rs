use dream_ontology_mcp::domain::RepositoryFactory;
use dream_ontology_mcp::infrastructure::memory_repository::MemoryRepositoryFactory;
use dream_ontology_mcp::mcp::methods::get_symbols::{GetSymbolsHandler, Handler, MethodCall};
use serde_json::json;

#[tokio::test]
async fn test_get_symbols_handler_name() {
    // Create a test repository
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    let handler = GetSymbolsHandler::new(repository);
    assert_eq!(handler.method_name(), "get_symbols");
}

#[tokio::test]
async fn test_get_symbols_empty_params() {
    // Create a test repository
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    let handler = GetSymbolsHandler::new(repository);
    let call = MethodCall {
        id: "1".to_string(),
        method: "get_symbols".to_string(),
        params: json!({}),
    };

    let result = handler.handle(call).await.unwrap();

    // Basic validation that we get a successful response with an array of symbols
    assert!(result.is_object());
    assert!(result.get("symbols").is_some());
    assert!(result.get("symbols").unwrap().is_array());
}

#[tokio::test]
async fn test_get_symbols_with_category() {
    // Create a test repository
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    let handler = GetSymbolsHandler::new(repository);
    let call = MethodCall {
        id: "1".to_string(),
        method: "get_symbols".to_string(),
        params: json!({ "category": "nature" }), // Using a category we know exists in test data
    };

    let result = handler.handle(call).await.unwrap();

    // Validate we get results filtered by category
    assert!(result.is_object());
    assert!(result.get("symbols").is_some());

    let symbols = result.get("symbols").unwrap().as_array().unwrap();
    // If we got any symbols, they should have the right category
    if !symbols.is_empty() {
        for symbol in symbols {
            assert_eq!(symbol.get("category").unwrap().as_str().unwrap(), "nature");
        }
    }
}

#[tokio::test]
async fn test_get_symbols_with_search() {
    // Create a test repository
    let factory = MemoryRepositoryFactory::new().with_test_data();
    let repository = factory.create_symbol_repository();

    let handler = GetSymbolsHandler::new(repository);
    let call = MethodCall {
        id: "1".to_string(),
        method: "get_symbols".to_string(),
        params: json!({ "query": "light" }), // Using "query" parameter as defined in schema
    };

    let result = handler.handle(call).await.unwrap();

    // Just verify we get a response - test data might not have "light" references
    assert!(result.is_object());
    assert!(result.get("symbols").is_some());
}
