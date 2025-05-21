# Integration Tests

This directory contains integration tests for the Symbol Ontology project. These tests verify the correct interaction between different components and ensure the project works as a whole.

## Directory Structure

```
tests/
├── db/                   # Database-related tests
│   └── repository/       # Repository implementation tests
├── mcp/                  # MCP-related tests
│   └── service_tests.rs  # Tests for MCP service functionality
└── lib.rs                # Main test entry point
```

## Running the Tests

To run all tests from the workspace root:

```bash
# Run all tests in the workspace
cargo test --workspace

# Run only integration tests
cargo test --test integration

# Run with MCP client local feature enabled for in-memory repositories
cargo test --workspace --features symbol-mcp-client/local
```

## Test Categories

### Repository Tests

Tests for database repositories, including:

- Symbol repository CRUD operations
- Symbol set repository CRUD operations
- Error handling and validation

### MCP Service Tests

Tests for the Model Context Protocol service, including:

- Service initialization
- MCP method implementations
- Error handling and validation

## Writing New Tests

When adding new integration tests:

1. Place tests in the appropriate subdirectory
2. Use the `#[tokio::test]` attribute for async tests
3. Use mock repositories when possible to avoid database dependencies
4. Keep tests independent and isolated

## Migration Status

Many tests are being migrated from this directory to their respective projects as part of the ongoing refactoring.

The migration script (`migrate-tests.sh`) helps move tests to the appropriate location:

```bash
# Run the migration script
./migrate-tests.sh
```

## Note

This test directory is transitional and will eventually be reduced as tests are moved to their respective crates. New tests should be added directly to the relevant crate's test directory.
