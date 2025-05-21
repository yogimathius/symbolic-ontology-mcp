# Symbol MCP Client

A standalone Model Context Protocol (MCP) client for the Symbol Ontology project. This client provides a symbolic reasoning engine that can be integrated with AI tools like Claude through the MCP protocol.

## Features

- **Standalone Operation**: Runs as a complete executable without requiring the API server
- **Direct Database Connectivity**: Connects directly to a PostgreSQL database
- **Full MCP Compliance**: Implements the MCP protocol with SSE transport
- **Multiple Symbol Operations**: Search, filter, and retrieve symbols and symbol sets
- **Easy Integration**: Works with Cursor AI and other MCP-compatible tools

## Installation

### From GitHub

```bash
# Install directly from the GitHub repository
cargo install --git https://github.com/yogimathius/symbolic-ontology-mcp symbol-mcp-client
```

### From Source

```bash
# Clone the repository
git clone https://github.com/yogimathius/symbolic-ontology-mcp.git
cd symbolic-ontology-mcp

# Build and install
cargo install --path symbol-mcp-client
```

## Usage

### Basic Usage

```bash
# Start the MCP server with default settings
symbol-mcp

# The server will start at http://localhost:3000 by default
# SSE endpoint: http://localhost:3000/sse
# Message endpoint: http://localhost:3000/message
```

### Command Line Options

```bash
# Show help
symbol-mcp --help

# Run with custom port
symbol-mcp --port 4000

# Run with custom database URL
symbol-mcp --database-url postgres://username:password@localhost:5432/symbol_ontology

# Run with increased verbosity (debug logging)
symbol-mcp --verbose
```

### Setting Up the Database

Before running the client, you need a PostgreSQL database:

```bash
# Create the database
createdb symbol_ontology

# Set environment variable (alternative to --database-url flag)
export DATABASE_URL=postgres://username:password@localhost:5432/symbol_ontology
```

## MCP Methods

The following MCP methods are available:

### get_symbols

List all symbols with optional limit.

```json
{
  "limit": 50 // Optional: Maximum number of symbols to return (default: 50)
}
```

### search_symbols

Search symbols by text query.

```json
{
  "query": "water", // Required: Search text
  "limit": 50 // Optional: Maximum number of symbols to return (default: 50)
}
```

### filter_by_category

Get symbols filtered by category.

```json
{
  "category": "dream", // Required: Category name to filter by
  "limit": 50 // Optional: Maximum number of symbols to return (default: 50)
}
```

### get_categories

Get all available symbol categories.

```json
{
  // No parameters required
}
```

### get_symbol_sets

List all symbol sets with optional limit.

```json
{
  "limit": 50 // Optional: Maximum number of symbol sets to return (default: 50)
}
```

### search_symbol_sets

Search symbol sets by name or description.

```json
{
  "query": "element", // Required: Search text
  "limit": 50 // Optional: Maximum number of symbol sets to return (default: 50)
}
```

## Integration with Claude

To use this client with Claude through Cursor AI:

1. Start the MCP server:

   ```bash
   symbol-mcp
   ```

2. In Cursor settings, add a new MCP provider:

   - Name: Symbol Ontology
   - URL: http://localhost:3000/sse
   - Authentication: None

3. Save the settings and the symbol-ontology methods will be available to Claude.

## Architecture

The client is structured as follows:

```
symbol-mcp-client/
├── src/
│   ├── mcp/             # MCP implementation
│   │   ├── methods/     # Method handlers
│   │   ├── schema.rs    # Data transfer objects
│   │   ├── service.rs   # MCP service implementation
│   │   └── mod.rs       # Module exports
│   ├── main.rs          # Executable entry point
│   └── lib.rs           # Library exports
└── tests/               # Unit and integration tests
```

## Development

### Running Tests

```bash
# Run tests with local feature enabled (for in-memory repositories)
cargo test --features local

# Run tests for everything in the workspace
cd ..
cargo test --workspace --features symbol-mcp-client/local
```

### Building for Release

```bash
cargo build --release
```

The release binary will be available at `target/release/symbol-mcp`.

## Dependencies

- `rmcp` - Rust implementation of the MCP protocol
- `tokio` - Async runtime
- `clap` - Command line argument parsing
- `serde` - Serialization/deserialization
- `ontology-core` - Core symbol ontology library (internal)

## License

This project is dual-licensed under:

- [Mozilla Public License 2.0](../LICENSE.md) for non-commercial use
- [Commercial License](../COMMERCIAL_LICENSE.md) for business/for-profit use
