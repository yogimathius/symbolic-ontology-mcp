/*!
 * # Domain Module
 *
 * The domain module contains the core business logic and models for the symbolic reasoning engine.
 * This includes:
 *
 * - **Symbol**: Represents a symbolic entity with interpretations and relationships
 * - **SymbolSet**: A collection of related symbols organized into an ontology
 * - **Repository**: Interfaces for persistence and data access
 *
 * The domain layer is designed to be persistence-agnostic, with implementations
 * provided in the infrastructure layer.
 */

// Domain module exports
pub mod ontology;
pub mod repository;
pub mod symbols;

// Re-exports for convenient access
pub use ontology::SymbolSet;
pub use repository::{
    RepositoryError, RepositoryFactory, RepositoryResult, SymbolRepository, SymbolSetRepository,
};
pub use symbols::Symbol;
