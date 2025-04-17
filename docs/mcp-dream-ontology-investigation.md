# MCP Implementation Plan for Dream Ontology

## Understanding the Reference Implementation

The `ghbountybot/mcp` repository provides a Rust implementation of the Model Context Protocol, with:

1. **Core Components**:

   - `Service` trait defining the MCP service interface
   - `BasicService` implementing the core MCP protocol functionality
   - Registry system for tools, resources, and prompts
   - JSON-RPC interface for MCP method handling

2. **Serving Mechanisms**:

   - `serve_over_stdio` - Enables CLI-based interaction
   - `serve_over_sse` - Provides HTTP/SSE-based service with Axum

3. **Resource Management**:
   - Support for fixed resources (predefined data)
   - Support for subscribable resources (change notifications)

## Integration Plan for Dream Ontology

Based on your current development checklist, here's a step-by-step plan for implementing MCP functionality:

### Phase 1: MCP Server Integration (Your Priority 1)

1. **Add MCP Endpoint to Axum Router**

   - [ ] Define an MCP JSON-RPC endpoint at `/mcp` in your main Axum router
   - [ ] Configure CORS handling for cross-origin access
   - [ ] Set up appropriate middleware for request/response logging

2. **Implement Basic MCP Service Structure**

   - [ ] Create a `DreamOntologyMcpService` that implements the `Service` trait
   - [ ] Define and implement required methods (init, ping, list_tools, etc.)
   - [ ] Provide proper error mapping between domain errors and MCP protocol errors

3. **Connect to Repository Layer**

   - [ ] Inject your existing `SymbolRepository` into the MCP service
   - [ ] Map repository operations to appropriate MCP methods
   - [ ] Ensure proper error handling for repository operations

4. **Enable `get_symbols` MCP Method**
   - [ ] Complete the implementation of the method in your MCP handler
   - [ ] Connect to existing repository functionality
   - [ ] Add parameter validation with detailed error messages
   - [ ] Implement pagination support for large result sets

### Phase 2: Testing and Documentation

1. **Create MCP Integration Tests**

   - [ ] Develop test cases for the MCP endpoint
   - [ ] Test all parameter variations and error conditions
   - [ ] Ensure compatibility with MCP protocol specification

2. **Document MCP Endpoints**
   - [ ] Enhance your API documentation to include MCP endpoints
   - [ ] Provide example requests and responses
   - [ ] Document error codes and their meanings

### Phase 3: MCP Method Expansion (Your Near-Term Plans)

1. **Implement Core Symbol Methods**

   - [ ] Add `get_symbol` method for retrieving a single symbol by ID
   - [ ] Add `get_symbol_sets` for retrieving related symbol sets
   - [ ] Add `get_related_symbols` for exploring connections between symbols

2. **Add Advanced Symbol Analysis**
   - [ ] Implement `analyze_symbolic_pattern` for pattern identification
   - [ ] Add `get_archetypal_symbols` for archetypal classifications
   - [ ] Create `interpret_symbol_context` for contextual interpretations

## Implementation Checklist

### 1. MCP Service Structure

```rust
// In a new file: src/mcp/service.rs
use crate::domain::{RepositoryFactory, SymbolRepository};
use crate::mcp::error::McpError;
use std::sync::Arc;

pub struct DreamOntologyMcpService {
    symbol_repository: Arc<dyn SymbolRepository>,
    // Other service components...
}

impl DreamOntologyMcpService {
    pub fn new(symbol_repository: Arc<dyn SymbolRepository>) -> Self {
        Self {
            symbol_repository,
            // Initialize other components...
        }
    }

    // Service-specific methods...
}

// Implement the MCP Service trait for your service
impl mcp::Service for DreamOntologyMcpService {
    // Implement required methods...
}
```

### 2. Axum Integration

```rust
// In src/main.rs or where you set up your Axum router
use crate::mcp::service::DreamOntologyMcpService;

// In your server setup function
let mcp_service = DreamOntologyMcpService::new(symbol_repository.clone());

// Add to your router
let app = Router::new()
    // Existing API routes...
    .route("/mcp", post(handle_mcp_request))
    .with_state(Arc::new(mcp_service));
```

### 3. MCP Request Handler

```rust
// In src/mcp/handlers.rs
use axum::{Json, extract::State};
use mcp_schema::{Request, Response};
use std::sync::Arc;

use crate::mcp::service::DreamOntologyMcpService;

pub async fn handle_mcp_request(
    State(service): State<Arc<DreamOntologyMcpService>>,
    Json(request): Json<Request>,
) -> Json<Response> {
    // Process the MCP request through your service
    let response = service.handle_request(request).await;
    Json(response)
}
```

### 4. MCP Error Handling

```rust
// In src/mcp/error.rs
#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("Repository error: {0}")]
    Repository(#[from] crate::domain::RepositoryError),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Method not found: {0}")]
    MethodNotFound(String),

    // Other error variants...
}

impl From<McpError> for mcp_schema::Error {
    fn from(error: McpError) -> Self {
        match error {
            McpError::Repository(repo_error) => {
                // Map repository errors to appropriate MCP error codes
                // ...
            }
            // Map other errors...
        }
    }
}
```

## Integration with Your Development Checklist

This plan aligns with your current development checklist priorities:

1. **MCP Server Integration (Priority 1)**

   - Directly addresses your top priority tasks
   - Provides a clear path for integrating MCP with your existing codebase

2. **API Enhancements (Priority 2)**

   - The MCP implementation will complement your API enhancements
   - Both can share the same repository access patterns

3. **Near-Term Development**
   - Sets the foundation for your planned domain model enhancements
   - Prepares for additional MCP methods matching your roadmap

## Differences from Reference Implementation

Your Dream Ontology MCP implementation will differ from the reference weather example in a few key ways:

1. **Focus on Symbolic Data**: Instead of dynamic weather data, you'll be providing access to your symbolic ontology

2. **Rich Relationships**: Your implementation will need to support complex relationships between symbols

3. **Integration with Existing Code**: You're integrating with an established codebase, rather than building from scratch

4. **Domain-Specific Functionality**: You'll need methods specific to symbolic reasoning and interpretation
