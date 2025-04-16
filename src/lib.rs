/*!
 * # Dream Ontology MCP Library
 *
 * This library implements a symbolic reasoning engine for dream interpretation
 * and symbolic analysis through the Model Context Protocol (MCP).
 *
 * ## Core Components
 *
 * - **Domain Models**: Define the core business objects like `Symbol` and `SymbolSet`
 * - **MCP Implementation**: Protocol-compliant methods for symbolic reasoning
 * - **Infrastructure**: Repository implementations and external integrations
 *
 * ## Usage Example
 *
 * ```rust
 * use dream_ontology_mcp::domain::{Symbol, SymbolRepository};
 * use dream_ontology_mcp::infrastructure::memory_repository::MemoryRepositoryFactory;
 *
 * // Create a repository with test data
 * let repo_factory = MemoryRepositoryFactory::new().with_test_data();
 * let symbol_repo = repo_factory.create_symbol_repository();
 *
 * // Use the repository asynchronously
 * async fn get_water_symbol(repo: Arc<dyn SymbolRepository>) -> Option<Symbol> {
 *     repo.get("water").await.ok()
 * }
 * ```
 */

// Re-export all modules for better ergonomics

/// Core domain models and business logic
pub mod domain;

/// MCP protocol implementation for symbolic reasoning
pub mod mcp;

/// Infrastructure implementations for repositories and external services
pub mod infrastructure;

/// Utility functions and shared helpers
mod utils;
