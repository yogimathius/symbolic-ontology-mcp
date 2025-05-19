// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use sqlx::{PgPool, Row};
use tracing::info;

use crate::db::models::{Symbol, SymbolSet};
use crate::db::pool::{DbError, DbResult};

pub struct SymbolQueries;

impl SymbolQueries {
    pub async fn get_by_id(pool: &PgPool, id: &str) -> DbResult<Symbol> {
        // TODO: Implement this
        Err(DbError::NotFound)
    }

    pub async fn list(pool: &PgPool, category: Option<&str>) -> DbResult<Vec<Symbol>> {
        // TODO: Implement this
        Ok(Vec::new())
    }

    pub async fn search(pool: &PgPool, query: &str) -> DbResult<Vec<Symbol>> {
        // TODO: Implement this
        Ok(Vec::new())
    }

    pub async fn create(pool: &PgPool, symbol: &Symbol) -> DbResult<Symbol> {
        // TODO: Implement this
        Err(DbError::NotFound)
    }

    pub async fn update(pool: &PgPool, symbol: &Symbol) -> DbResult<Symbol> {
        // TODO: Implement this
        Err(DbError::NotFound)
    }

    pub async fn delete(pool: &PgPool, id: &str) -> DbResult<()> {
        // TODO: Implement this
        Err(DbError::NotFound)
    }

    pub async fn seed_test_data(pool: &PgPool) -> DbResult<()> {
        // TODO: Implement this
        Ok(())
    }
}

pub struct SymbolSetQueries;

impl SymbolSetQueries {
    pub async fn get_by_id(pool: &PgPool, id: &str) -> DbResult<SymbolSet> {
        // TODO: Implement this
        Err(DbError::NotFound)
    }

    pub async fn list(pool: &PgPool, category: Option<&str>) -> DbResult<Vec<SymbolSet>> {
        // TODO: Implement this
        Ok(Vec::new())
    }

    pub async fn search(pool: &PgPool, query: &str) -> DbResult<Vec<SymbolSet>> {
        // TODO: Implement this
        Ok(Vec::new())
    }

    pub async fn create(pool: &PgPool, set: &SymbolSet) -> DbResult<SymbolSet> {
        // TODO: Implement this
        Err(DbError::NotFound)
    }

    pub async fn update(pool: &PgPool, set: &SymbolSet) -> DbResult<SymbolSet> {
        // TODO: Implement this
        Err(DbError::NotFound)
    }

    pub async fn delete(pool: &PgPool, id: &str) -> DbResult<()> {
        // TODO: Implement this
        Err(DbError::NotFound)
    }
}
