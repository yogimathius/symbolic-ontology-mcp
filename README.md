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
- (Optional) Docker and Docker Compose for PostgreSQL setup

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/dream-ontology-mcp.git
cd dream-ontology-mcp

# Build the project
cargo build
```

### Database Setup

The Dream Ontology Server supports two storage backends:

1. **In-Memory Repository**: Default option, good for development and testing
2. **PostgreSQL with pgvector**: Production-ready option with vector embedding support

#### Using the In-Memory Repository

The in-memory repository is enabled by default and requires no additional setup.

<!-- Not implemented -->

#### Using PostgreSQL with pgvector

1. Start a PostgreSQL instance with pgvector using Docker Compose:

```bash
# Start PostgreSQL with pgvector
docker-compose up -d
```

2. Configure the application to use PostgreSQL by editing `.env`:

```
# Set to "false" to use PostgreSQL
USE_MEMORY_REPOSITORY=false

# Uncomment the DATABASE_URL
DATABASE_URL=postgres://postgres:postgres@localhost:5432/symbol_ontology
```

The server will automatically:

- Create the necessary tables on startup
- Enable the pgvector extension
- Seed test data (in development mode)

#### Vector Embedding Support

The PostgreSQL implementation includes support for vector embeddings:

- Symbols can have 384-dimensional vector embeddings
- Vector similarity search is available for finding related symbols
- This follows the architecture in [docs/architecture/vector-db-architecture.md](docs/architecture/vector-db-architecture.md)

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

## Database Seeder

The project includes database seeding tools to populate your database with dream symbols from various datasets. For detailed instructions, see [README_SEEDER.md](README_SEEDER.md).

Quick start:

```bash
# Seed with the sample dataset
cargo run --bin manual_seed data/sample_dream_symbols.csv

# Or seed with Kaggle datasets (after downloading)
cargo run --bin manual_seed path/to/downloaded/csv
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.
