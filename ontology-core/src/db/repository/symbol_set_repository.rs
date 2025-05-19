// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use async_trait::async_trait;
use sqlx::PgPool;

use crate::db::pool::DbError;
use crate::db::repository::interfaces::{
    Repository, RepositoryError, RepositoryResult, SymbolSetRepository,
};
use crate::domain::SymbolSet;

pub struct PgSymbolSetRepository {
    pool: PgPool,
}

impl Repository for PgSymbolSetRepository {}

impl PgSymbolSetRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSymbolSetRepository { pool }
    }
}

#[async_trait]
impl SymbolSetRepository for PgSymbolSetRepository {
    async fn get_symbol_set(&self, id: &str) -> RepositoryResult<SymbolSet> {
        // TODO: Implement SymbolSetQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolSetQueries not yet implemented in ontology-core".to_string(),
        ))
    }

    async fn list_symbol_sets(&self, category: Option<&str>) -> RepositoryResult<Vec<SymbolSet>> {
        // TODO: Implement SymbolSetQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolSetQueries not yet implemented in ontology-core".to_string(),
        ))
    }

    async fn search_symbol_sets(&self, query: &str) -> RepositoryResult<Vec<SymbolSet>> {
        // TODO: Implement SymbolSetQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolSetQueries not yet implemented in ontology-core".to_string(),
        ))
    }

    async fn create_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet> {
        // TODO: Implement SymbolSetQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolSetQueries not yet implemented in ontology-core".to_string(),
        ))
    }

    async fn update_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet> {
        // TODO: Implement SymbolSetQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolSetQueries not yet implemented in ontology-core".to_string(),
        ))
    }

    async fn delete_symbol_set(&self, id: &str) -> RepositoryResult<()> {
        // TODO: Implement SymbolSetQueries in ontology-core
        // For now just return a placeholder error
        Err(RepositoryError::NotImplemented(
            "SymbolSetQueries not yet implemented in ontology-core".to_string(),
        ))
    }
}
