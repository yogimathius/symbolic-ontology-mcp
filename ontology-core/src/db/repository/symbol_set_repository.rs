// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use async_trait::async_trait;
use sqlx::PgPool;

use crate::db::pool::DbError;
use crate::db::queries::{SymbolQueries, SymbolSetQueries};
use crate::db::repository::interfaces::{
    Repository, RepositoryError, RepositoryResult, SymbolSetRepository,
};
use crate::domain::SymbolSet;

pub struct PgSymbolSetRepository {
    pool: PgPool,
}

impl PgSymbolSetRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSymbolSetRepository { pool }
    }
}

impl Repository for PgSymbolSetRepository {}

#[async_trait]
impl SymbolSetRepository for PgSymbolSetRepository {
    async fn get_symbol_set(&self, id: &str) -> RepositoryResult<SymbolSet> {
        let db_set = SymbolSetQueries::get_by_id(&self.pool, id)
            .await
            .map_err(|e| match e {
                DbError::NotFound => {
                    RepositoryError::NotFound(format!("SymbolSet with id {} not found", id))
                }
                _ => RepositoryError::Internal(format!("Database error: {}", e)),
            })?;

        let db_symbols = SymbolQueries::list(&self.pool, None)
            .await
            .map_err(|e| RepositoryError::Internal(format!("Database error: {}", e)))?;

        Ok(db_set.to_domain(&db_symbols))
    }

    async fn list_symbol_sets(&self, category: Option<&str>) -> RepositoryResult<Vec<SymbolSet>> {
        let db_sets = SymbolSetQueries::list(&self.pool, category)
            .await
            .map_err(|e| RepositoryError::Internal(format!("Database error: {}", e)))?;

        let db_symbols = SymbolQueries::list(&self.pool, None)
            .await
            .map_err(|e| RepositoryError::Internal(format!("Database error: {}", e)))?;

        let symbol_sets = db_sets
            .into_iter()
            .map(|db_set| db_set.to_domain(&db_symbols))
            .collect();

        Ok(symbol_sets)
    }

    async fn search_symbol_sets(&self, query: &str) -> RepositoryResult<Vec<SymbolSet>> {
        let db_sets = SymbolSetQueries::search(&self.pool, query)
            .await
            .map_err(|e| RepositoryError::Internal(format!("Database error: {}", e)))?;

        let db_symbols = SymbolQueries::list(&self.pool, None)
            .await
            .map_err(|e| RepositoryError::Internal(format!("Database error: {}", e)))?;

        let symbol_sets = db_sets
            .into_iter()
            .map(|db_set| db_set.to_domain(&db_symbols))
            .collect();

        Ok(symbol_sets)
    }

    async fn create_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet> {
        let db_set = crate::db::models::SymbolSet::from_domain(symbol_set.clone());

        SymbolSetQueries::create(&self.pool, &db_set)
            .await
            .map_err(|e| match e {
                DbError::Conflict(msg) => RepositoryError::Conflict(msg),
                _ => RepositoryError::Internal(format!("Database error: {}", e)),
            })?;

        Ok(symbol_set)
    }

    async fn update_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet> {
        let db_set = crate::db::models::SymbolSet::from_domain(symbol_set.clone());

        SymbolSetQueries::update(&self.pool, &db_set)
            .await
            .map_err(|e| match e {
                DbError::NotFound => RepositoryError::NotFound(format!(
                    "SymbolSet with id {} not found",
                    symbol_set.id
                )),
                _ => RepositoryError::Internal(format!("Database error: {}", e)),
            })?;

        Ok(symbol_set)
    }

    async fn delete_symbol_set(&self, id: &str) -> RepositoryResult<()> {
        SymbolSetQueries::delete(&self.pool, id)
            .await
            .map_err(|e| match e {
                DbError::NotFound => {
                    RepositoryError::NotFound(format!("SymbolSet with id {} not found", id))
                }
                _ => RepositoryError::Internal(format!("Database error: {}", e)),
            })
    }
}
