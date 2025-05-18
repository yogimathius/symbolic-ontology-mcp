use async_trait::async_trait;
use sqlx::PgPool;

use crate::db::pool::DbError;
use crate::db::queries::SymbolQueries;
use crate::db::repository::interfaces::{RepositoryError, RepositoryResult, SymbolRepository};
use crate::domain::Symbol;

pub struct PgSymbolRepository {
    pool: PgPool,
}

impl PgSymbolRepository {
    pub fn new(pool: PgPool) -> Self {
        PgSymbolRepository { pool }
    }
}

#[async_trait]
impl SymbolRepository for PgSymbolRepository {
    async fn get_symbol(&self, id: &str) -> RepositoryResult<Symbol> {
        let db_symbol = SymbolQueries::get_by_id(&self.pool, id)
            .await
            .map_err(|e| match e {
                DbError::NotFound => {
                    RepositoryError::NotFound(format!("Symbol with id {} not found", id))
                }
                _ => RepositoryError::Internal(format!("Database error: {}", e)),
            })?;

        Ok(Symbol {
            id: db_symbol.id,
            name: db_symbol.name,
            category: db_symbol.category,
            description: db_symbol.description,
            interpretations: db_symbol.interpretations,
            related_symbols: db_symbol.related_symbols,
            properties: db_symbol.properties,
        })
    }

    async fn list_symbols(&self, category: Option<&str>) -> RepositoryResult<Vec<Symbol>> {
        let db_symbols = SymbolQueries::list(&self.pool, category)
            .await
            .map_err(|e| RepositoryError::Internal(format!("Database error: {}", e)))?;

        let symbols = db_symbols
            .into_iter()
            .map(|db_symbol| Symbol {
                id: db_symbol.id,
                name: db_symbol.name,
                category: db_symbol.category,
                description: db_symbol.description,
                interpretations: db_symbol.interpretations,
                related_symbols: db_symbol.related_symbols,
                properties: db_symbol.properties,
            })
            .collect();

        Ok(symbols)
    }

    async fn search_symbols(&self, query: &str) -> RepositoryResult<Vec<Symbol>> {
        let db_symbols = SymbolQueries::search(&self.pool, query)
            .await
            .map_err(|e| RepositoryError::Internal(format!("Database error: {}", e)))?;

        let symbols = db_symbols
            .into_iter()
            .map(|db_symbol| Symbol {
                id: db_symbol.id,
                name: db_symbol.name,
                category: db_symbol.category,
                description: db_symbol.description,
                interpretations: db_symbol.interpretations,
                related_symbols: db_symbol.related_symbols,
                properties: db_symbol.properties,
            })
            .collect();

        Ok(symbols)
    }

    async fn create_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        let db_symbol = crate::db::models::Symbol {
            id: symbol.id.clone(),
            name: symbol.name.clone(),
            category: symbol.category.clone(),
            description: symbol.description.clone(),
            interpretations: symbol.interpretations.clone(),
            related_symbols: symbol.related_symbols.clone(),
            properties: symbol.properties.clone(),
        };

        SymbolQueries::create(&self.pool, &db_symbol)
            .await
            .map_err(|e| match e {
                DbError::Conflict(msg) => RepositoryError::Conflict(msg),
                _ => RepositoryError::Internal(format!("Database error: {}", e)),
            })?;

        Ok(symbol)
    }

    async fn update_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        let db_symbol = crate::db::models::Symbol {
            id: symbol.id.clone(),
            name: symbol.name.clone(),
            category: symbol.category.clone(),
            description: symbol.description.clone(),
            interpretations: symbol.interpretations.clone(),
            related_symbols: symbol.related_symbols.clone(),
            properties: symbol.properties.clone(),
        };

        SymbolQueries::update(&self.pool, &db_symbol)
            .await
            .map_err(|e| match e {
                DbError::NotFound => {
                    RepositoryError::NotFound(format!("Symbol with id {} not found", symbol.id))
                }
                _ => RepositoryError::Internal(format!("Database error: {}", e)),
            })?;

        Ok(symbol)
    }

    async fn delete_symbol(&self, id: &str) -> RepositoryResult<()> {
        SymbolQueries::delete(&self.pool, id)
            .await
            .map_err(|e| match e {
                DbError::NotFound => {
                    RepositoryError::NotFound(format!("Symbol with id {} not found", id))
                }
                _ => RepositoryError::Internal(format!("Database error: {}", e)),
            })
    }
}
