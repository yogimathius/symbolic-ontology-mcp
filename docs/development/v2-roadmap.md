# Symbol Ontology v2 Roadmap

This document outlines the planned improvements and features for version 2 of the Symbol Ontology project. It consolidates analysis of the current system and specific implementation plans.

## Current Limitations and Analysis

Our analysis of the current system identified several key areas for improvement:

### 1. Symbolic Richness vs. Flat Representation

- **Current State:** Symbol interpretations are stored in flat, key-value form with minimal depth
- **Need:** Support for layered symbolic interpretations and symbolic amplification

### 2. Cognitive Graphs & Symbolic Networks

- **Current State:** Relationships are shallow (e.g., `related_symbols` is a Vec of IDs)
- **Need:** A typed symbolic graph with semantically meaningful relationships

### 3. Ontology Structure & Archetypal Modeling

- **Current State:** `SymbolSet` groups symbols, but lacks hierarchy or multi-resolution organization
- **Need:** Hierarchical archetypal structures and ontology levels

### 4. LLM-Specific Structuring & Integration

- **Current State:** The API is clean but not optimized for LLM symbolic prompting
- **Need:** LLM-grounding endpoints and prompt templates for context

## Phase 1: Layered Interpretation Model

The first major improvement will be implementing layered interpretation for symbols, providing greater depth and context.

### Implementation Plan

1. **Create new `InterpretationLayer` struct**:

   - Add `universal: Option<String>` field for universal/archetypal meaning
   - Add `cultural: Option<String>` field for cultural context
   - Add `personal: Option<String>` field for personal associations
   - Implement serialization traits

2. **Refactor `Symbol` model**:

   - Update `interpretations` field to use `HashMap<String, InterpretationLayer>`
   - Ensure backward compatibility with existing data
   - Update serialization/deserialization

3. **Update Repositories**:

   - Modify in-memory repository implementation
   - Update PostgreSQL repository implementation
   - Implement fallback for legacy data

4. **Add LLM-Oriented MCP Endpoint**:
   - Create `get_symbol_prompt_context` MCP method
   - Format layered interpretations for LLM consumption
   - Support compact output option

## Phase 2: Symbolic Relationship Graph

The second major improvement will enhance the relationships between symbols to create a rich semantic network.

### Implementation Plan

1. **Create `SymbolRelation` Model**:

   - Define relationship types (similar_to, opposite_of, transforms_into, etc.)
   - Create join table for symbol relationships
   - Add directional relationship support

2. **Update Repository Layer**:

   - Add methods for relationship queries
   - Optimize graph traversal operations
   - Implement relationship CRUD operations

3. **Add Graph Query MCP Methods**:

   - Create `get_symbol_relations` method
   - Implement `get_related_network` for multi-hop relationships
   - Support filtering by relationship type

4. **Add Graph Visualization Support**:
   - Generate graph data structures for visualization
   - Support exporting in standard graph formats
   - Create optional visualization endpoint

## Phase 3: Archetypal Hierarchy

The third phase will add hierarchical organization to the ontology, enabling archetypal patterns to be represented.

### Implementation Plan

1. **Create Archetypal Models**:

   - Implement `Archetype` struct with description and properties
   - Create hierarchy relationships between archetypes
   - Link symbols to archetypes

2. **Update Database Schema**:

   - Add archetype tables
   - Create relationship tables
   - Implement migration for existing data

3. **Add Archetypal MCP Methods**:

   - Create `get_archetypes` method
   - Implement `get_symbols_by_archetype`
   - Add `get_archetype_hierarchy` for tree visualization

4. **Improve Symbol Search**:
   - Add search by archetype
   - Implement relevance scoring
   - Support filtering by archetypal qualities

## Phase 4: LLM Prompt Optimization

The final phase will focus on optimizing the integration with LLMs through specialized endpoints and formats.

### Implementation Plan

1. **Create Prompt Template System**:

   - Design flexible template format
   - Implement template rendering engine
   - Create library of common templates

2. **Add Context Generation Endpoints**:

   - Implement `get_interpretation_context` method
   - Create `get_symbol_network_context` for related symbols
   - Add `get_archetypal_context` for higher-level patterns

3. **Optimize for Token Efficiency**:

   - Implement compact output formats
   - Create summarization options for large contexts
   - Add priority ranking for context elements

4. **Add Client Helpers**:
   - Create SDK methods for common prompt patterns
   - Implement example clients
   - Add documentation with best practices

## Implementation Timeline

| Phase | Feature                      | Timeline | Priority |
| ----- | ---------------------------- | -------- | -------- |
| 1     | Layered Interpretation Model | TDB      | High     |
| 1     | Repository Updates           | TDB      | High     |
| 1     | LLM Context Endpoint         | TDB      | High     |
| 2     | Symbolic Relationship Graph  | TDB      | Medium   |
| 2     | Graph Query Methods          | TDB      | Medium   |
| 3     | Archetypal Models            | TDB      | Medium   |
| 3     | Archetypal MCP Methods       | TDB      | Medium   |
| 4     | Prompt Template System       | TDB      | Low      |
| 4     | Context Generation           | TDB      | Low      |
