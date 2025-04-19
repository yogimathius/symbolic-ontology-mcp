use crate::domain::{Symbol, SymbolSet};
use async_trait::async_trait;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

/// Domain-specific error type for repository operations
#[derive(Debug)]
pub enum RepositoryError {
    /// Entity not found
    NotFound(String),
    /// Conflict with existing entity
    Conflict(String),
    /// Internal repository error
    Internal(String),
    /// Validation error
    Validation(String),
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
            Self::Validation(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

// Add From implementation for sqlx::Error
impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => RepositoryError::NotFound("Entity not found".to_string()),
            _ => RepositoryError::Internal(format!("Database error: {}", error)),
        }
    }
}

// Add From implementation for serde_json::Error
impl From<serde_json::Error> for RepositoryError {
    fn from(error: serde_json::Error) -> Self {
        RepositoryError::Internal(format!("JSON serialization error: {}", error))
    }
}

impl Error for RepositoryError {}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

/// Repository trait for Symbol entities
#[async_trait]
pub trait SymbolRepository: Send + Sync {
    /// Get a symbol by its ID
    async fn get_symbol(&self, id: &str) -> RepositoryResult<Symbol>;

    /// List all symbols, optionally filtering by category
    async fn list_symbols(&self, category: Option<&str>) -> RepositoryResult<Vec<Symbol>>;

    /// Search for symbols containing the query in name or description
    async fn search_symbols(&self, query: &str) -> RepositoryResult<Vec<Symbol>>;

    /// Create a new symbol
    async fn create_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol>;

    /// Update an existing symbol
    async fn update_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol>;

    /// Delete a symbol by its ID
    async fn delete_symbol(&self, id: &str) -> RepositoryResult<()>;
}

/// Repository trait for SymbolSet entities
#[async_trait]
pub trait SymbolSetRepository: Send + Sync {
    /// Get a symbol set by its ID
    async fn get_symbol_set(&self, id: &str) -> RepositoryResult<SymbolSet>;

    /// List all symbol sets, optionally filtering by category
    async fn list_symbol_sets(&self, category: Option<&str>) -> RepositoryResult<Vec<SymbolSet>>;

    /// Search for symbol sets containing the query in name or description
    async fn search_symbol_sets(&self, query: &str) -> RepositoryResult<Vec<SymbolSet>>;

    /// Create a new symbol set
    async fn create_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet>;

    /// Update an existing symbol set
    async fn update_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet>;

    /// Delete a symbol set by its ID
    async fn delete_symbol_set(&self, id: &str) -> RepositoryResult<()>;
}

/// Factory for creating repository instances
pub trait RepositoryFactory {
    /// Create a new symbol repository
    fn create_symbol_repository(&self) -> Arc<dyn SymbolRepository>;

    /// Create a new symbol set repository
    fn create_symbol_set_repository(&self) -> Arc<dyn SymbolSetRepository>;
}
