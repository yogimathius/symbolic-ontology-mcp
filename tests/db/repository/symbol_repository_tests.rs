use async_trait::async_trait;
use dream_ontology_mcp::db::repository::{
    PgSymbolRepository,
    interfaces::{RepositoryError, RepositoryResult, SymbolRepository},
};
use dream_ontology_mcp::domain::Symbol;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

struct TestSymbolRepository {
    symbols: Arc<RwLock<HashMap<String, Symbol>>>,
    should_fail: Arc<RwLock<bool>>,
}

impl TestSymbolRepository {
    fn new() -> Self {
        Self {
            symbols: Arc::new(RwLock::new(HashMap::new())),
            should_fail: Arc::new(RwLock::new(false)),
        }
    }

    fn with_test_data(self) -> Self {
        let mut symbols = HashMap::new();

        let water = Symbol::new(
            "water".to_string(),
            "Water".to_string(),
            "dream".to_string(),
            "Symbol of life and emotion".to_string(),
        );
        symbols.insert(water.id.clone(), water);

        let fire = Symbol::new(
            "fire".to_string(),
            "Fire".to_string(),
            "dream".to_string(),
            "Symbol of transformation".to_string(),
        );
        symbols.insert(fire.id.clone(), fire);

        let mountain = Symbol::new(
            "mountain".to_string(),
            "Mountain".to_string(),
            "dream".to_string(),
            "Symbol of obstacles or achievement".to_string(),
        );
        symbols.insert(mountain.id.clone(), mountain);

        *self.symbols.write().unwrap() = symbols;
        self
    }

    fn set_fail(&self, should_fail: bool) {
        *self.should_fail.write().unwrap() = should_fail;
    }

    fn check_failure(&self) -> Result<(), RepositoryError> {
        if *self.should_fail.read().unwrap() {
            return Err(RepositoryError::Internal("Simulated failure".to_string()));
        }
        Ok(())
    }
}

#[async_trait]
impl SymbolRepository for TestSymbolRepository {
    async fn get_symbol(&self, id: &str) -> RepositoryResult<Symbol> {
        self.check_failure()?;

        let symbols = self.symbols.read().unwrap();
        symbols
            .get(id)
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound(format!("Symbol with id {} not found", id)))
    }

    async fn list_symbols(&self, category: Option<&str>) -> RepositoryResult<Vec<Symbol>> {
        self.check_failure()?;

        let symbols = self.symbols.read().unwrap();

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

        let symbols = self.symbols.read().unwrap();
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

        let mut symbols = self.symbols.write().unwrap();

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

        let mut symbols = self.symbols.write().unwrap();

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

        let mut symbols = self.symbols.write().unwrap();

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

#[tokio::test]
async fn test_pg_symbol_repository_new() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(1))
        .connect_lazy("postgres://mock:mock@mock/mock")
        .expect("Failed to create mock pool");

    let _repo = PgSymbolRepository::new(pool);
    assert!(true, "Successfully created repository");
}

#[tokio::test]
async fn test_get_symbol_success() {
    let repo = TestSymbolRepository::new().with_test_data();

    let result = repo.get_symbol("water").await;

    assert!(result.is_ok());
    let symbol = result.unwrap();
    assert_eq!(symbol.id, "water");
    assert_eq!(symbol.name, "Water");
    assert_eq!(symbol.category, "dream");
}

#[tokio::test]
async fn test_get_symbol_not_found() {
    let repo = TestSymbolRepository::new().with_test_data();

    let result = repo.get_symbol("nonexistent").await;

    assert!(result.is_err());
    match result {
        Err(RepositoryError::NotFound(_)) => assert!(true),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_list_symbols_all() {
    let repo = TestSymbolRepository::new().with_test_data();

    let result = repo.list_symbols(None).await;

    assert!(result.is_ok());
    let _symbols = result.unwrap();
}

#[tokio::test]
async fn test_list_symbols_by_category() {
    let repo = TestSymbolRepository::new().with_test_data();

    let result = repo.list_symbols(Some("dream")).await;

    assert!(result.is_ok());
    let symbols = result.unwrap();

    for symbol in symbols {
        assert_eq!(symbol.category, "dream");
    }
}

#[tokio::test]
async fn test_search_symbols() {
    let repo = TestSymbolRepository::new().with_test_data();

    let result = repo.search_symbols("water").await;

    assert!(result.is_ok());
    let symbols = result.unwrap();
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].id, "water");
}

#[tokio::test]
async fn test_create_symbol_success() {
    let repo = TestSymbolRepository::new();
    let new_symbol = Symbol::new(
        "earth".to_string(),
        "Earth".to_string(),
        "element".to_string(),
        "Symbol of stability".to_string(),
    );

    let result = repo.create_symbol(new_symbol.clone()).await;

    assert!(result.is_ok());

    let get_result = repo.get_symbol("earth").await;
    assert!(get_result.is_ok());
    let saved_symbol = get_result.unwrap();
    assert_eq!(saved_symbol.id, "earth");
    assert_eq!(saved_symbol.name, "Earth");
    assert_eq!(saved_symbol.category, "element");
}

#[tokio::test]
async fn test_create_symbol_conflict() {
    let repo = TestSymbolRepository::new().with_test_data();
    let new_symbol = Symbol::new(
        "water".to_string(),
        "Water Element".to_string(),
        "element".to_string(),
        "Symbol of life and emotion".to_string(),
    );

    let result = repo.create_symbol(new_symbol).await;

    assert!(result.is_err());
    match result {
        Err(RepositoryError::Conflict(_)) => assert!(true),
        _ => panic!("Expected Conflict error"),
    }
}

#[tokio::test]
async fn test_update_symbol_success() {
    let repo = TestSymbolRepository::new().with_test_data();

    let mut symbol = repo.get_symbol("water").await.unwrap();
    symbol.description = "Updated description for water".to_string();

    let result = repo.update_symbol(symbol.clone()).await;

    assert!(result.is_ok());

    let updated = repo.get_symbol("water").await.unwrap();
    assert_eq!(updated.description, "Updated description for water");
}

#[tokio::test]
async fn test_update_symbol_not_found() {
    let repo = TestSymbolRepository::new().with_test_data();
    let symbol = Symbol::new(
        "nonexistent".to_string(),
        "Nonexistent".to_string(),
        "test".to_string(),
        "This doesn't exist".to_string(),
    );

    let result = repo.update_symbol(symbol).await;

    assert!(result.is_err());
    match result {
        Err(RepositoryError::NotFound(_)) => assert!(true),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_delete_symbol_success() {
    let repo = TestSymbolRepository::new().with_test_data();

    let result = repo.delete_symbol("water").await;

    assert!(result.is_ok());

    let get_result = repo.get_symbol("water").await;
    assert!(get_result.is_err());
}

#[tokio::test]
async fn test_delete_symbol_not_found() {
    let repo = TestSymbolRepository::new().with_test_data();

    let result = repo.delete_symbol("nonexistent").await;

    assert!(result.is_err());
    match result {
        Err(RepositoryError::NotFound(_)) => assert!(true),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_repository_failure_handling() {
    let repo = TestSymbolRepository::new().with_test_data();
    repo.set_fail(true);

    let result = repo.get_symbol("water").await;

    assert!(result.is_err());
    match result {
        Err(RepositoryError::Internal(_)) => assert!(true),
        _ => panic!("Expected Internal error"),
    }
}
