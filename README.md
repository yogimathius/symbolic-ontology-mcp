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
├── src/                     # Legacy code (being migrated)
└── tests/                   # Integration tests
```

## Getting Started

### Prerequisites

- Rust 1.75.0 or later
- PostgreSQL 15.0 or later (for production use)
- SQLite 3.35.0 or later (for development/testing)

### Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/symbol-ontology.git
cd symbol-ontology
```

### Development Setup

1. Install dependencies:

```bash
cargo build
```

2. Set up the database:

```bash
# Development with SQLite
export DATABASE_URL="sqlite:symbol_ontology.db"

# Or with PostgreSQL
export DATABASE_URL="postgres://username:password@localhost/symbol_ontology"
```

## Usage

### Running the API Server

Start the API server:

```bash
cargo run -p ontology-api-server
```

The server will start on http://localhost:8080 by default. You can access the API at http://localhost:8080/api/v1/symbols.

### Using the MCP Client

Start the MCP client to connect to the API server:

```bash
cargo run -p symbol-mcp-client
```

The client will start an MCP server on port 3000 by default, which connects to the API server at http://localhost:8080/api/v1.

### Seeding Data

Populate the database with symbols:

```bash
cargo run --bin ontology_seeder -- --test-data
```

## API Reference

### Endpoints

- `GET /api/v1/symbols` - List all symbols or filter by category/query
- `GET /api/v1/symbols/:id` - Get a specific symbol by ID
- `POST /api/v1/symbols` - Create a new symbol
- `DELETE /api/v1/symbols/:id` - Delete a symbol

### MCP Methods

- `get_symbols` - Get a list of symbols with optional filtering
  - Parameters: `query` (optional), `category` (optional)

## License

This project is dual-licensed under:

- [Mozilla Public License 2.0](LICENSE.md) for non-commercial use
- [Commercial License](COMMERCIAL_LICENSE.md) for business/for-profit use

Please see the LICENSE files for details.

For commercial licensing inquiries, please contact [info@yogimathius.dev].
