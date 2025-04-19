use crate::domain::{
    RepositoryError, RepositoryFactory, RepositoryResult, Symbol, SymbolRepository, SymbolSet,
    SymbolSetRepository,
};
use async_trait;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Generic repository trait for basic CRUD operations
pub trait Repository<K, V> {
    /// Get an item by its key
    #[allow(dead_code)]
    fn get(&self, key: &K) -> Result<Option<V>, RepositoryError>;

    /// Save an item with the given key
    #[allow(dead_code)]
    fn save(&self, key: K, value: V) -> Result<(), RepositoryError>;

    /// Delete an item by its key, returns true if item was deleted
    #[allow(dead_code)]
    fn delete(&self, key: &K) -> Result<bool, RepositoryError>;

    /// List all items as key-value pairs
    #[allow(dead_code)]
    fn list(&self) -> Result<Vec<(K, V)>, RepositoryError>;
}

/// In-memory implementation of the repositories for testing and development
#[derive(Default, Clone)]
pub struct MemoryRepositoryFactory {
    symbols: Arc<RwLock<HashMap<String, Symbol>>>,
    symbol_sets: Arc<RwLock<HashMap<String, SymbolSet>>>,
}

impl MemoryRepositoryFactory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_test_data(self) -> Self {
        // Load test data
        let test_symbols = vec![
            Symbol::new(
                "sun".to_string(),
                "Sun".to_string(),
                "nature".to_string(),
                "Celestial body at the center of our solar system".to_string(),
            )
            .with_category("nature")
            .with_related(vec!["light", "day"]),
            Symbol::new(
                "moon".to_string(),
                "Moon".to_string(),
                "nature".to_string(),
                "Natural satellite of Earth".to_string(),
            )
            .with_category("nature")
            .with_related(vec!["night", "tide"]),
            Symbol::new(
                "light".to_string(),
                "Light".to_string(),
                "concept".to_string(),
                "Electromagnetic radiation visible to the human eye".to_string(),
            )
            .with_category("concept")
            .with_related(vec!["sun", "illumination"]),
            Symbol::new(
                "dark".to_string(),
                "Darkness".to_string(),
                "concept".to_string(),
                "Absence of light".to_string(),
            )
            .with_category("concept")
            .with_related(vec!["night", "shadow"]),
            Symbol::new(
                "tree".to_string(),
                "Tree".to_string(),
                "nature".to_string(),
                "Perennial plant with an elongated stem and branches".to_string(),
            )
            .with_category("nature")
            .with_related(vec!["forest", "wood"]),
        ];

        let test_sets = vec![
            SymbolSet::new(
                "celestial".to_string(),
                "Celestial Bodies".to_string(),
                "nature".to_string(),
                "Celestial bodies and phenomena".to_string(),
            )
            .with_symbols(vec!["sun", "moon", "star"]),
            SymbolSet::new(
                "opposites".to_string(),
                "Opposing Concepts".to_string(),
                "concept".to_string(),
                "Paired opposing concepts".to_string(),
            )
            .with_symbols(vec!["light", "dark", "day", "night"]),
        ];

        // Insert test data
        {
            let mut symbols = self.symbols.write().unwrap();
            for symbol in test_symbols {
                symbols.insert(symbol.id.clone(), symbol);
            }

            let mut symbol_sets = self.symbol_sets.write().unwrap();
            for set in test_sets {
                symbol_sets.insert(set.id.clone(), set);
            }
        }

        self
    }
}

impl RepositoryFactory for MemoryRepositoryFactory {
    fn create_symbol_repository(&self) -> Arc<dyn SymbolRepository> {
        Arc::new(MemorySymbolRepository {
            data: Arc::clone(&self.symbols),
        })
    }

    fn create_symbol_set_repository(&self) -> Arc<dyn SymbolSetRepository> {
        Arc::new(MemorySymbolSetRepository {
            data: Arc::clone(&self.symbol_sets),
        })
    }
}

/// In-memory implementation of SymbolRepository
struct MemorySymbolRepository {
    data: Arc<RwLock<HashMap<String, Symbol>>>,
}

#[async_trait::async_trait]
impl SymbolRepository for MemorySymbolRepository {
    async fn get_symbol(&self, id: &str) -> RepositoryResult<Symbol> {
        let symbols = self.data.read().unwrap();
        symbols
            .get(id)
            .cloned()
            .ok_or(RepositoryError::NotFound(format!(
                "Symbol not found: {}",
                id
            )))
    }

