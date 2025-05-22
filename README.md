# Symbol Ontology

[![License: MPL 2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](https://mozilla.org/MPL/2.0/)

**IMPORTANT: This software is dual-licensed under Mozilla Public License 2.0 (for non-commercial use) and a Commercial License (for business/for-profit use). Commercial use requires a paid license. [Learn more](./LICENSE.md)**

A symbolic reasoning engine built in Rust that serves as an MCP-compliant server for symbolic ontology. This server enables applications to query and reason about symbols, their meanings, and their relationships.

## Project Structure

The Symbol Ontology project is structured as a Rust workspace with multiple crates:

```
symbol-ontology/
├── ontology-core/           # Core domain models and database logic
├── ontology-api-server/     # API server implementation
├── symbol-mcp-client/       # Public MCP client binary
├── tests/                   # Integration tests
└── docs/                    # Documentation
```

## Features

### MCP Client Features

- Direct database connectivity without requiring the API server
- Full MCP protocol compliance with SSE transport for Claude integration
- Available MCP methods:
  - `get_symbols` - List all symbols with optional filtering
  - `search_symbols` - Search symbols by text query
  - `filter_by_category` - Get symbols filtered by category
  - `get_categories` - Get all available symbol categories
  - `get_symbol_sets` - List all symbol sets
  - `search_symbol_sets` - Search symbol sets by name or description

### API Server Features

- REST API endpoints for symbol management
- JSON response format
- Database-backed persistence

### Core Library Features

- Domain models for symbols and symbol sets
- Database repository interfaces and implementations
- Shared utilities for both client and server

## Getting Started

### Installation

#### Install from GitHub

```bash
# Install the symbol-mcp-client package directly from GitHub
cargo install --git https://github.com/yogimathius/symbolic-ontology-mcp symbol-mcp-client

# Verify installation
symbol-mcp --help
```

#### Build from Source

```bash
# Clone the repository
git clone https://github.com/yogimathius/symbolic-ontology-mcp.git
cd symbolic-ontology-mcp

# Build the project
cargo build --release

# Run the MCP client
cargo run -p symbol-mcp-client
```

### Database Setup

Before running the MCP client, you need a PostgreSQL database:

```bash
# Set up database connection string
export DATABASE_URL=postgres://username:password@localhost:5432/symbol_ontology

# Create database (if it doesn't exist)
createdb symbol_ontology
```

### Required Environment Variables

The Symbol Ontology MCP client requires the following environment variables:

| Variable     | Description                          | Example                                        |
| ------------ | ------------------------------------ | ---------------------------------------------- |
| DATABASE_URL | PostgreSQL connection string         | postgres://user:pass@host:5432/symbol_ontology |
| MCP_PORT     | Port for the MCP client to listen on | 3002                                           |
| RUST_LOG     | Log level (debug, info, warn, error) | info                                           |

### Docker Deployment

You can easily run the Symbol Ontology MCP client using Docker:

```bash
# Build the Docker image
docker build -t symbol-ontology-mcp .

# Run the container
docker run -p 3002:3002 \
  -e DATABASE_URL=postgres://user:pass@host:5432/symbol_ontology \
  -e RUST_LOG=info \
  -e MCP_PORT=3002 \
  symbol-ontology-mcp
```

#### Using docker-compose

```bash
# Create a .env file with your DATABASE_URL
echo "DATABASE_URL=postgres://user:pass@host:5432/symbol_ontology" > .env

# Start the service
docker-compose up -d
```

#### Deploying to Fly.io

This project includes configuration for deploying to Fly.io:

1. Install the Fly CLI:

   ```bash
   curl -L https://fly.io/install.sh | sh
   ```

2. Login to Fly.io:

   ```bash
   fly auth login
   ```

3. Set your database secret:

   ```bash
   fly secrets set DATABASE_URL=postgres://postgres:password@your-project.supabase.co:5432/postgres
   ```

4. Deploy the application:
   ```bash
   fly deploy
   ```

### Running the Client

Start the MCP client to serve symbolic queries:

```bash
# Run with default settings
symbol-mcp

# Run with custom database URL
symbol-mcp --database-url postgres://username:password@localhost:5432/symbol_ontology

# Run with debug logging
symbol-mcp --verbose
```

### Integration with Cursor

To use Symbol Ontology with Cursor AI:

1. Start the MCP server locally:

   ```bash
   symbol-mcp
   ```

   The server will start at http://localhost:3000 by default.

2. In Cursor, open the settings:

   - Navigate to the "MCP" section

3. Add a new MCP provider:

   - Name: Symbol Ontology
   - URL: http://localhost:3000/sse
   - Authentication: None (or as required)

4. Save your settings

5. The symbol-ontology tools will now be available in Cursor AI.

### Running the API Server (Optional)

The API server provides REST endpoints if needed:

```bash
# Run from the project directory
cargo run -p ontology-api-server
```

## API Reference

### MCP Methods

| Method               | Description                | Parameters                                                                 |
| -------------------- | -------------------------- | -------------------------------------------------------------------------- |
| `get_symbols`        | List all symbols           | `limit` (optional): Maximum symbols to return                              |
| `search_symbols`     | Search symbols by text     | `query`: Search text<br>`limit` (optional): Maximum symbols to return      |
| `filter_by_category` | Filter symbols by category | `category`: Category name<br>`limit` (optional): Maximum symbols to return |
| `get_categories`     | List all categories        | None                                                                       |
| `get_symbol_sets`    | List all symbol sets       | `limit` (optional): Maximum sets to return                                 |
| `search_symbol_sets` | Search symbol sets         | `query`: Search text<br>`limit` (optional): Maximum sets to return         |

### REST API Endpoints

| Endpoint              | Method | Description                                  |
| --------------------- | ------ | -------------------------------------------- |
| `/api/v1/symbols`     | GET    | List all symbols or filter by category/query |
| `/api/v1/symbols/:id` | GET    | Get a specific symbol by ID                  |
| `/api/v1/symbols`     | POST   | Create a new symbol                          |
| `/api/v1/symbols/:id` | PUT    | Update a symbol                              |
| `/api/v1/symbols/:id` | DELETE | Delete a symbol                              |

## Upcoming Features

The following features are planned for future releases:

- Database seeding tool for easy initialization
- Improved SSE integration for the MCP client
- Authentication for the API server
- Rate limiting for the API server
- License validation
- Centralized logging setup
- Improved configuration system
- More comprehensive documentation

## License

This project is dual-licensed under:

- [Mozilla Public License 2.0](LICENSE.md) for non-commercial use
- [Commercial License](COMMERCIAL_LICENSE.md) for business/for-profit use

Please see the LICENSE files for details.

For commercial licensing inquiries, please contact [info@yogimathius.dev].
