use dream_ontology_mcp::mcp::methods::get_symbols::{GetSymbolsHandler, Handler, MethodCall};
use serde_json::json;

#[tokio::test]
async fn test_get_symbols_handler_name() {
    let handler = GetSymbolsHandler::new();
    assert_eq!(handler.method_name(), "get_symbols");
}

#[tokio::test]
async fn test_get_symbols_empty_params() {
    let handler = GetSymbolsHandler::new();
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
    let handler = GetSymbolsHandler::new();
    let call = MethodCall {
        id: "1".to_string(),
        method: "get_symbols".to_string(),
        params: json!({ "category": "dream" }),
    };

    let result = handler.handle(call).await.unwrap();

    // Validate we get results filtered by category
    assert!(result.is_object());
    assert!(result.get("symbols").is_some());

    let symbols = result.get("symbols").unwrap().as_array().unwrap();
    for symbol in symbols {
        assert_eq!(symbol.get("category").unwrap().as_str().unwrap(), "dream");
    }
}

#[tokio::test]
async fn test_get_symbols_with_search() {
    let handler = GetSymbolsHandler::new();
    let call = MethodCall {
        id: "1".to_string(),
        method: "get_symbols".to_string(),
        params: json!({ "search": "water" }),
    };

    let result = handler.handle(call).await.unwrap();

    // Validate search results contain the term in name or description
    let symbols = result.get("symbols").unwrap().as_array().unwrap();
    for symbol in symbols {
        let name = symbol.get("name").unwrap().as_str().unwrap().to_lowercase();
        let description = symbol
            .get("description")
            .unwrap()
            .as_str()
            .unwrap()
            .to_lowercase();

        assert!(name.contains("water") || description.contains("water"));
    }
}
