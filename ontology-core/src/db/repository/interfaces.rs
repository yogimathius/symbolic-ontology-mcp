// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use crate::domain::{Symbol, SymbolSet};
use async_trait::async_trait;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    Conflict(String),
    Internal(String),
    Validation(String),
    NotImplemented(String),
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::Conflict(msg) => write!(f, "Conflict: {}", msg),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
            Self::Validation(msg) => write!(f, "Validation error: {}", msg),
            Self::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
        }
    }
}

impl From<sqlx::Error> for RepositoryError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => RepositoryError::NotFound("Entity not found".to_string()),
            _ => RepositoryError::Internal(format!("Database error: {}", error)),
        }
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(error: serde_json::Error) -> Self {
        RepositoryError::Internal(format!("JSON serialization error: {}", error))
    }
}

impl Error for RepositoryError {}

pub type RepositoryResult<T> = Result<T, RepositoryError>;

// General Repository trait for use in repository factory
pub trait Repository {}

#[async_trait]
pub trait SymbolRepository: Repository + Send + Sync {
    async fn get_symbol(&self, id: &str) -> RepositoryResult<Symbol>;

    async fn list_symbols(&self, category: Option<&str>) -> RepositoryResult<Vec<Symbol>>;

    async fn search_symbols(&self, query: &str) -> RepositoryResult<Vec<Symbol>>;

    async fn create_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol>;

    async fn update_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol>;

    async fn delete_symbol(&self, id: &str) -> RepositoryResult<()>;
}

#[async_trait]
pub trait SymbolSetRepository: Repository + Send + Sync {
    async fn get_symbol_set(&self, id: &str) -> RepositoryResult<SymbolSet>;

    async fn list_symbol_sets(&self, category: Option<&str>) -> RepositoryResult<Vec<SymbolSet>>;

    async fn search_symbol_sets(&self, query: &str) -> RepositoryResult<Vec<SymbolSet>>;

    async fn create_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet>;

    async fn update_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet>;

    async fn delete_symbol_set(&self, id: &str) -> RepositoryResult<()>;
}
