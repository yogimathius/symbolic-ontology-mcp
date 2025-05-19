# Dream Ontology MCP Server

[![License: MPL-2.0 or Commercial](https://img.shields.io/badge/license-MPL--2.0%20or%20Commercial-blue.svg)](./LICENSE.md)

**IMPORTANT: This software is dual-licensed under Mozilla Public License 2.0 (for non-commercial use) and a Commercial License (for business/for-profit use). Commercial use requires a paid license. [Learn more](./LICENSE.md)**

A symbolic reasoning engine built in Rust that serves as an MCP-compliant server for dream and symbolic ontology. This server enables applications to query and reason about dream symbols, their meanings, and their relationships.

## Project Status

**✅ Status: Feature Complete for v1.5/2**

The Dream Ontology MCP Server is now feature complete for version 1.5/2, with:

- Core domain model implementation
- Data storage with both in-memory and PostgreSQL support
- MCP protocol implementation with multiple methods
- HTTP API with Axum
- Data seeding capabilities
- Comprehensive test coverage

The production server is deployed at: `symbolic-grounding-api.fly.dev`

## Architecture

The Dream Ontology project consists of two separate components:

1. **REST API Server**: Provides HTTP endpoints for accessing the symbol ontology
2. **MCP Server**: Implements the Model Context Protocol for integration with LLM agents

This separation allows both traditional API access and MCP-based integration.

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

### Core Features

- **Symbol and SymbolSet Domain Models**: Fully implemented with properties for id, name, category, description, interpretations, and related symbols
- **Repository Pattern**: Clean separation between domain logic and data storage
- **MCP Protocol Implementation**: Compliant with the Model Context Protocol for LLM tool use
- **Vector Embedding Support**: (PostgreSQL) For semantic search capabilities
- **Data Seeding**: Tools to populate the database with dream symbols

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

# Database connection string
DATABASE_URL=postgres://postgres:postgres@localhost:5432/symbol_ontology
```

The server will automatically:

- Create the necessary tables on startup
- Enable the pgvector extension
- Seed test data (in development mode)

## Running the Services

### REST API Server

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

```bash
# Run the MCP server
cargo run --bin mcp_server

# The server will start at http://127.0.0.1:3001
```

## MCP API Documentation

The Symbol Ontology MCP API provides several methods for searching and retrieving dream symbols and their interpretations.

### Important Note on Parameter Handling

Due to a known issue with optional parameters in the MCP protocol implementation, this API provides specialized endpoints for different query types:

- Use `search_symbols` for text-based searches
- Use `filter_by_category` for category filtering
- Use `get_symbols` for listing all symbols (without filters)
- Use `

## License

This project is dual-licensed:

1. **Mozilla Public License 2.0 (MPL-2.0)**: For personal, educational, non-profit, and open-source use.
2. **Commercial License**: Required for any commercial or for-profit use.

See [LICENSE.md](./LICENSE.md) for complete details and terms.

For commercial licensing inquiries, please contact [info@yogimathius.dev].
