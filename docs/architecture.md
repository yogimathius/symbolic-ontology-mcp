# Dream Ontology System Architecture

## System Overview

The Dream Ontology project consists of two separate but complementary services:

1. **Symbolic Ontology MCP Server** (this repository)
2. **Dream Interpretation MCP Client** (separate repository)

This architectural separation follows the best practices for Model Context Protocol (MCP) applications, where the data sources and the LLM integration are separated to provide clear boundaries of responsibility.

## Architectural Diagram

```
┌─────────────────────┐               ┌─────────────────────┐               ┌─────────────────────┐
│                     │               │                     │               │                     │
│   Dream Ontology    │◄────MCP────►│   Dream Interp.     │◄────API────►│    LLM Provider      │
│   MCP Server        │               │   MCP Client        │               │    (OpenRouter)     │
│   (This Repository) │               │   (Separate Repo)   │               │                     │
└─────────────────────┘               └─────────────────────┘               └─────────────────────┘
         │                                      │
         │                                      │
         ▼                                      ▼
┌─────────────────────┐               ┌─────────────────────┐
│                     │               │                     │
│   Symbol Database   │               │    Dream Database   │
│                     │               │                     │
└─────────────────────┘               └─────────────────────┘
```

## Component Responsibilities

### 1. Symbolic Ontology MCP Server (This Repository)

**Primary Responsibility**: Act as the "source of truth" for symbolic data and relationships.

**Key Functions**:

- Store and manage the ontology of symbols (dreams, mythology, archetypes)
- Provide MCP-compliant endpoints for querying symbols (`get_symbols`, etc.)
- Ensure data integrity and accuracy
- Implement efficient data retrieval patterns

**Does NOT Include**:

- LLM integration or API calls
- Prompt engineering
- Dream interpretation logic
- User authentication (relies on client)

### 2. Dream Interpretation MCP Client (Separate Repository)

**Primary Responsibility**: Provide LLM-powered interpretation of dreams using symbolic data.

**Key Functions**:

- Store and manage user dreams and interpretations
- Query the Symbolic Ontology MCP Server for symbolic data
- Construct effective prompts for LLM integration
- Call LLM APIs (via OpenRouter) with appropriate context
- Handle user authentication and personalization

## Why This Separation?

This architectural decision provides several benefits:

1. **Clear Separation of Concerns**: Each service has a focused responsibility, making the code more maintainable.

2. **Independent Scaling**: The symbol server can be optimized for data retrieval, while the interpretation service can be scaled based on LLM processing needs.

3. **MCP Best Practices**: This follows the Model Context Protocol pattern where:

   - MCP Servers provide factual data and grounding
   - MCP Clients consume that data and integrate with LLMs

4. **Future Flexibility**: Multiple different client applications can use the same symbolic data service, each with potentially different LLM integrations or prompt strategies.

5. **Technology Evolution**: As LLM technology rapidly evolves, the interpretation service can be updated without affecting the symbol service.

## Communication Flow

A typical request flow would be:

1. User requests dream interpretation from the Dream Interpretation Client
2. The client service queries the Symbolic Ontology Server (this repo) via MCP
3. The Symbolic Ontology Server returns relevant symbolic data
4. The client service constructs a prompt incorporating this symbolic data
5. The client service calls the LLM provider (OpenRouter) with this prompt
6. The interpretation is returned to the user

## Development Roadmap

Each repository has its own development roadmap:

- For this repository (Symbolic Ontology Server), see [updated-checklist.md](../updated-checklist.md)
- The Dream Interpretation Client will have its own separate implementation plan
