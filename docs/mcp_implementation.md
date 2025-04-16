# MCP Implementation Documentation

This document provides an overview of the Model Context Protocol (MCP) implementation in the Dream Ontology Symbolic server.

## Overview

The MCP implementation allows an LLM client (such as Claude, ChatGPT, or a custom AI assistant) to query our symbolic database through a standardized JSON-RPC 2.0 protocol. This enables AI models to ground their responses in factual data from our symbolic ontology without requiring custom integration for each LLM provider.

## MCP Methods

### `get_symbols`

The primary MCP method implemented is `get_symbols`, which allows clients to retrieve symbol data from the ontology.

**Parameters:**

```json
{
  "category": "string?", // Optional category filter
  "query": "string?", // Optional search query
  "limit": "number?" // Optional result limit (default: 50)
}
```

**Response:**

```json
{
  "symbols": [
    {
      "id": "string",
      "name": "string",
      "category": "string",
      "description": "string",
      "related_symbols": ["string"]
    }
  ],
  "total_count": "number"
}
```

### Handler Implementation

The `GetSymbolsHandler` class is responsible for processing MCP requests:

1. It validates the incoming parameters
2. Queries the appropriate repository based on filters
3. Converts domain models to DTO responses
4. Handles error cases according to JSON-RPC 2.0 spec

## Error Handling

The MCP implementation includes standardized error handling with the following error codes:

| Code   | Name             | Description                      |
| ------ | ---------------- | -------------------------------- |
| -32700 | Parse error      | Invalid JSON                     |
| -32600 | Invalid request  | Request object validation failed |
| -32601 | Method not found | Method doesn't exist             |
| -32602 | Invalid params   | Invalid method parameters        |
| -32603 | Internal error   | Internal server error            |
| -32000 | Repository error | Data access error                |
| -32001 | Not found        | Requested entity not found       |
| -32002 | Conflict         | Entity already exists            |

## Integration with Axum

The MCP server is integrated with the Axum web framework through:

1. A dedicated `/mcp` endpoint
2. Middleware for request validation and logging
3. Connection to the same repositories used by the REST API

## Testing

MCP functionality can be tested using curl or the provided test script in `scripts/test_api.sh`. Example:

```bash
curl -X POST http://localhost:3001/mcp -H "Content-Type: application/json" -d '{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "get_symbols",
  "params": {}
}'
```

## Future MCP Methods

The following methods are planned for future implementation:

1. `get_symbol` - Retrieve a single symbol by ID
2. `get_symbol_sets` - Retrieve sets of related symbols
3. `get_related_symbols` - Get symbols related to a given symbol

## Development Guidelines

When extending the MCP functionality:

1. Define a clear schema for the method in `schema.rs`
2. Implement the handler following the `Handler` trait
3. Register the method in the MCP server
4. Write comprehensive tests for the method
5. Document the method in this file

## MCP Compliance

Our implementation aims to be fully compliant with the MCP specification:

- Proper JSON-RPC 2.0 formatting
- Support for request IDs and batching
- Standard error codes and messages
- Synchronous result handling

## References

- [MCP Specification](https://modelcontextprotocol.io/)
- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
- [RMCP SDK Documentation](https://docs.rs/rmcp/)
