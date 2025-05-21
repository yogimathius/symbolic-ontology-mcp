# Ontology Core

The core library for the Symbol Ontology project, containing domain models, database interfaces, and shared utilities.

## Purpose

This crate serves as the foundation for both the API server and MCP client, providing:

1. **Domain Models** - Core data structures for symbols and symbol sets
2. **Database Access** - Repository interfaces and implementations
3. **Shared Utilities** - Helper functions used across the project

## Features

### Domain Models

- `Symbol` - Represents a symbolic entity with properties:

  - `id` - Unique identifier
  - `name` - Human-readable name
  - `category` - Classification category
  - `description` - Detailed description
  - `interpretations` - Multiple context-specific meanings
  - `related_symbols` - References to related symbols
  - `properties` - Extensible key-value pairs

- `SymbolSet` - A collection of related symbols:
  - `id` - Unique identifier
  - `name` - Human-readable name
  - `category` - Classification category
  - `description` - Detailed description
  - `symbols` - Map of symbol IDs to symbol objects

### Repository Layer

- Repository interfaces with clean separation of concerns:

  - `SymbolRepository` - Operations for symbol data
  - `SymbolSetRepository` - Operations for symbol set data

- PostgreSQL implementations:
  - `PgSymbolRepository` - PostgreSQL-backed symbol repository
  - `PgSymbolSetRepository` - PostgreSQL-backed symbol set repository

### Database Utilities

- Connection pool management
- Schema definitions
- Migration utilities
- SQL query helpers

## Usage

This crate is not meant to be used directly by end users. Instead, it's a dependency for:

- `symbol-mcp-client` - The public MCP client
- `ontology-api-server` - The REST API server

### For Developers

If you're working on the Symbol Ontology project, you'll use this crate as follows:

```rust
// Import domain models
use ontology_core::domain::{Symbol, SymbolSet};

// Import repository interfaces
use ontology_core::db::repository::{SymbolRepository, SymbolSetRepository};

// Create a database connection pool
use ontology_core::db::pool;
let pool = pool::create_pool("postgres://user:pass@localhost/symbol_ontology").await?;

// Create repositories
use ontology_core::db::repository::{PgSymbolRepository, PgSymbolSetRepository};
let symbol_repo = PgSymbolRepository::new(pool.clone());
let symbol_set_repo = PgSymbolSetRepository::new(pool.clone());

// Use repositories
let symbols = symbol_repo.list_symbols(None).await?;
```

## Architecture

The crate is organized as follows:

```
ontology-core/
├── src/
│   ├── domain/          # Domain models
│   │   ├── symbols.rs   # Symbol model
│   │   ├── ontology.rs  # SymbolSet model
│   │   └── mod.rs       # Module exports
│   ├── db/              # Database layer
│   │   ├── repository/  # Repository interfaces and implementations
│   │   ├── models.rs    # Database models
│   │   ├── pool.rs      # Connection pool management
│   │   ├── queries.rs   # SQL query helpers
│   │   ├── schema.rs    # Database schema definitions
│   │   └── mod.rs       # Module exports
│   └── lib.rs           # Library entry point
└── tests/               # Unit and integration tests
```

## Dependencies

- `sqlx` - Async SQL toolkit
- `serde` - Serialization/deserialization
- `tokio` - Async runtime
- `async-trait` - Async trait support

## Note

This crate is part of the Symbol Ontology project and is meant to be used in conjunction with the other crates in the workspace. It is not designed to be used as a standalone library.
