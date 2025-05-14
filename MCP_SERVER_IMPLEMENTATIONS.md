# MCP Server Implementations

This document describes the different MCP (Model Context Protocol) server implementations available in this project.

## Overview

We have implemented several different approaches to expose MCP services:

1. **SSE-based MCP Server** (original implementation)
2. **WebSocket-based MCP Server** (new implementation)
3. **HTTP Upgrade-based MCP Server** (new implementation)
4. **Streamable HTTP-based MCP Server** (new implementation)

Each implementation has its own advantages and disadvantages, which are described below.

## 1. SSE-based MCP Server

**Binary:** `mcp_server`

**Port:** 3000

**Description:**
The original implementation uses Server-Sent Events (SSE) for one-way communication from server to client, combined with regular HTTP requests for client-to-server communication.

**Advantages:**

- Simple to implement
- Works well with web browsers (native EventSource support)
- Passes through proxies and firewalls easily

**Disadvantages:**

- Not truly bidirectional (uses separate HTTP requests for client-to-server communication)
- Limited to HTTP/1.1
- Connection limit in browsers (typically 6 connections per domain)

**Usage:**

```bash
cargo run --bin mcp_server
```

## 2. WebSocket-based MCP Server

**Binary:** `mcp_websocket_server`

**Port:** 3003

**Description:**
This implementation uses WebSockets for bidirectional communication between client and server.

**Advantages:**

- True bidirectional communication
- Lower latency than SSE
- Better for high-frequency messaging
- Single connection for both directions

**Disadvantages:**

- More complex to implement
- May be blocked by some proxies and firewalls
- Requires WebSocket support in client

**Usage:**

```bash
cargo run --bin mcp_websocket_server
```

**Test with:** `test_websocket.html`

## 3. HTTP Upgrade-based MCP Server

**Binary:** `mcp_http_upgrade_server`

**Port:** 3004

**Description:**
This implementation uses HTTP Upgrade mechanism to establish a persistent connection that can be used for bidirectional communication.

**Advantages:**

- Works with standard HTTP infrastructure
- Can upgrade an existing HTTP connection
- Good for scenarios where you start with HTTP and need to switch to a more efficient protocol

**Disadvantages:**

- Not supported in browsers directly
- Requires special client implementation
- May be blocked by some proxies

**Usage:**

```bash
cargo run --bin mcp_http_upgrade_server
```

**Test with:** `test_http_upgrade.html` (basic connectivity check only)

## 4. Streamable HTTP-based MCP Server

**Binary:** `mcp_streamable_http_server`

**Port:** 3005

**Description:**
This implementation uses a combination of HTTP endpoints to create a session-based communication channel. The client establishes a session and then polls for messages while sending requests via regular HTTP.

**Advantages:**

- Works with any HTTP client
- Passes through all proxies and firewalls
- No special protocol requirements
- Easy to implement on both client and server

**Disadvantages:**

- Higher latency due to polling
- More resource-intensive due to frequent connections
- Not truly real-time

**Usage:**

```bash
cargo run --bin mcp_streamable_http_server
```

**Test with:** `test_streamable_http.html`

## Testing All Implementations

A combined test page is available at `test_all_servers.html` that allows you to test and compare all three new implementations.

## Deployment Considerations

When deploying to production, consider the following:

1. **For public-facing services:**

   - HTTP-based approaches (SSE or Streamable HTTP) are more likely to work across various network environments
   - Add proper CORS headers for browser clients
   - Consider rate limiting and authentication

2. **For internal services or controlled environments:**

   - WebSocket or HTTP Upgrade may provide better performance
   - Ensure your network infrastructure supports these protocols

3. **For Fly.io deployment:**
   - Ensure your `fly.toml` includes the appropriate port configurations
   - Add health checks appropriate for the protocol used

## Choosing the Right Implementation

The best implementation depends on your specific use case:

- **Browser clients:** SSE or WebSocket
- **Low latency requirements:** WebSocket
- **Maximum compatibility:** Streamable HTTP
- **Internal services:** HTTP Upgrade or WebSocket
- **Mobile clients:** Any, but WebSocket often performs better

For most cases, the WebSocket implementation provides the best balance of performance and functionality.
