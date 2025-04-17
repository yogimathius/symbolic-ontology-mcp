# Dream Ontology MCP Server

A symbolic reasoning engine built in Rust that serves as an MCP-compliant server for dream and symbolic ontology.

## Architecture

The Dream Ontology project consists of two separate components in this repository:

1. **REST API Server**: Provides HTTP endpoints for accessing the symbol ontology
2. **MCP Server**: Implements the Model Context Protocol for integration with LLM agents

This separation follows the architecture described in [docs/architecture/architecture.md](docs/architecture/architecture.md), allowing both traditional API access and MCP-based integration.

```
┌─────────────────────┐               ┌─────────────────────┐
│                     │               │                     │
│   REST API Server   │◄─────────────►│   Symbol Database   │
│   (src/main.rs)     │               │                     │
└─────────────────────┘               └─────────────────────┘
                                              ▲
                                              │
                                              │
┌─────────────────────┐                       │
│                     │                       │
│   MCP Server        │◄──────────────────────┘
│   (src/bin/mcp_server.rs)│
└─────────────────────┘
```

## Getting Started

### Prerequisites

- Rust 1.65 or higher
- Cargo

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/dream-ontology-mcp.git
cd dream-ontology-mcp

# Build the project
cargo build
```

## Running the Services

### REST API Server

The REST API server provides HTTP endpoints for accessing the symbol ontology:

```bash
# Run the API server
cargo run

# The server will start at http://127.0.0.1:3000
```

Available endpoints:

- `GET /health` - Health check endpoint
- `GET /symbols` - List all symbols
- `GET /symbols/{id}` - Get a specific symbol by ID

### MCP Server

The MCP server implements the Model Context Protocol for integration with LLM agents:

```bash
# Run the MCP server
cargo run --bin mcp_server

# The server will start at http://127.0.0.1:3001
```

Available MCP tools:

- `get_symbols` - Get symbols from the ontology with optional filtering
- More tools coming soon!

### Running Both Services

For development, you can run both services simultaneously:

```bash
# In one terminal
cargo run

# In another terminal
cargo run --bin mcp_server
```

## Testing

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test domain::symbols
```

## MCP Integration

The MCP server can be used with Claude Desktop or any other MCP-compatible client.

To configure Claude Desktop to use this server:

1. Add this to your `claude_desktop_config.json`:

```json
{
  "tool_servers": [
    {
      "name": "DreamOntology",
      "transport": {
        "type": "sse",
        "url": "http://localhost:3001"
      }
    }
  ]
}
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
