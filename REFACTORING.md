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

- [ ] Move domain models to `ontology-core/src/domain/`
- [ ] Move database logic to `ontology-core/src/db/`
- [ ] Move shared utilities to `ontology-core/src/utils/`
- [ ] Create public API for the core library
- [ ] Add tests to ensure functionality is preserved

### Phase 3: API Server Implementation

- [ ] Create API server in `ontology-api-server`
- [ ] Add dependency on `ontology-core`
- [ ] Implement authentication mechanism
- [ ] Add license validation endpoint
- [ ] Implement tiered rate limiting

### Phase 4: MCP Client Implementation

- [ ] Create minimal MCP client binary in `symbol-mcp-client` (renamed)
- [ ] Implement connection to API service
- [ ] Add license verification
- [ ] Create MCP service interface
- [ ] Document client usage

### Phase 5: Testing & Documentation

- [ ] Test client against live API
- [ ] Document installation process
- [ ] Create example configurations
- [ ] Update README with new structure information

### Phase 6: Deployment & Release

- [ ] Deploy API server with new structure
- [ ] Publish client crate to cargo
- [ ] Update documentation with installation instructions

### Phase 7: Terminology Refactoring

- [ ] Rename `symbol-mcp-client` directory to `symbol-mcp-client`
- [ ] Update all imports and references in code
- [ ] Rename all dream-specific struct names and variables
- [ ] Change all default category values from "dream" to "symbol" or "general"
- [ ] Update documentation to remove dream-specific terminology
- [ ] Update function and method names that reference dreams
- [ ] Revise test fixtures to use general symbol examples
- [ ] Rename imports and crate references
- [ ] Update CLI help text and environment variable names

## Migration Strategy

1. Keep existing code intact until new structure is tested
2. Gradually move functionality to new crates
3. Test each component thoroughly after migration
4. Switch to new structure only when everything is working
5. Remove deprecated code after successful migration
6. Complete terminology refactoring as a final step

## Next Steps

1. Begin moving domain models to the core library
2. Extract database logic from main crate
3. Implement basic API server with authentication
4. Develop MCP client with API integration
5. Start terminology refactoring in parallel
