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
