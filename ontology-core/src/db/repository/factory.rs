// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use sqlx::PgPool;
use std::sync::Arc;

use crate::db::repository::{
    PgSymbolRepository, PgSymbolSetRepository,
    interfaces::{Repository, SymbolRepository, SymbolSetRepository},
};

pub trait RepositoryFactory {
    fn create_symbol_repository(&self) -> Arc<dyn SymbolRepository>;
    fn create_symbol_set_repository(&self) -> Arc<dyn SymbolSetRepository>;
}

pub struct PgRepositoryFactory {
    pool: PgPool,
}

impl PgRepositoryFactory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl RepositoryFactory for PgRepositoryFactory {
    fn create_symbol_repository(&self) -> Arc<dyn SymbolRepository> {
        Arc::new(PgSymbolRepository::new(self.pool.clone()))
    }

    fn create_symbol_set_repository(&self) -> Arc<dyn SymbolSetRepository> {
        Arc::new(PgSymbolSetRepository::new(self.pool.clone()))
    }
}
