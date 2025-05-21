# Ontology API Server

A REST API server for the Symbol Ontology project that provides HTTP endpoints for managing symbols and symbol sets.

## Overview

The Ontology API Server provides a REST interface for:

- Retrieving symbols and symbol sets
- Creating new symbols and symbol sets
- Updating existing symbols and symbol sets
- Deleting symbols and symbol sets
- Searching and filtering the symbol database

This server is optional for using the Symbol Ontology project, as the MCP client can connect directly to the database. The API server is useful for web applications, integrations, or other scenarios where a REST API is preferred.

## Features

- **REST API** - Standard HTTP endpoints with JSON responses
- **CRUD Operations** - Complete Create, Read, Update, Delete operations
- **Search & Filter** - Query and filter symbols by various criteria
- **Database Backend** - Uses the same database as the MCP client
- **Axum Framework** - Built on the performant Axum web framework

## Running the Server

From the workspace root:

```bash
# Run the API server
cargo run -p ontology-api-server

# Run with custom port
cargo run -p ontology-api-server -- --port 8080

# Run with custom database URL
cargo run -p ontology-api-server -- --database-url postgres://username:password@localhost:5432/symbol_ontology

# Run with increased verbosity
cargo run -p ontology-api-server -- --verbose
```

### Environment Variables

You can also configure using environment variables:

- `PORT` - The port to listen on (default: 8080)
- `DATABASE_URL` - PostgreSQL connection string
- `LOG_LEVEL` - Logging level (info, debug, trace)
- `CORS_ORIGINS` - Allowed CORS origins (comma-separated)

## API Endpoints

The API is versioned and all endpoints are prefixed with `/api/v1`.

### Symbols

| Method | Endpoint                             | Description                                |
| ------ | ------------------------------------ | ------------------------------------------ |
| GET    | `/api/v1/symbols`                    | List all symbols (with optional filtering) |
| GET    | `/api/v1/symbols/:id`                | Get a specific symbol by ID                |
| POST   | `/api/v1/symbols`                    | Create a new symbol                        |
| PUT    | `/api/v1/symbols/:id`                | Update an existing symbol                  |
| DELETE | `/api/v1/symbols/:id`                | Delete a symbol                            |
| GET    | `/api/v1/symbols/search`             | Search symbols by text                     |
| GET    | `/api/v1/symbols/category/:category` | Get symbols by category                    |

### Symbol Sets

| Method | Endpoint                     | Description                   |
| ------ | ---------------------------- | ----------------------------- |
| GET    | `/api/v1/symbol-sets`        | List all symbol sets          |
| GET    | `/api/v1/symbol-sets/:id`    | Get a specific symbol set     |
| POST   | `/api/v1/symbol-sets`        | Create a new symbol set       |
| PUT    | `/api/v1/symbol-sets/:id`    | Update an existing symbol set |
| DELETE | `/api/v1/symbol-sets/:id`    | Delete a symbol set           |
| GET    | `/api/v1/symbol-sets/search` | Search symbol sets by text    |

### Categories

| Method | Endpoint             | Description                   |
| ------ | -------------------- | ----------------------------- |
| GET    | `/api/v1/categories` | List all available categories |

## Request & Response Examples

### List Symbols

**Request:**

```
GET /api/v1/symbols?limit=10
```

**Response:**

```json
{
  "symbols": [
    {
      "id": "water",
      "name": "Water",
      "category": "element",
      "description": "Symbol of emotion and life",
      "related_symbols": ["ocean", "rain"]
    }
    // More symbols...
  ],
  "total_count": 150
}
```

### Create Symbol

**Request:**

```
POST /api/v1/symbols
Content-Type: application/json

{
  "id": "tree",
  "name": "Tree",
  "category": "nature",
  "description": "Symbol of growth and connection",
  "related_symbols": ["forest", "roots"]
}
```

**Response:**

```json
{
  "id": "tree",
  "name": "Tree",
  "category": "nature",
  "description": "Symbol of growth and connection",
  "related_symbols": ["forest", "roots"],
  "interpretations": {},
  "properties": {}
}
```

## Architecture

The API server is structured as follows:

```
ontology-api-server/
├── src/
│   ├── api/              # API implementation
│   │   ├── error.rs      # Error handling
│   │   ├── handlers.rs   # Request handlers
│   │   ├── mod.rs        # Module exports
│   │   └── models.rs     # API-specific models
│   ├── routes/           # Route definitions
│   │   ├── symbols.rs    # Symbol routes
│   │   └── mod.rs        # Module exports
│   ├── config.rs         # Server configuration
│   ├── logging.rs        # Logging setup
│   └── main.rs           # Server entry point
└── tests/                # API tests
```

## Upcoming Features

The following features are planned for future releases:

- Authentication and authorization
- Rate limiting
- Advanced filtering and pagination
- GraphQL endpoint
- Websocket support for real-time updates

## Development

### Running Tests

```bash
cargo test -p ontology-api-server
```

### Building for Release

```bash
cargo build --release -p ontology-api-server
```

## License

This project is dual-licensed under:

- [Mozilla Public License 2.0](../LICENSE.md) for non-commercial use
- [Commercial License](../COMMERCIAL_LICENSE.md) for business/for-profit use
