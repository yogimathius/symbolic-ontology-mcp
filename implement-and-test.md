# Consolidated Cleanup and Testing Checklist

## Unused Imports Cleanup

- [x] `db/mod.rs`: Remove unused imports:
  - [x] `pool::create_pool`
  - [x] `models::{Symbol, SymbolSet}`
- [x] `db/repository/mod.rs`: Remove unused imports:
  - [x] `interfaces::{RepositoryResult, SymbolSetRepository}` in conditional re-export
- [x] `api/handlers.rs`: Remove unused import:
  - [x] `axum::http::StatusCode`
- [x] `mcp/methods/filter_by_category.rs`: Remove unused import:
  - [x] No `super::*` in tests module found (already clean)
- [x] `mcp/methods/search_symbols.rs`: Remove unused import:
  - [x] `super::*` in tests module
- [x] `mcp/methods/get_symbols_tests.rs`: Remove unused import:
  - [x] No such file exists (directory `tests/mcp` not found)
- [x] `logging.rs`: Remove unused imports:
  - [x] `tracing_subscriber::layer::SubscriberExt`
  - [x] `tracing_subscriber::util::SubscriberInitExt`
- [x] `main.rs`: Remove unused imports:
  - [x] `std::sync::Arc`
  - [x] `rmcp::ServerImpl`
  - [x] `crate::mcp::service::SymbolService`

## Symbol Model Methods Implementation and Testing

- [x] `db/models.rs`: Implement and test the following methods:
  - [x] `add_interpretation(context: String, interpretation: String)` with validation tests
  - [x] `add_related_symbol(symbol_id: String)` with edge case tests

## Repository Pattern Implementation and Testing

### Symbol Repository Interface

- [x] `db/repository/interfaces.rs`: Implement and test the following methods:
  - [x] `get_symbol(id: &str)` - Add integration tests with real database
  - [x] `create_symbol(symbol: Symbol)` - Add concurrent operation tests
  - [x] `update_symbol(symbol: Symbol)` - Add validation tests
  - [x] `delete_symbol(id: &str)` - Add tests for non-existent symbols

### Symbol Set Repository Interface

- [x] `db/repository/interfaces.rs`: Implement and test the following methods:
  - [x] `get_symbol_set(id: &str)` - Add integration tests
  - [x] `list_symbol_sets(category: Option<&str>)` - Add pagination tests
  - [x] `search_symbol_sets(query: &str)` - Add complex search tests
  - [x] `create_symbol_set(symbol_set: SymbolSet)` - Add validation tests
  - [x] `update_symbol_set(symbol_set: SymbolSet)` - Add concurrent operation tests
  - [x] `delete_symbol_set(id: &str)` - Add error condition tests
  - [x] `create_symbol_set_repository()` - Add unit tests

### Repository Factory Implementation

- [x] `db/repository/factory.rs`: Implement and test the following methods:
  - [x] `PgRepositoryFactory::new()` - Add unit tests
  - [x] `create_symbol_repository()` - Add unit tests
  - [x] `create_symbol_set_repository()` - Add unit tests

### Repository Implementation

- [x] `db/repository/symbol_repository.rs`: Implement and test:
  - [x] `PgSymbolRepository::new()` - Add unit tests

## MCP Method Handlers Implementation and Testing

- [ ] `mcp/methods/filter_by_category.rs`: Implement registration and tests:

  - [ ] `FilterByCategoryHandler::new()` - Add unit tests
  - [ ] `filter_by_category(symbol_repository)` factory function - Add unit tests
  - [ ] Add edge case tests for this MCP method

- [ ] `mcp/methods/search_symbols.rs`: Implement registration and tests:
  - [ ] `SearchSymbolsHandler::new()` - Add unit tests
  - [ ] `search_symbols(symbol_repository)` factory function - Add unit tests
  - [ ] Add comprehensive parameter validation tests

## Logging

- [x] `logging.rs`: Clean up unused imports:
  - [x] `tracing_subscriber::layer::SubscriberExt`
  - [x] `tracing_subscriber::util::SubscriberInitExt`

## Symbol Set Repository Implementation

- [x] Complete the Symbol Set functionality with tests:
  - [x] Update `src/db/repository/mod.rs` to properly export SymbolSetRepository
  - [x] Implement and test the SymbolSetRepository interface with proper DB queries

## MCP Architecture Decision

- [x] Decide on MCP Architecture Approach and add appropriate tests:
  - [x] Use RMCP-based approach in `src/mcp/service.rs` with comprehensive tests
  - [x] Fix main.rs to remove MCP server integration (moved to mcp_server.rs)
  - [x] Update error handling in api/error.rs to use rmcp::Error directly

## Implementation With Tests

- [ ] Implement the full MCP tool registration system with tests:

  - [ ] `filter_by_category` - Add edge case tests
  - [ ] `search_symbols` - Add comprehensive parameter validation
  - [ ] Add tests for duplicate functionality between implementations

- [ ] Implement Symbol Set functionality throughout the application with tests:

  - [ ] Complete the Symbol Set repository pattern with thorough tests
  - [ ] Add MCP methods for querying and managing Symbol Sets with tests
  - [ ] Create REST API endpoints for Symbol Set management with tests

- [ ] Implement proper validation for all MCP methods with tests:

  - [ ] Add parameter validation with detailed error messages tests
  - [ ] Add pagination support for large result sets with tests

- [ ] Update end-to-end tests for all MCP tools
