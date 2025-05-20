use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use symbol_ontology_mcp::db::repository::interfaces::{
    Repository, RepositoryError, RepositoryResult, SymbolRepository, SymbolSetRepository,
};
use symbol_ontology_mcp::domain::{Symbol, SymbolSet};

use super::fixtures;

pub struct MockSymbolRepository {
    symbols: Arc<RwLock<HashMap<String, Symbol>>>,
    fail_next_operation: Arc<RwLock<Option<RepositoryError>>>,
}

impl Repository for MockSymbolRepository {}

impl MockSymbolRepository {
    pub fn new() -> Self {
        MockSymbolRepository {
            symbols: Arc::new(RwLock::new(HashMap::new())),
            fail_next_operation: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_test_data(self) -> Self {
        let test_symbols = vec![
            fixtures::create_test_symbol("water", "Water", "dream"),
            fixtures::create_test_symbol("fire", "Fire", "dream"),
            fixtures::create_test_symbol("mountain", "Mountain", "dream"),
        ];

        let mut symbols_map = HashMap::new();
        for symbol in test_symbols {
            symbols_map.insert(symbol.id.clone(), symbol);
        }
        *self.symbols.write().unwrap() = symbols_map;
        self
    }

    pub fn fail_next(&self, error: RepositoryError) {
        *self.fail_next_operation.write().unwrap() = Some(error);
    }

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
    async fn get_symbol(&self, id: &str) -> RepositoryResult<Symbol> {
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

    async fn list_symbols(&self, category: Option<&str>) -> RepositoryResult<Vec<Symbol>> {
        self.check_failure()?;

        let symbols = self.symbols.read().unwrap();
        let symbols: Vec<Symbol> = match category {
            Some(cat) => symbols
                .values()
                .filter(|s| s.category == cat)
                .cloned()
                .collect(),
            None => symbols.values().cloned().collect(),
        };

        Ok(symbols)
    }

    async fn search_symbols(&self, query: &str) -> RepositoryResult<Vec<Symbol>> {
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

    async fn create_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
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

    async fn update_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
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

    async fn delete_symbol(&self, id: &str) -> RepositoryResult<()> {
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

pub struct MockSymbolSetRepository {
    symbol_sets: Arc<RwLock<HashMap<String, SymbolSet>>>,
    fail_next_operation: Arc<RwLock<Option<RepositoryError>>>,
}

impl Repository for MockSymbolSetRepository {}

impl MockSymbolSetRepository {
    pub fn new() -> Self {
        MockSymbolSetRepository {
            symbol_sets: Arc::new(RwLock::new(HashMap::new())),
            fail_next_operation: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_test_data(self) -> Self {
        let test_sets = vec![
            fixtures::create_test_symbol_set(
                "dream-symbols",
                "Dream Symbols",
                "dream",
                "Common symbols in dreams",
            ),
            fixtures::create_test_symbol_set(
                "myth-symbols",
                "Mythological Symbols",
                "mythological",
                "Symbols from mythology",
            ),
        ];

        let mut sets_map = HashMap::new();
        for set in test_sets {
            sets_map.insert(set.id.clone(), set);
        }
        *self.symbol_sets.write().unwrap() = sets_map;
        self
    }

    pub fn fail_next(&self, error: RepositoryError) {
        *self.fail_next_operation.write().unwrap() = Some(error);
    }

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
    async fn get_symbol_set(&self, id: &str) -> RepositoryResult<SymbolSet> {
        self.check_failure()?;

        let sets = self.symbol_sets.read().unwrap();
        sets.get(id)
            .cloned()
            .ok_or(RepositoryError::NotFound(format!(
                "SymbolSet with ID {} not found",
                id
            )))
    }

    async fn list_symbol_sets(&self, category: Option<&str>) -> RepositoryResult<Vec<SymbolSet>> {
        self.check_failure()?;

        let sets = self.symbol_sets.read().unwrap();
        let sets: Vec<SymbolSet> = match category {
            Some(cat) => sets
                .values()
                .filter(|s| s.category == cat)
                .cloned()
                .collect(),
            None => sets.values().cloned().collect(),
        };

        Ok(sets)
    }

    async fn search_symbol_sets(&self, query: &str) -> RepositoryResult<Vec<SymbolSet>> {
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

    async fn create_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet> {
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

    async fn update_symbol_set(&self, symbol_set: SymbolSet) -> RepositoryResult<SymbolSet> {
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

    async fn delete_symbol_set(&self, id: &str) -> RepositoryResult<()> {
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
        let symbol_repo = MockSymbolRepository::new().with_test_data();
        let symbol_set_repo = MockSymbolSetRepository::new().with_test_data();

        MockRepositoryFactory {
            symbol_repository: Arc::new(symbol_repo),
            symbol_set_repository: Arc::new(symbol_set_repo),
        }
    }
}
