use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use dream_ontology_mcp::domain::{
    RepositoryError, RepositoryFactory, RepositoryResult, Symbol, SymbolRepository, SymbolSet,
    SymbolSetRepository,
};

use super::fixtures;

/// Mock repository that allows customizing behavior for tests
pub struct MockSymbolRepository {
    symbols: Arc<RwLock<HashMap<String, Symbol>>>,
    fail_next_operation: Arc<RwLock<Option<RepositoryError>>>,
}

impl MockSymbolRepository {
    pub fn new() -> Self {
        MockSymbolRepository {
            symbols: Arc::new(RwLock::new(HashMap::new())),
            fail_next_operation: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_test_data(mut self) -> Self {
        let test_symbols = fixtures::create_test_symbols();
        let mut symbols_map = HashMap::new();
        for symbol in test_symbols {
            symbols_map.insert(symbol.id.clone(), symbol);
        }
        *self.symbols.write().unwrap() = symbols_map;
        self
    }

    /// Makes the next repository operation fail with the specified error
    pub fn fail_next(&self, error: RepositoryError) {
        *self.fail_next_operation.write().unwrap() = Some(error);
    }

    /// Helper to check and consume the next failure if one is set
    fn check_failure(&self) -> Result<(), RepositoryError> {
        let mut fail_guard = self.fail_next_operation.write().unwrap();
        if let Some(err) = fail_guard.take() {
            return Err(err);
        }
        Ok(())
    }
}

#[async_trait]
impl SymbolRepository for MockSymbolRepository {
    async fn get(&self, id: &str) -> RepositoryResult<Symbol> {
        self.check_failure()?;

        let symbols = self.symbols.read().unwrap();
        symbols
            .get(id)
            .cloned()
            .ok_or(RepositoryError::NotFound(format!(
                "Symbol with ID {} not found",
                id
            )))
    }

    async fn list(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> RepositoryResult<Vec<Symbol>> {
        self.check_failure()?;

        let symbols = self.symbols.read().unwrap();
        let offset = offset.unwrap_or(0);
        let symbols: Vec<Symbol> = symbols.values().cloned().collect();

        if let Some(limit) = limit {
            Ok(symbols.into_iter().skip(offset).take(limit).collect())
        } else {
            Ok(symbols.into_iter().skip(offset).collect())
        }
    }

    async fn search(&self, query: &str) -> RepositoryResult<Vec<Symbol>> {
        self.check_failure()?;

        let symbols = self.symbols.read().unwrap();
        let query = query.to_lowercase();

        let results: Vec<Symbol> = symbols
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query)
                    || s.description.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();

        Ok(results)
    }

    async fn create(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        self.check_failure()?;

        let mut symbols = self.symbols.write().unwrap();
        if symbols.contains_key(&symbol.id) {
            return Err(RepositoryError::Conflict(format!(
                "Symbol with ID {} already exists",
                symbol.id
            )));
        }

        let symbol_clone = symbol.clone();
        symbols.insert(symbol.id.clone(), symbol);
        Ok(symbol_clone)
    }

    async fn update(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        self.check_failure()?;

        let mut symbols = self.symbols.write().unwrap();
        if !symbols.contains_key(&symbol.id) {
            return Err(RepositoryError::NotFound(format!(
                "Symbol with ID {} not found for update",
                symbol.id
            )));
        }

        let symbol_clone = symbol.clone();
        symbols.insert(symbol.id.clone(), symbol);
        Ok(symbol_clone)
    }

    async fn delete(&self, id: &str) -> RepositoryResult<()> {
        self.check_failure()?;

        let mut symbols = self.symbols.write().unwrap();
        if !symbols.contains_key(id) {
            return Err(RepositoryError::NotFound(format!(
                "Symbol with ID {} not found for deletion",
                id
            )));
        }

        symbols.remove(id);
        Ok(())
    }
}

/// Mock SymbolSet repository for testing
pub struct MockSymbolSetRepository {
    symbol_sets: Arc<RwLock<HashMap<String, SymbolSet>>>,
    fail_next_operation: Arc<RwLock<Option<RepositoryError>>>,
}

impl MockSymbolSetRepository {
    pub fn new() -> Self {
        MockSymbolSetRepository {
            symbol_sets: Arc::new(RwLock::new(HashMap::new())),
            fail_next_operation: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_test_data(mut self) -> Self {
        let test_sets = fixtures::create_test_symbol_sets();
        let mut sets_map = HashMap::new();
        for set in test_sets {
            sets_map.insert(set.id.clone(), set);
        }
        *self.symbol_sets.write().unwrap() = sets_map;
        self
    }

    /// Makes the next repository operation fail with the specified error
    pub fn fail_next(&self, error: RepositoryError) {
        *self.fail_next_operation.write().unwrap() = Some(error);
    }

    /// Helper to check and consume the next failure if one is set
    fn check_failure(&self) -> Result<(), RepositoryError> {
        let mut fail_guard = self.fail_next_operation.write().unwrap();
        if let Some(err) = fail_guard.take() {
            return Err(err);
        }
        Ok(())
    }
}

#[async_trait]
impl SymbolSetRepository for MockSymbolSetRepository {
    async fn get(&self, id: &str) -> RepositoryResult<SymbolSet> {
        self.check_failure()?;

        let sets = self.symbol_sets.read().unwrap();
        sets.get(id)
            .cloned()
            .ok_or(RepositoryError::NotFound(format!(
                "SymbolSet with ID {} not found",
                id
            )))
    }

    async fn list(
        &self,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> RepositoryResult<Vec<SymbolSet>> {
        self.check_failure()?;

        let sets = self.symbol_sets.read().unwrap();
        let offset = offset.unwrap_or(0);
        let sets: Vec<SymbolSet> = sets.values().cloned().collect();

        if let Some(limit) = limit {
            Ok(sets.into_iter().skip(offset).take(limit).collect())
        } else {
            Ok(sets.into_iter().skip(offset).collect())
        }
    }

    async fn search(&self, query: &str) -> RepositoryResult<Vec<SymbolSet>> {
        self.check_failure()?;

        let sets = self.symbol_sets.read().unwrap();
        let query = query.to_lowercase();

        let results: Vec<SymbolSet> = sets
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query)
                    || s.description.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();

        Ok(results)
    }

    async fn create(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet> {
        self.check_failure()?;

        let mut sets = self.symbol_sets.write().unwrap();
        if sets.contains_key(&symbol_set.id) {
            return Err(RepositoryError::Conflict(format!(
                "SymbolSet with ID {} already exists",
                symbol_set.id
            )));
        }

        let set_clone = symbol_set.clone();
        sets.insert(symbol_set.id.clone(), symbol_set);
        Ok(set_clone)
    }

    async fn update(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet> {
        self.check_failure()?;

        let mut sets = self.symbol_sets.write().unwrap();
        if !sets.contains_key(&symbol_set.id) {
            return Err(RepositoryError::NotFound(format!(
                "SymbolSet with ID {} not found for update",
                symbol_set.id
            )));
        }

        let set_clone = symbol_set.clone();
        sets.insert(symbol_set.id.clone(), symbol_set);
        Ok(set_clone)
    }

    async fn delete(&self, id: &str) -> RepositoryResult<()> {
        self.check_failure()?;

        let mut sets = self.symbol_sets.write().unwrap();
        if !sets.contains_key(id) {
            return Err(RepositoryError::NotFound(format!(
                "SymbolSet with ID {} not found for deletion",
                id
            )));
        }

        sets.remove(id);
        Ok(())
    }
}

/// Mock repository factory for testing
pub struct MockRepositoryFactory {
    symbol_repository: Arc<MockSymbolRepository>,
    symbol_set_repository: Arc<MockSymbolSetRepository>,
}

impl MockRepositoryFactory {
    pub fn new() -> Self {
        MockRepositoryFactory {
            symbol_repository: Arc::new(MockSymbolRepository::new()),
            symbol_set_repository: Arc::new(MockSymbolSetRepository::new()),
        }
    }

    pub fn with_test_data(self) -> Self {
        MockRepositoryFactory {
            symbol_repository: Arc::new(self.symbol_repository.with_test_data()),
            symbol_set_repository: Arc::new(self.symbol_set_repository.with_test_data()),
        }
    }
}

impl RepositoryFactory for MockRepositoryFactory {
    fn create_symbol_repository(&self) -> Arc<dyn SymbolRepository> {
        self.symbol_repository.clone()
    }

    fn create_symbol_set_repository(&self) -> Arc<dyn SymbolSetRepository> {
        self.symbol_set_repository.clone()
    }
}
