# Cursor MCP Client Workaround

## Issue Description

When using the Cursor MCP client with our API, there's a problem with optional string parameters (`Option<String>`). The client returns an error when trying to use optional parameters:

```
Error calling tool: Parameter 'category' must be of type string,null, got string
```

## Details

- Our implementation follows the standard MCP patterns for optional parameters
- The error occurs specifically with `Option<String>` parameters (query and category)
- The limit parameter (which is a usize) works correctly
- The error message suggests a validation issue in how Cursor's MCP client handles the JSON schema for optional parameters

## Workaround Implementation

To work around this issue, we've implemented dedicated methods with non-optional parameters:

1. `search_symbols` - A method that takes a required `query` parameter
2. `filter_by_category` - A method that takes a required `category` parameter
3. `get_symbols` - A simplified method that no longer takes a category parameter

### Original Method (Modified):

```rust
#[tool(description = "List all symbols (without filtering)")]
async fn get_symbols(
    &self,
    #[tool(aggr)] params: GetSymbolsParams,
) -> Result<CallToolResult, rmcp::Error> {
    // Implementation with no category parameter
}
```

### Workaround Methods:

```rust
#[tool(description = "Search symbols by query term (non-optional parameter)")]
async fn search_symbols(
    &self,
    #[tool(aggr)] params: SearchSymbolsParams,
) -> Result<CallToolResult, rmcp::Error> {
    // Implementation using required query parameter
}

#[tool(description = "Filter symbols by category (non-optional parameter)")]
async fn filter_by_category(
    &self,
    #[tool(aggr)] params: CategorySymbolsParams,
) -> Result<CallToolResult, rmcp::Error> {
    // Implementation using required category parameter
}
```

## How to Use

When working with Cursor's MCP client:

1. For searches, use the `search_symbols` method
2. For category filtering, use the `filter_by_category` method
3. For simple listing with just a limit, use the `get_symbols` method

Example:

```
mcp_symbol_ontology_search_symbols(query="water", limit=5)
mcp_symbol_ontology_filter_by_category(category="nature", limit=3)
mcp_symbol_ontology_get_symbols(limit=10)
```

## Future Considerations

This workaround is temporary until the issue is resolved in the Cursor MCP client. Once the issue is fixed, we can consider:

1. Keeping these methods for backward compatibility
2. Restoring optional parameters to the `get_symbols` method
3. Documenting the change for users

## References

- [MCP Official Documentation](https://modelcontextprotocol.io/introduction)
- [Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk)
