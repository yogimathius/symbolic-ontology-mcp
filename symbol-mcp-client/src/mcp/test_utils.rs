// SPDX-License-Identifier: MPL-2.0 OR Commercial
// Copyright (c) 2024 Symbol Ontology Contributors

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;

use ontology_core::db::repository::interfaces::{RepositoryError, RepositoryResult};
use ontology_core::db::repository::{Repository, SymbolRepository, SymbolSetRepository};
use ontology_core::domain::{Symbol, SymbolSet};

/// A simple in-memory implementation of SymbolRepository for testing
#[derive(Default)]
pub struct InMemorySymbolRepository {
    symbols: Mutex<HashMap<String, Symbol>>,
}

impl Repository for InMemorySymbolRepository {}

impl InMemorySymbolRepository {
    pub fn new() -> Self {
        Self {
            symbols: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl SymbolRepository for InMemorySymbolRepository {
    async fn get_symbol(&self, id: &str) -> RepositoryResult<Symbol> {
        let symbols = self.symbols.lock().unwrap();
        symbols
            .get(id)
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound(format!("Symbol with id {} not found", id)))
    }

    async fn list_symbols(&self, category: Option<&str>) -> RepositoryResult<Vec<Symbol>> {
        let symbols = self.symbols.lock().unwrap();
        let result = match category {
            Some(cat) => symbols
                .values()
                .filter(|s| s.category == cat)
                .cloned()
                .collect(),
            None => symbols.values().cloned().collect(),
        };
        Ok(result)
    }

    async fn search_symbols(&self, query: &str) -> RepositoryResult<Vec<Symbol>> {
        let symbols = self.symbols.lock().unwrap();
        let query = query.to_lowercase();

        let result = symbols
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query)
                    || s.description.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();

        Ok(result)
    }

    async fn create_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        let mut symbols = self.symbols.lock().unwrap();

        if symbols.contains_key(&symbol.id) {
            return Err(RepositoryError::Conflict(format!(
                "Symbol with id {} already exists",
                symbol.id
            )));
        }

        let cloned = symbol.clone();
        symbols.insert(symbol.id.clone(), symbol);

        Ok(cloned)
    }

    async fn update_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        let mut symbols = self.symbols.lock().unwrap();

        if !symbols.contains_key(&symbol.id) {
            return Err(RepositoryError::NotFound(format!(
                "Symbol with id {} not found",
                symbol.id
            )));
        }

        let cloned = symbol.clone();
        symbols.insert(symbol.id.clone(), symbol);

        Ok(cloned)
    }

    async fn delete_symbol(&self, id: &str) -> RepositoryResult<()> {
        let mut symbols = self.symbols.lock().unwrap();

        if !symbols.contains_key(id) {
            return Err(RepositoryError::NotFound(format!(
                "Symbol with id {} not found",
                id
            )));
        }

        symbols.remove(id);

        Ok(())
    }
}

/// A simple in-memory implementation of SymbolSetRepository for testing
#[derive(Default)]
pub struct InMemorySymbolSetRepository {
    symbol_sets: Mutex<HashMap<String, SymbolSet>>,
}

impl Repository for InMemorySymbolSetRepository {}

impl InMemorySymbolSetRepository {
    pub fn new() -> Self {
        Self {
            symbol_sets: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl SymbolSetRepository for InMemorySymbolSetRepository {
    async fn get_symbol_set(&self, id: &str) -> RepositoryResult<SymbolSet> {
        let symbol_sets = self.symbol_sets.lock().unwrap();
        symbol_sets
            .get(id)
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound(format!("SymbolSet with id {} not found", id)))
    }

    async fn list_symbol_sets(&self, category: Option<&str>) -> RepositoryResult<Vec<SymbolSet>> {
        let symbol_sets = self.symbol_sets.lock().unwrap();

        let result = match category {
            Some(cat) => symbol_sets
                .values()
                .filter(|s| s.category == cat)
                .cloned()
                .collect(),
            None => symbol_sets.values().cloned().collect(),
        };

        Ok(result)
    }

    async fn search_symbol_sets(&self, query: &str) -> RepositoryResult<Vec<SymbolSet>> {
        let symbol_sets = self.symbol_sets.lock().unwrap();
        let query = query.to_lowercase();

        let result = symbol_sets
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query)
                    || s.description.to_lowercase().contains(&query)
                    || s.symbols.values().any(|sym| {
                        sym.name.to_lowercase().contains(&query)
                            || sym.description.to_lowercase().contains(&query)
                    })
            })
            .cloned()
            .collect();

        Ok(result)
    }

    async fn create_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet> {
        let mut symbol_sets = self.symbol_sets.lock().unwrap();

        if symbol_sets.contains_key(&symbol_set.id) {
            return Err(RepositoryError::Conflict(format!(
                "SymbolSet with id {} already exists",
                symbol_set.id
            )));
        }

        let cloned = symbol_set.clone();
        symbol_sets.insert(symbol_set.id.clone(), symbol_set);

        Ok(cloned)
    }

    async fn update_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet> {
        let mut symbol_sets = self.symbol_sets.lock().unwrap();

        if !symbol_sets.contains_key(&symbol_set.id) {
            return Err(RepositoryError::NotFound(format!(
                "SymbolSet with id {} not found",
                symbol_set.id
            )));
        }

        let cloned = symbol_set.clone();
        symbol_sets.insert(symbol_set.id.clone(), symbol_set);

        Ok(cloned)
    }

    async fn delete_symbol_set(&self, id: &str) -> RepositoryResult<()> {
        let mut symbol_sets = self.symbol_sets.lock().unwrap();

        if !symbol_sets.contains_key(id) {
            return Err(RepositoryError::NotFound(format!(
                "SymbolSet with id {} not found",
                id
            )));
        }

        symbol_sets.remove(id);

        Ok(())
    }
}
