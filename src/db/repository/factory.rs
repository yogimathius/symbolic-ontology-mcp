use sqlx::PgPool;
use std::sync::Arc;

use crate::db::repository::{
    PgSymbolRepository, PgSymbolSetRepository,
    interfaces::{SymbolRepository, SymbolSetRepository},
};

pub struct PgRepositoryFactory {
    pool: PgPool,
}

impl PgRepositoryFactory {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn create_symbol_repository(&self) -> Arc<dyn SymbolRepository> {
        Arc::new(PgSymbolRepository::new(self.pool.clone()))
    }

    pub fn create_symbol_set_repository(&self) -> Arc<dyn SymbolSetRepository> {
        Arc::new(PgSymbolSetRepository::new(self.pool.clone()))
    }
}
