# Symbol Ontology Technical Stack

## Backend Architecture Components

The Symbol Ontology backend is built on a modern Rust stack with the following key components:

### Core Technologies

- **Rust** - Primary programming language
- **Axum** - Web framework for HTTP API and WebSockets
- **RMCP** - Rust Model Context Protocol SDK
- **Tokio** - Asynchronous runtime
- **SQLx** - Database access library (PostgreSQL)
- **Serde** - Serialization/deserialization for JSON

### System Components

The system is divided into two distinct backend services:

#### 1. Symbol Ontology MCP Server (This Repository)

**Purpose**: Serves as the symbolic knowledge repository and MCP server.

**Key Components**:

- **Ontology Core** - Domain models and core symbolic logic
- **MCP Implementation** - JSON-RPC endpoints serving symbol data via MCP
- **Repository Layer** - Data access abstractions for symbol storage

**Data Flow**:

- Receives MCP requests for symbolic data
- Processes requests using the repository layer
- Returns structured symbolic data via MCP responses

#### 2. Dream Interpretation Backend (Separate Repository)

**Purpose**: Serves as the user-facing backend for dream interpretation.

**Key Components**:

- **API Server** - REST endpoints for user interaction
- **LLM Integration** - Connections to external LLMs via OpenRouter
- **MCP Client** - Queries the Symbol Ontology MCP Server

**Data Flow**:

- Receives user dream inputs via REST API
- Fetches relevant symbolic data via MCP
- Constructs prompts with symbolic context
- Sends prompts to LLM and returns interpretations

## Crate Structure

The project is organized as a Rust workspace with the following crates:

- **ontology-core** - Core domain models and shared logic
- **symbol-mcp-client** - MCP client implementation
- **ontology-api-server** - REST API server (optional)

## Database Schema

The system uses PostgreSQL with the following key tables:

- **symbols** - Stores symbolic entities with their properties
- **symbol_sets** - Groups of related symbols
- **categories** - Classification categories for symbols

## MCP Endpoints

The Symbol Ontology MCP Server implements these primary methods:

- **get_symbols** - Retrieve symbols with optional filtering
- **search_symbols** - Search for symbols by text query
- **filter_by_category** - Filter symbols by category
- **get_symbol_sets** - Retrieve sets of related symbols
- **search_symbol_sets** - Search for symbol sets

## Development Environment

- **Cargo** - Rust package manager
- **PostgreSQL** - Local or Docker-based database
- **Postman/curl** - API testing tools
- **Cursor** - Recommended IDE with MCP integration
