# MCP Error Codes Reference

This document provides a reference for error codes used in the Model Context Protocol (MCP) implementation in this project.

## JSON-RPC 2.0 Error Codes

The Model Context Protocol (MCP) is built on top of JSON-RPC 2.0, which defines standard error codes in the following ranges:

- `-32700 to -32600`: Reserved for pre-defined errors
- `-32000 to -32099`: Reserved for implementation-defined server errors
- Client-defined codes: May be used for custom errors

### Pre-defined JSON-RPC 2.0 Error Codes

| Code   | Name             | Description                                  |
| ------ | ---------------- | -------------------------------------------- |
| -32600 | `InvalidRequest` | The JSON sent is not a valid Request object  |
| -32601 | `MethodNotFound` | The method does not exist / is not available |
| -32602 | `InvalidParams`  | Invalid method parameter(s)                  |
| -32603 | `InternalError`  | Internal JSON-RPC error                      |

### Implementation-defined Server Error Codes

| Code   | Name          | Description                            |
| ------ | ------------- | -------------------------------------- |
| -32000 | `ServerError` | Generic server-side error              |
| -32001 | `NotFound`    | Requested resource not found           |
| -32002 | `Conflict`    | Resource conflict (e.g., duplicate ID) |

## Mapping to HTTP Status Codes

When serving MCP methods over HTTP, we map JSON-RPC error codes to appropriate HTTP status codes:

| MCP/JSON-RPC Error        | HTTP Status Code          | Reason                                   |
| ------------------------- | ------------------------- | ---------------------------------------- |
| `InvalidRequest` (-32600) | 400 Bad Request           | The client sent an invalid request       |
| `MethodNotFound` (-32601) | 404 Not Found             | The requested endpoint does not exist    |
| `InvalidParams` (-32602)  | 400 Bad Request           | The request parameters are invalid       |
| `InternalError` (-32603)  | 500 Internal Server Error | An error occurred on the server          |
| `ServerError` (-32000)    | 500 Internal Server Error | An error occurred on the server          |
| `NotFound` (-32001)       | 404 Not Found             | The requested resource does not exist    |
| `Conflict` (-32002)       | 409 Conflict              | The request conflicts with current state |

## Error Response Format

### JSON-RPC 2.0 Error Response

```json
{
  "jsonrpc": "2.0",
  "id": "<request-id>",
  "error": {
    "code": -32000,
    "message": "Server error",
    "data": "Detailed error message"
  }
}
```

### HTTP API Error Response

```json
{
  "error": {
    "type": "Not Found",
    "message": "Symbol with ID 'xyz' not found"
  }
}
```

## References

- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification#error_object)
- [Model Context Protocol Specification](https://modelcontextprotocol.io)
- [HTTP Status Codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status)
- [RMCP (Rust MCP) SDK Documentation](https://docs.rs/rmcp)
- [RMCP GitHub Repository](https://github.com/4t145/rmcp)
