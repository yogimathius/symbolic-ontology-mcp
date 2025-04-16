# Dream Ontology: Symbolic Reasoning Engine

## Overview

Implement a Rust backend server using Axum that acts as a symbolic reasoning engine utilizing Model Context Protocol (MCP) via the official Rust MCP SDK (rmcp).

## Core Requirements

### Framework & Tools

- **Axum** for HTTP endpoints
- **RMCP** (official Rust MCP SDK) to expose and consume MCP-compliant endpoints
- **Tokio** for async runtime
- **Serde** for serialization/deserialization
- **SQLx** with PostgreSQL (optional for MVP) or in-memory storage for initial prototyping

### Symbolic Reasoning Domain

- Implement structured ontologies of symbols (dream, mythological, archetypal)
- Provide an MCP method (`get_symbols`) allowing external MCP clients to query symbolic data
- ~~Implement an endpoint (`POST /interpret`) that accepts symbolic input and returns an LLM-generated, MCP-grounded interpretation~~

> **Architecture Note:** This repository implements only the MCP server providing symbolic data. The interpretation endpoint that uses LLM integration will be implemented in a separate MCP client service. This separation follows best practices for MCP architecture, keeping the data server focused on providing accurate information while allowing various clients to handle interpretation as needed.

### ~~Prompt Templating for LLM~~

> **Architecture Note:** Prompt templating and LLM integration are not part of this service. These features will be implemented in a separate MCP client service for dream interpretation.

### Protocol & Schema

- Define clear MCP-compliant JSON schemas for both input requests and responses
- Provide extensibility mechanisms to plug in additional symbolic ontologies via MCP endpoints

## 🎯 Deliverables

### Code Components

- Axum handlers for HTTP endpoints
- MCP handlers using rmcp SDK
- Ontology schema (SymbolSet, Symbol)
- ~~LLM prompt builders~~
- MCP server demonstration setup
- Error handling and structured logging

## TDD Guardrails

### Testing Framework

- Use `tokio-test` for async test support
- Implement unit tests with `#[tokio::test]` for all handlers and business logic
- Use `axum-test` for integration testing of HTTP endpoints

### Development Workflow

1. Write failing test first
2. Implement minimal code to pass test
3. Refactor while maintaining test pass
4. Run `cargo test` after each meaningful change

### Continuous Integration

- Set up GitHub Actions to run tests on each PR
- Enforce clippy lints with `cargo clippy -- -D warnings`
- Maintain code documentation with `cargo doc --no-deps`

### Quality Controls

- Achieve minimum 80% test coverage
- Run `cargo fmt` before commits
- Use `cargo check` frequently during development
- Create mock implementations for external dependencies (~~OpenRouter,~~ databases)

### Testing Categories

1. Unit tests for domain models and business logic
2. Integration tests for HTTP endpoints
3. Schema validation tests for MCP protocol compliance
4. ~~End-to-end tests with mock LLM responses~~

## Cursor IDE Guidelines

### Development Experience

- Use Cursor's "Jump to Definition" to navigate between types and implementations
- Utilize inline test execution for faster feedback cycles
- Create module-specific test files alongside source files
- Leverage Cursor's code suggestions for idiomatic Rust patterns

### Automation Commands

- Set up a command palette shortcut for running `cargo check`
- Configure keyboard shortcuts for test execution
- Use split-pane view to see tests and implementation side by side
- Create snippets for common test patterns (arrange-act-assert)

## Project Structure

### Directory Organization

```
src/
├── api/            # HTTP API handlers
│   ├── routes.rs   # Route definitions
│   └── handlers/   # Request handlers
├── mcp/            # MCP protocol implementation
│   ├── schema.rs   # MCP schema definitions
│   └── methods/    # MCP method implementations
├── domain/         # Core business logic
│   ├── symbols.rs  # Symbol definitions
│   └── ontology.rs # Ontology implementations
├── utils/          # Shared utilities
└── main.rs         # Application entry point
```

> **Note:** The `llm/` directory is intentionally omitted as LLM integration is implemented in a separate MCP client service.

### Module Boundaries

- Enforce clear separation between API, domain, and infrastructure concerns
- Create well-defined interfaces between modules
- Use feature flags for optional components
- Document module responsibilities in module-level comments

### Dependency Management

- Organize Cargo.toml dependencies into features
- Pin dependency versions for reproducible builds
- Document reasons for non-standard dependencies
- Create separate dev-dependencies section for testing tools