    async fn list_symbols(&self, category: Option<&str>) -> RepositoryResult<Vec<Symbol>> {
        let symbols = self.data.read().unwrap();
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
        let symbols = self.data.read().unwrap();
        let query = query.to_lowercase();

        let result = symbols
            .values()
            .filter(|s| {
                s.id.to_lowercase().contains(&query)
                    || s.description.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();

        Ok(result)
    }

    async fn create_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        let mut symbols = self.data.write().unwrap();

        // Check if symbol already exists
        if symbols.contains_key(&symbol.id) {
            return Err(RepositoryError::Conflict(format!(
                "Symbol already exists: {}",
                symbol.id
            )));
        }

        // Insert the new symbol
        let result = symbol.clone();
        symbols.insert(symbol.id.clone(), symbol);

        Ok(result)
    }

    async fn update_symbol(&self, symbol: Symbol) -> RepositoryResult<Symbol> {
        let mut symbols = self.data.write().unwrap();

        // Check if symbol exists
        if !symbols.contains_key(&symbol.id) {
            return Err(RepositoryError::NotFound(format!(
                "Symbol not found: {}",
                symbol.id
            )));
        }

        // Update the symbol
        let result = symbol.clone();
        symbols.insert(symbol.id.clone(), symbol);

        Ok(result)
    }

    async fn delete_symbol(&self, id: &str) -> RepositoryResult<()> {
        let mut symbols = self.data.write().unwrap();

        // Check if symbol exists
        if !symbols.contains_key(id) {
            return Err(RepositoryError::NotFound(format!(
                "Symbol not found: {}",
                id
            )));
        }

        // Remove the symbol
        symbols.remove(id);

        Ok(())
    }
}

/// In-memory implementation of SymbolSetRepository
struct MemorySymbolSetRepository {
    data: Arc<RwLock<HashMap<String, SymbolSet>>>,
}

#[async_trait::async_trait]
impl SymbolSetRepository for MemorySymbolSetRepository {
    async fn get_symbol_set(&self, id: &str) -> RepositoryResult<SymbolSet> {
        let sets = self.data.read().unwrap();
        sets.get(id)
            .cloned()
            .ok_or(RepositoryError::NotFound(format!(
                "SymbolSet not found: {}",
                id
            )))
    }

    async fn list_symbol_sets(&self, category: Option<&str>) -> RepositoryResult<Vec<SymbolSet>> {
        let sets = self.data.read().unwrap();
        let result = match category {
            Some(cat) => sets
                .values()
                .filter(|s| s.category == cat)
                .cloned()
                .collect(),
            None => sets.values().cloned().collect(),
        };
        Ok(result)
    }

    async fn search_symbol_sets(&self, query: &str) -> RepositoryResult<Vec<SymbolSet>> {
        let sets = self.data.read().unwrap();
        let query = query.to_lowercase();

        let result = sets
            .values()
            .filter(|s| {
                s.id.to_lowercase().contains(&query)
                    || s.description.to_lowercase().contains(&query)
            })
            .cloned()
            .collect();

        Ok(result)
    }

    async fn create_symbol_set(&self, set: SymbolSet) -> RepositoryResult<SymbolSet> {
        let mut sets = self.data.write().unwrap();

        // Check if set already exists
        if sets.contains_key(&set.id) {
            return Err(RepositoryError::Conflict(format!(
                "SymbolSet already exists: {}",
                set.id
            )));
        }

        // Insert the new set
        let result = set.clone();
        sets.insert(set.id.clone(), set);

        Ok(result)
    }

    async fn update_symbol_set(&self, set: SymbolSet) -> RepositoryResult<SymbolSet> {
        let mut sets = self.data.write().unwrap();

        // Check if set exists
        if !sets.contains_key(&set.id) {
            return Err(RepositoryError::NotFound(format!(
                "SymbolSet not found: {}",
                set.id
            )));
        }

        // Update the set
        let result = set.clone();
        sets.insert(set.id.clone(), set);

        Ok(result)
    }

    async fn delete_symbol_set(&self, id: &str) -> RepositoryResult<()> {
        let mut sets = self.data.write().unwrap();

        // Check if set exists
        if !sets.contains_key(id) {
            return Err(RepositoryError::NotFound(format!(
                "SymbolSet not found: {}",
                id
            )));
        }

        // Remove the set
        sets.remove(id);

        Ok(())
    }
}

/// An in-memory implementation of the Repository trait.
/// This is primarily useful for testing and prototyping.
pub struct MemoryRepository<K, V> {
    data: Arc<RwLock<HashMap<K, V>>>,
}

impl<K, V> MemoryRepository<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    /// Create a new empty repository
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a repository with initial data
    #[allow(dead_code)]
    pub fn with_data(initial_data: HashMap<K, V>) -> Self {
        Self {
            data: Arc::new(RwLock::new(initial_data)),
        }
    }
}

impl<K, V> Repository<K, V> for MemoryRepository<K, V>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    fn get(&self, key: &K) -> Result<Option<V>, RepositoryError> {
        match self.data.read() {
            Ok(data) => Ok(data.get(key).cloned()),
            Err(_) => Err(RepositoryError::Internal(
                "Failed to acquire lock".to_string(),
            )),
        }
    }

    fn save(&self, key: K, value: V) -> Result<(), RepositoryError> {
        match self.data.write() {
            Ok(mut data) => {
                data.insert(key, value);
                Ok(())
            }
            Err(_) => Err(RepositoryError::Internal(
                "Failed to acquire lock".to_string(),
            )),
        }
    }

    fn delete(&self, key: &K) -> Result<bool, RepositoryError> {
        match self.data.write() {
            Ok(mut data) => Ok(data.remove(key).is_some()),
            Err(_) => Err(RepositoryError::Internal(
                "Failed to acquire lock".to_string(),
            )),
        }
    }

    fn list(&self) -> Result<Vec<(K, V)>, RepositoryError> {
        match self.data.read() {
            Ok(data) => {
                let items = data.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                Ok(items)
            }
            Err(_) => Err(RepositoryError::Internal(
                "Failed to acquire lock".to_string(),
            )),
        }
    }
}
