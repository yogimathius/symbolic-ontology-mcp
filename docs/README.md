# Symbol Ontology MCP Project Documentation

This directory contains documentation for the Symbol Ontology MCP (Model Context Protocol) project, a Rust-based symbolic reasoning engine that implements the Model Context Protocol.

## Current Documentation

### API Documentation

- [API Test Requests](api_test_requests.md) - List of curl commands for testing API endpoints
- [MCP Implementation](mcp_implementation.md) - Details about the MCP implementation

### Development Planning

- [V1.5 Checklist](v1.5-checklist.md) - Current detailed roadmap and checklist for the v1.5 MVP
- [Project README](project-readme.md) - Original project README

### MCP Integration

- [Cursor MCP Integration](cursor-mcp-integration.md) - Guide for integrating with Cursor
- [MCP Prompt Templates](mcp-prompt-templates.md) - Example prompts for using the MCP server

### Architecture

- [Requirements](architecture/requirements.md) - Project requirements specification
- [Backend Tech Stack](architecture/backend-tech-stack.md) - Overview of the backend technology stack

### Research

- [MCP Symbolic Reasoning](research/mcps-symbolic-reasoning.md) - Research on MCP and symbolic reasoning
- [Ontology Research](research/ontology-research.md) - Research on symbolic ontologies and their structure

### Scripts

- [`scripts/test_api.sh`](../scripts/test_api.sh) - Shell script to test API and MCP endpoints

## Completed/Legacy Documentation

Documentation about completed work or historical requirements:

- [Foundational Checklist](completed/foundational-checklist.md) - Core requirements that have been implemented
- [Updated Checklist](completed/updated-checklist.md) - Revised checklist with progress updates
- [Development Checklist](completed/development-checklist.md) - Original development roadmap (archived)
- [MVP Roadmap](completed/mvp-roadmap.md) - Original MVP roadmap (archived)
- [Missing Requirements Analysis](completed/missing-requirements.md) - Previous gap analysis (archived)

## Backlog Documentation

Ideas and requirements for future implementation:

- [Tower Requirements](backlog/tower-requirements.md) - Requirements for Tower middleware integration
- [Future Development Items](backlog/future-development.md) - Features planned for future versions

## Directory Structure

The documentation is organized into the following directories:

```
docs/
  ├── README.md                           # This index file
  ├── v1.5-checklist.md                   # Current development checklist
  ├── cursor-mcp-integration.md           # Cursor integration guide
  ├── mcp-prompt-templates.md             # Example MCP prompts
  ├── mcp_implementation.md               # MCP implementation details
  ├── project-readme.md                   # Original project README
  ├── architecture/                       # System architecture documentation
  │   ├── backend-tech-stack.md           # Backend technology documentation
  │   └── requirements.md                 # Project requirements
  ├── completed/                          # Documentation for completed work
  │   ├── foundational-checklist.md       # Completed foundational tasks
  │   ├── updated-checklist.md            # Completed items with progress updates
  │   ├── development-checklist.md        # Original development roadmap
  │   ├── mvp-roadmap.md                  # Original MVP roadmap
  │   └── missing-requirements.md         # Previous gap analysis
  ├── research/                           # Research documentation
  │   ├── mcps-symbolic-reasoning.md      # Research on MCP and symbolic reasoning
  │   └── ontology-research.md            # Research on symbolic ontologies
  └── backlog/                            # Future implementation ideas
      ├── tower-requirements.md           # Tower middleware requirements
      └── future-development.md           # Features for future versions
```

## How to Use This Documentation

- For current development priorities, refer to `v1.5-checklist.md`
- For API testing, use `api_test_requests.md` or the test script in `scripts/`
- For historical context on what's been implemented, check the `completed/` directory
- For future planning and ideas, see the `backlog/` directory
- For architectural design and research, see the respective subdirectories
