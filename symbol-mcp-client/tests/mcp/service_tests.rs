use symbol_mcp_client::mcp::service::SymbolService;

#[cfg(feature = "local")]
#[tokio::test]
async fn test_symbol_service_new() {
    // Test that we can create a service instance without errors
    let _service = SymbolService::new();
    assert!(true, "SymbolService was successfully created");
}
