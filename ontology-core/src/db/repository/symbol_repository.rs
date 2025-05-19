// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use async_trait::async_trait;
use sqlx::PgPool;

use crate::db::pool::DbError;
use crate::db::repository::interfaces::{
    Repository, RepositoryError, RepositoryResult, SymbolRepository,
};
use crate::domain::Symbol;

pub struct PgSymbolRepository {
    pool: PgPool,
}

impl Repository for PgSymbolRepository {}

impl PgSymbolRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSymbolRepository { pool }
    }
}

#[async_trait]
impl SymbolRepository for PgSymbolRepository {
    async fn get_symbol(&self, id: &str) -> RepositoryResult<Symbol> {
        // TODO: Implement SymbolQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolQueries not yet implemented in ontology-core".to_string(),
        ))
    }

    async fn list_symbols(&self, category: Option<&str>) -> RepositoryResult<Vec<Symbol>> {
        // TODO: Implement SymbolQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolQueries not yet implemented in ontology-core".to_string(),
        ))
    }

    async fn search_symbols(&self, query: &str) -> RepositoryResult<Vec<Symbol>> {
        // TODO: Implement SymbolQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolQueries not yet implemented in ontology-core".to_string(),
        ))
    }

    async fn create_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        // TODO: Implement SymbolQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolQueries not yet implemented in ontology-core".to_string(),
        ))
    }

    async fn update_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        // TODO: Implement SymbolQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolQueries not yet implemented in ontology-core".to_string(),
        ))
    }

    async fn delete_symbol(&self, id: &str) -> RepositoryResult<()> {
        // TODO: Implement SymbolQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolQueries not yet implemented in ontology-core".to_string(),
        ))
    }
}
