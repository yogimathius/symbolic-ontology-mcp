// This is the main library entry point
// Re-export all modules for better ergonomics

// Core domain models
pub mod domain;

// API endpoints and handlers
pub mod api;

// MCP protocol implementation
pub mod mcp;

// LLM integration
pub mod llm;

// Utilities
pub mod utils;
