## Revised Foundational Building Checklist

### 1. Improve Testing Foundation (FIRST PRIORITY)

- [x] **Refactor Tests into Separate Modules**

  - [x] Create `tests/` directory with proper structure
    - [x] `tests/domain/` for testing domain models
    - [x] `tests/api/` for testing API endpoints
    - [x] `tests/mcp/` for testing MCP handlers
    - [x] `tests/llm/` for testing LLM integration
  - [x] Move unit tests from in-file to dedicated test modules
  - [x] Create shared test utilities and fixtures

- [x] **Create Test Fixtures and Helpers**

  - [x] Implement test data factory functions
  - [ ] Create mock repositories for testing handlers
  - [x] Add test context setup/teardown utilities

- [x] **Expand Test Coverage**
  - [x] Add tests for current placeholder functionality
  - [x] Write tests for error cases and edge conditions
  - [x] Create integration tests between components

### 2. Complete Core Domain Implementation

- [x] **Finalize Symbol and SymbolSet models**

  - [x] Review and validate the current fields against requirements
  - [x] Add any missing validators or business logic
  - [x] Ensure proper error handling in methods

- [x] **Create a Repository Trait and Implementation**
  - [x] Define `SymbolRepository` trait with CRUD operations
  - [x] Implement in-memory version for development/testing
  - [x] Add proper error types for repository operations

### 3. Improve Error Handling

- [x] **Create Domain-Specific Error Types**

  - [x] Define error enum with meaningful variants
  - [x] Implement proper error conversion traits
  - [x] Add context to errors for better debugging

- [x] **Implement Error Mapping**
  - [x] Map domain errors to API responses
  - [x] Map repository errors to service errors
  - [x] Add proper HTTP status codes to API errors

### 4. Connect Components and Remove Placeholders

- [x] **Complete MCP Handler Implementation**

  - [x] Inject repository into `GetSymbolsHandler`
  - [x] Implement actual symbol retrieval logic
  - [x] Add proper error mapping to MCP protocol errors

- [ ] **Complete API Handlers**

  - [ ] Connect API handlers to repositories
  - [ ] Implement proper response mapping
  - [ ] Add validation for incoming requests

- [ ] **Complete LLM Client**
  - [ ] Implement real HTTP calls to OpenRouter
  - [ ] Add proper request/response handling
  - [ ] Implement error handling for API failures

### 5. Add Missing Infrastructure

- [ ] **Implement Configuration Management**

  - [ ] Create configuration struct for app settings
  - [ ] Add environment variable loading
  - [ ] Set up defaults for development

- [ ] **Enhance Logging**
  - [ ] Add structured logging for important operations
  - [ ] Configure appropriate log levels
  - [ ] Add request/response logging for debugging

### 6. Create Initial Symbol Dataset

- [x] **Develop a Basic Symbol Dataset**

  - [x] Create a small set of well-documented symbols (10-15)
  - [x] Include sample interpretations in multiple contexts
  - [x] Add relationships between symbols for testing

- [x] **Implement Data Loading**
  - [x] Create a loader for seed data
  - [x] Add initialization in main.rs
  - [x] Create test fixtures from this data

### 7. Code Quality Improvements

- [x] **Fix Unused Code**

  - [x] Remove or implement unused fields and methods
  - [x] Address compiler warnings
  - [x] Resolve unused imports

- [ ] **Improve Documentation**
  - [ ] Add module-level documentation
  - [ ] Document public interfaces
  - [ ] Add examples where appropriate

## Implementation Steps for Test Refactoring (Detailed)

1. **Create Test Directory Structure** ✅

   ```
   tests/
   ├── common/         # Shared test utilities
   │   ├── mod.rs
   │   └── fixtures.rs
   ├── domain/         # Domain model tests
   │   ├── mod.rs
   │   ├── symbol_tests.rs
   │   └── symbolset_tests.rs
   ├── api/            # API endpoint tests
   │   ├── mod.rs
   │   └── handlers_tests.rs
   ├── mcp/            # MCP method tests
   │   ├── mod.rs
   │   └── get_symbols_tests.rs
   └── llm/            # LLM integration tests
       ├── mod.rs
       ├── client_tests.rs
       └── prompts_tests.rs
   ```

2. **Create Test Utilities** ✅

   - Implement helper functions to create test symbols and symbol sets
   - Create mock repositories for testing handlers (TODO)
   - Set up test context with predictable data

3. **Migrating Existing Tests** ✅

   - Move test cases from in-file tests to the new structure
   - Update tests to use shared fixtures and utilities
   - Add assertions to verify all expected behaviors

4. **Add Integration Tests** ✅
   - Create tests that verify components work together correctly
   - Test API endpoints with in-memory repositories
   - Verify MCP handlers correctly process requests and return expected responses
