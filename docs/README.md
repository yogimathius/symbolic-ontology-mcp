# Dream Ontology MCP Project Documentation

This directory contains documentation for the Dream Ontology MCP (Model Context Protocol) project, a Rust-based symbolic reasoning engine that implements the Model Context Protocol.

## Current Documentation

### API Documentation

- [API Test Requests](api_test_requests.md) - List of curl commands for testing API endpoints
- [MCP Implementation](mcp_implementation.md) - Details about the MCP implementation

### Development Planning

- [Development Checklist](development-checklist.md) - Prioritized roadmap of planned development tasks
- [Project README](project-readme.md) - Original project README

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

## Backlog Documentation

Ideas and requirements for future implementation:

- [Tower Requirements](backlog/tower-requirements.md) - Requirements for Tower middleware integration

## Directory Structure

The documentation is organized into the following directories:

```
docs/
  ├── README.md                           # This index file
  ├── api_test_requests.md                # API testing guide
  ├── development-checklist.md            # Current development roadmap
  ├── mcp_implementation.md               # MCP implementation details
  ├── project-readme.md                   # Original project README
  ├── architecture/                       # System architecture documentation
  │   ├── backend-tech-stack.md           # Backend technology documentation
  │   └── requirements.md                 # Project requirements
  ├── completed/                          # Documentation for completed work
  │   ├── foundational-checklist.md       # Completed foundational tasks
  │   └── updated-checklist.md            # Completed items with progress updates
  ├── research/                           # Research documentation
  │   ├── mcps-symbolic-reasoning.md      # Research on MCP and symbolic reasoning
  │   └── ontology-research.md            # Research on symbolic ontologies
  └── backlog/                            # Future implementation ideas
      └── tower-requirements.md           # Tower middleware requirements
```

## How to Use This Documentation

- For current development priorities, refer to `development-checklist.md`
- For API testing, use `api_test_requests.md` or the test script in `scripts/`
- For historical context on what's been implemented, check the `completed/` directory
- For future planning and ideas, see the `backlog/` directory
- For architectural design and research, see the respective subdirectories
