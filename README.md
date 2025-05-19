# Dream Ontology MCP Server

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
- Use `get_categories` to retrieve all available categories

### Available MCP Methods

#### 1. `search_symbols`

**Description:** Search for symbols matching a text query.

**Parameters:**

- `query` (required): Text to search for in symbol names and descriptions
- `limit` (optional): Maximum number of results to return (default: 50)

**Example:**

```
// Search for symbols related to water
search_symbols(query: "water", limit: 10)
```

#### 2. `filter_by_category`

**Description:** Get symbols belonging to a specific category.

**Parameters:**

- `category` (required): Category name to filter by
- `limit` (optional): Maximum number of results to return (default: 50)

**Example:**

```
// Get symbols in the "nature" category
filter_by_category(category: "nature", limit: 10)
```

#### 3. `get_symbols`

**Description:** List all symbols (without filtering).

**Parameters:**

- `limit` (optional): Maximum number of results to return (default: 50)

**Example:**

```
// Get all symbols
get_symbols(limit: 20)
```

#### 4. `get_categories`

**Description:** Get a list of all available symbol categories.

**Parameters:** None

**Example:**

```
// Get all available categories
get_categories()
```

### Response Format

All symbol-related endpoints return responses in the following format:

```json
{
  "symbols": [
    {
      "id": "water",
      "name": "Water",
      "category": "nature",
      "description": "Symbolizes emotions and the unconscious",
      "related_symbols": ["ocean", "river", "rain"]
    }
    // Additional symbols...
  ],
  "total_count": 10
}
```

For `get_categories`, the response format is:

```json
{
  "categories": ["animal", "nature", "mythological", "jungian"],
  "count": 4
}
```

## MCP Integration with Clients

The MCP server can be used with Claude Desktop or any other MCP-compatible client.

### Using the Deployed Server

To use the production MCP server deployed at `symbolic-grounding-api.fly.dev`:

1. Add this to your MCP client configuration:

```json
{
  "tool_servers": [
    {
      "name": "DreamOntology",
      "transport": {
        "type": "sse",
        "url": "https://symbolic-grounding-api.fly.dev"
      }
    }
  ]
}
```

### Using a Local Server

To configure Claude Desktop to use a local MCP server:

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

## Testing

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test domain::symbols
```

### Test Coverage

The project uses cargo-tarpaulin for test coverage analysis.

#### Prerequisites

Install cargo-tarpaulin:

```bash
cargo install cargo-tarpaulin
```

#### Running Coverage Reports

Run the coverage script to generate HTML and XML reports:

```bash
# Generate coverage reports
./scripts/coverage.sh
```

This will:

- Generate HTML reports in the `coverage` directory
- Generate XML reports for CI integration
- Set a minimum coverage threshold of 70%

## Database Seeding

The project includes database seeding tools to populate your database with dream symbols:

```bash
# Seed with the sample dataset
cargo run --bin ontology_seeder data/sample_dream_symbols.csv

# Or seed with your own dataset
cargo run --bin ontology_seeder path/to/your/csv
```

## Deployment

The MCP server is deployed to [fly.io](https://fly.io) using the configuration in `fly.toml`. Key configuration aspects include:

- HTTP service on port 3002
- CORS configuration allowing cross-origin requests
- Health checks via TCP
- Auto-scaling configuration

To deploy your own instance:

1. Install the [flyctl](https://fly.io/docs/hands-on/install-flyctl/) command line tool
2. Log in to fly.io: `flyctl auth login`
3. Deploy the application: `flyctl deploy`

## Future Milestones

While the project is feature complete for v1.5/2, several areas for future improvement have been identified:

### ✅ Code Cleanup & Error Handling (Completed)

- ✅ Standardized error handling across MCP methods
- ✅ Refactored duplicated code in MCP method handlers
- ✅ Improved documentation consistency across modules
- ✅ Enhanced logging for better debugging and monitoring

### Feature Extensions

- Complete vector embedding support for semantic search
- Implement proper full-text search capabilities
- Add caching mechanism for frequently accessed symbols
- Enhance the repository pattern with more sophisticated queries

### Performance and Scalability

- Optimize connection pooling configuration
- Implement caching for frequently accessed symbols
- Enhance full-text search capabilities
- Add metrics and observability for production monitoring

### Security

- Implement authentication/authorization for API endpoints
- Add rate limiting for API protection
- Create versioning strategy for the API

## License

This project is licensed under the MIT License - see the LICENSE file for details.
