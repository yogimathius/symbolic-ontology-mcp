use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use ontology_core::db::repository::interfaces::{
    Repository, RepositoryError, RepositoryResult, SymbolRepository, SymbolSetRepository,
};
use ontology_core::domain::{Symbol, SymbolSet};

use super::fixtures;

pub struct MockRepository<T> {
    items: Arc<RwLock<HashMap<String, T>>>,
    should_fail: Arc<RwLock<bool>>,
}

impl<T> Repository for MockRepository<T> {}

impl<T> MockRepository<T> {
    pub fn new() -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            should_fail: Arc::new(RwLock::new(false)),
        }
    }

    pub fn with_items(self, items: HashMap<String, T>) -> Self {
        *self.items.write().unwrap() = items;
        self
    }

    pub fn set_fail(&self, should_fail: bool) {
        *self.should_fail.write().unwrap() = should_fail;
    }

    pub fn check_failure(&self) -> Result<(), RepositoryError> {
        if *self.should_fail.read().unwrap() {
            return Err(RepositoryError::Internal("Simulated failure".to_string()));
        }
        Ok(())
    }
}

#[async_trait]
impl SymbolRepository for MockRepository<Symbol> {
    async fn get_symbol(&self, id: &str) -> RepositoryResult<Symbol> {
        self.check_failure()?;

        let symbols = self.items.read().unwrap();
        symbols
            .get(id)
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound(format!("Symbol with id {} not found", id)))
    }

    async fn list_symbols(&self, category: Option<&str>) -> RepositoryResult<Vec<Symbol>> {
        self.check_failure()?;

        let symbols = self.items.read().unwrap();

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
        self.check_failure()?;

        let symbols = self.items.read().unwrap();
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
        self.check_failure()?;

        let mut symbols = self.items.write().unwrap();

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
        self.check_failure()?;

        let mut symbols = self.items.write().unwrap();

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
        self.check_failure()?;

        let mut symbols = self.items.write().unwrap();

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

#[async_trait]
impl SymbolSetRepository for MockRepository<SymbolSet> {
    async fn get_symbol_set(&self, id: &str) -> RepositoryResult<SymbolSet> {
        self.check_failure()?;

        let symbol_sets = self.items.read().unwrap();
        symbol_sets
            .get(id)
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound(format!("SymbolSet with id {} not found", id)))
    }

    async fn list_symbol_sets(&self, category: Option<&str>) -> RepositoryResult<Vec<SymbolSet>> {
        self.check_failure()?;

        let symbol_sets = self.items.read().unwrap();

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
        self.check_failure()?;

        let symbol_sets = self.items.read().unwrap();
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
        self.check_failure()?;

        let mut symbol_sets = self.items.write().unwrap();

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
        self.check_failure()?;

        let mut symbol_sets = self.items.write().unwrap();

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
        self.check_failure()?;

        let mut symbol_sets = self.items.write().unwrap();

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

pub struct MockRepositoryFactory {
    symbol_repository: Arc<MockRepository<Symbol>>,
    symbol_set_repository: Arc<MockRepository<SymbolSet>>,
}

impl MockRepositoryFactory {
    pub fn new() -> Self {
        MockRepositoryFactory {
            symbol_repository: Arc::new(MockRepository::new()),
            symbol_set_repository: Arc::new(MockRepository::new()),
        }
    }

    pub fn with_test_data(self) -> Self {
        let symbol_repo = MockRepository::new().with_items(fixtures::create_test_symbols());
        let symbol_set_repo = MockRepository::new().with_items(fixtures::create_test_symbol_sets());

        MockRepositoryFactory {
            symbol_repository: Arc::new(symbol_repo),
            symbol_set_repository: Arc::new(symbol_set_repo),
        }
    }
}
