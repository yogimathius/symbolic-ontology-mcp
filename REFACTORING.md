# Symbol Ontology Refactoring Plan

## Overview

This document outlines the plan to refactor the Symbol Ontology project into a multi-crate workspace with:

- A private core library (`ontology-core`)
- A private API server (`ontology-api-server`)
- A public MCP client (`symbol-mcp-client`) (renamed from `symbol-mcp-client`)

## Goals

- Separate concerns between API logic and client interface
- Keep business logic and database interactions private
- Provide a minimal public client for MCP integration
- Support license verification and tiered access
- Make the client easily installable via `cargo install`
- Remove all dream-specific terminology to make this a pure symbolic ontology

## Implementation Checklist

### Phase 1: Setup Workspace Structure

- [x] Update main `Cargo.toml` to define workspace
- [x] Create directory structure for the three crates
- [x] Create initial `Cargo.toml` files for each crate
- [x] Define dependencies and ensure `publish = false` for private crates
- [x] Create placeholder source files for all crates
- [x] Test building the workspace structure

### Phase 2: Core Library Implementation

- [x] Move domain models to `ontology-core/src/domain/`
- [x] Move database logic to `ontology-core/src/db/`
- [x] Move shared utilities to `ontology-core/src/utils/`
- [x] Create public API for the core library
- [ ] Add tests to ensure functionality is preserved

### Phase 3: API Server Implementation

- [x] Create basic API server in `ontology-api-server`
- [x] Add dependency on `ontology-core`
- [ ] Move API routes and handlers from `src/api/` to `ontology-api-server`
- [ ] Implement authentication mechanism
- [ ] Add license validation endpoint
- [ ] Implement tiered rate limiting

### Phase 4: MCP Client Implementation

- [x] Create minimal MCP client binary in `symbol-mcp-client` (renamed)
- [x] Implement connection to API service
- [ ] Move MCP methods and service logic from `src/mcp/` to `symbol-mcp-client`
- [ ] Add license verification
- [ ] Create MCP service interface
- [ ] Document client usage

### Phase 5: Testing & Documentation

- [ ] Migrate tests to new structure
- [ ] Document installation process
- [ ] Create example configurations
- [x] Update README with new structure information

### Phase 6: Deployment & Release

- [ ] Deploy API server with new structure
- [ ] Publish client crate to cargo
- [ ] Update documentation with installation instructions

### Phase 7: Terminology Refactoring

- [x] Rename `dream-mcp-client` directory to `symbol-mcp-client`
- [x] Update all imports and references in code
- [x] Rename all dream-specific struct names and variables
- [x] Change all default category values from "dream" to "symbol" or "general"
- [x] Update documentation to remove dream-specific terminology
- [x] Update function and method names that reference dreams
- [x] Revise test fixtures to use general symbol examples
- [x] Rename imports and crate references
- [x] Update CLI help text and environment variable names

## Migration Strategy

1. Keep existing code intact until new structure is tested
2. Gradually move functionality to new crates
3. Test each component thoroughly after migration
4. Switch to new structure only when everything is working
5. Remove deprecated code after successful migration
6. Complete terminology refactoring as a final step

## Current Status and Next Steps

1. ✅ Workspace structure has been set up
2. ✅ Domain models have been moved to `ontology-core`
3. ✅ Database models and interfaces have been moved to `ontology-core`
4. ✅ Basic API server framework has been set up
5. ✅ Symbol-mcp-client has been created and renamed from dream-mcp-client
6. ✅ All dream-specific terminology has been updated to symbol-centric terms

### Remaining Tasks (In Order)

1. Move API logic from `src/api/` to `ontology-api-server/src/`
2. Move MCP logic from `src/mcp/` to appropriate locations (`symbol-mcp-client` or `ontology-core`)
3. Review and migrate binaries in `src/bin/`
4. Clean up remaining root files (`src/main.rs`, `src/lib.rs`, etc.)
5. Update tests to reference new crate structure
6. Finalize documentation and examples
