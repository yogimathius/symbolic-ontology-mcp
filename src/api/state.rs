use sqlx::PgPool;
use std::sync::Arc;

use crate::db::repository::{PgRepositoryFactory, SymbolRepository, SymbolSetRepository};

#[derive(Clone)]
pub struct AppState {
    pub symbol_repository: Arc<dyn SymbolRepository>,
    pub symbol_set_repository: Arc<dyn SymbolSetRepository>,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        let factory = PgRepositoryFactory::new(db_pool);
        let symbol_repository = factory.create_symbol_repository();
        let symbol_set_repository = factory.create_symbol_set_repository();

        Self {
            symbol_repository,
            symbol_set_repository,
        }
    }
}
