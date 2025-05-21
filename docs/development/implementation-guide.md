# Symbol Ontology Implementation Guide

This guide provides developers with practical information for implementing features and contributing to the Symbol Ontology project.

## Development Environment Setup

### Prerequisites

- Rust (latest stable version)
- PostgreSQL (15+)
- Git

### Getting Started

1. **Clone the repository**:

```bash
git clone https://github.com/yourusername/symbol-ontology.git
cd symbol-ontology
```

2. **Install dependencies**:

```bash
# Install PostgreSQL (if not already installed)
# On macOS:
brew install postgresql

# On Ubuntu:
sudo apt install postgresql
```

3. **Set up the database**:

```bash
# Create a database
createdb symbol_ontology

# Initialize schema (uses sqlx migrations)
cargo run --bin setup-db
```

4. **Environment configuration**:

Create a `.env` file in the project root:

```
DATABASE_URL=postgres://username:password@localhost/symbol_ontology
PORT=8080
MCP_PORT=3200
RUST_LOG=info
```

## Project Structure

The Symbol Ontology project is organized as a Rust workspace with multiple crates:

```
symbol-ontology/
├── ontology-core/           # Core domain models and database logic
├── symbol-mcp-client/       # MCP server implementation
├── ontology-api-server/     # Optional REST API server
└── tests/                   # Integration tests
```

### Key Directories

- **ontology-core/src/domain/** - Core domain models (Symbol, SymbolSet, etc.)
- **ontology-core/src/db/** - Database repository implementations
- **symbol-mcp-client/src/mcp/** - MCP method implementations
- **symbol-mcp-client/src/main.rs** - MCP server entry point
- **seed-data/** - JSON files for seeding the database

## Common Development Tasks

### Running the Server

```bash
# Run the MCP server
cargo run --bin symbol-mcp-client

# Run the API server (if using)
cargo run --bin ontology-api-server
```

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p ontology-core
```

### Adding a New Symbol

Symbols can be added programmatically or via seed data:

#### Via Seed Data

1. Add a new entry to `seed-data/symbols.json`:

```json
{
  "id": "unique-id",
  "name": "Symbol Name",
  "category": "Category",
  "description": "Description of the symbol",
  "interpretations": {
    "dream": "Interpretation in dreams",
    "mythology": "Interpretation in mythology"
  },
  "related_symbols": ["related-id-1", "related-id-2"]
}
```

2. Run the seed script:

```bash
cargo run --bin seed-db
```

#### Programmatically

```rust
let symbol = Symbol {
    id: "unique-id".to_string(),
    name: "Symbol Name".to_string(),
    category: "Category".to_string(),
    description: "Description".to_string(),
    interpretations: HashMap::from([
        ("dream".to_string(), "Interpretation".to_string()),
    ]),
    related_symbols: vec!["related-id".to_string()],
    properties: HashMap::new(),
};

repository.create_symbol(symbol).await?;
```

## Implementing New Features

### Adding a New MCP Method

1. Create a new file in `symbol-mcp-client/src/mcp/methods/`:

```rust
// symbol-mcp-client/src/mcp/methods/my_new_method.rs

use rmcp::model::*;
use rmcp::tool;
use std::sync::Arc;

use crate::mcp::schema::MyNewMethodParams;
use ontology_core::db::repository::SymbolRepository;

pub fn my_new_method(repository: Arc<dyn SymbolRepository>) -> Box<dyn Handler> {
    Box::new(MyNewMethodHandler {
        repository,
    })
}

struct MyNewMethodHandler {
    repository: Arc<dyn SymbolRepository>,
}

#[tool]
impl MyNewMethodHandler {
    #[tool(description = "Description of the method")]
    async fn call(&self, params: MyNewMethodParams) -> Result<CallToolResult, rmcp::Error> {
        // Implementation here

        // Return results in CallToolResult format
        Ok(CallToolResult::new(serde_json::json!({
            "result": "value"
        })))
    }

    fn method_name(&self) -> &'static str {
        "my_new_method"
    }
}
```

2. Add the schema in `symbol-mcp-client/src/mcp/schema.rs`:

```rust
#[derive(Debug, Deserialize)]
pub struct MyNewMethodParams {
    pub param1: String,
    pub param2: Option<i32>,
}
```

3. Register the method in `symbol-mcp-client/src/mcp/service.rs`:

```rust
// Add import
use crate::mcp::methods::my_new_method::my_new_method;

// In the register_tools method, add:
tools.register(my_new_method(self.symbol_repository.clone()));
```

### Extending the Symbol Model

To extend the Symbol model with new fields:

1. Update the model in `ontology-core/src/domain/symbols.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: String,
    pub name: String,
    pub category: String,
    pub description: String,
    pub interpretations: HashMap<String, String>,
    pub related_symbols: Vec<String>,
    pub properties: HashMap<String, String>,
    // Add new fields here
    pub new_field: Option<String>,
}
```

2. Update the repository interfaces in `ontology-core/src/db/repository/interfaces.rs`
3. Update the implementations in `ontology-core/src/db/repository/`
4. Add migration if using PostgreSQL

## Best Practices

### Code Style

- Follow Rust's standard formatting (use `cargo fmt`)
- Run `cargo clippy` to catch common issues
- Add tests for all new functionality
- Document public APIs with rustdoc comments

### MCP Implementation

- Keep methods focused on a single responsibility
- Validate input parameters early
- Return structured, consistent JSON responses
- Include proper error handling and error codes
- Add method documentation in tool descriptions

### Database Access

- Use the repository pattern consistently
- Implement proper error handling
- Use transactions for multi-step operations
- Consider performance for large datasets

## Troubleshooting

### Common Issues

- **Database connection failures**: Check DATABASE_URL in .env
- **Missing tables**: Run migrations or setup-db script
- **MCP connection issues**: Verify MCP_PORT and check for port conflicts
- **Compilation errors**: Run `cargo check` for detailed error messages

### Logging

The project uses the `tracing` crate for logging. Set the log level with:

```
RUST_LOG=debug cargo run --bin symbol-mcp-client
```

### Getting Help

If you need assistance:

1. Check existing documentation in the `docs/` directory
2. Look for similar issues in the issue tracker
3. Reach out to the core team via the project's communication channels
