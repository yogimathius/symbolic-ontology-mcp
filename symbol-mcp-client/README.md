# Symbol Ontology MCP Client

[![License: MPL-2.0 or Commercial](https://img.shields.io/badge/license-MPL--2.0%20or%20Commercial-blue.svg)](../LICENSE.md)

A client for accessing the Symbol Ontology symbolic reasoning engine through the Model Context Protocol (MCP).

## Installation

```bash
# Install from crates.io
cargo install symbol-mcp-client

# Or install directly from the repository
cargo install --git https://github.com/yogimathius/symbolic-ontology-mcp
```

## Usage

Once installed, you can run the MCP client:

```bash
# Start the client with default settings
symbol-mcp

# Specify a custom port
symbol-mcp --port 4000

# Use an API key for unlocking premium features
symbol-mcp --api-key your-api-key

# Connect to a specific API endpoint
symbol-mcp --api-url https://your-custom-endpoint.com

# Enable verbose logging
symbol-mcp --verbose
```

## Environment Variables

You can also configure the client using environment variables:

- `PORT`: The port to listen on (default: 3002)
- `SYMBOL_MCP_API_KEY`: Your API key for premium features
- `SYMBOL_MCP_API_URL`: The API endpoint to connect to
- `VERBOSE`: Enable verbose logging

## Using with Claude or Other MCP Clients

To use with Claude Desktop or other MCP-compatible clients, add this to your MCP client configuration:

```json
{
  "tool_servers": [
    {
      "name": "DreamOntology",
      "transport": {
        "type": "sse",
        "url": "http://localhost:3002"
      }
    }
  ]
}
```

## Features

- **Symbol Search**: Search for symbols and their meanings
- **Category Filtering**: Filter symbols by category
- **Symbol Details**: Get detailed interpretations of symbols
- **Relationship Queries**: Explore relationships between symbols

## License

This project is dual-licensed:

1. **Mozilla Public License 2.0 (MPL-2.0)**: For personal, educational, non-profit, and open-source use.
2. **Commercial License**: Required for any commercial or for-profit use.

See [LICENSE.md](../LICENSE.md) for complete details and terms.

For commercial licensing inquiries, please contact [info@yogimathius.dev].
