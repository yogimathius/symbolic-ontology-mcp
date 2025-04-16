# Dream Ontology MCP Server

A symbolic reasoning engine that provides ontological data about symbols via the Model Context Protocol (MCP).

## Purpose

This project implements a specialized MCP server that focuses on providing structured symbolic data for use in applications that work with dreams, mythology, and archetypes. It acts as a "source of truth" for symbolic meaning and relationships, allowing MCP clients to retrieve accurate symbolic information.

## Architecture

The Dream Ontology MCP Server follows a clean, layered architecture:

- **Domain Layer**: Core business logic and entities (Symbol, SymbolSet)
- **Repository Layer**: Data access interfaces and implementations
- **API Layer**: HTTP endpoints powered by Axum
- **MCP Layer**: Protocol handlers adhering to the MCP specification

### Key Components

- **Axum Web Framework**: Powers the HTTP API and MCP endpoints
- **RMCP SDK**: Rust implementation of the MCP protocol
- **In-memory Repository**: Stores and retrieves symbolic data (with PostgreSQL option for production)
- **Tower Middleware**: Provides robust request processing and testing capabilities

## Separation of Concerns

This repository implements **only the symbolic data server** component. The LLM integration for symbolic interpretation is implemented in a separate MCP client service. This intentional separation follows best practices for MCP implementations:

1. **This repository (MCP Server)**:

   - Provides accurate symbolic data via MCP methods
   - Acts as the "source of truth" for ontological information
   - Focuses on data integrity, performance, and MCP protocol compliance

2. **Separate MCP Client (not in this repo)**:
   - Consumes data from this MCP server
   - Handles LLM integration via OpenRouter
   - Implements prompt engineering and interpretation
   - Manages the actual dream interpretation logic

This separation allows each service to excel at its specific responsibility while maintaining a clean architecture.

## Getting Started

[Instructions to be added]

## Project Status

This project is currently in active development. See [updated-checklist.md](updated-checklist.md) for current status and roadmap.

## License

[License information to be added]
