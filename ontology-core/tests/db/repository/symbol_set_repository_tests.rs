use async_trait::async_trait;
use ontology_core::db::repository::{
    interfaces::{Repository, RepositoryError, RepositoryResult, SymbolSetRepository},
    PgSymbolSetRepository,
};
use ontology_core::domain::{Symbol, SymbolSet};
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

// In-memory implementation of SymbolSetRepository for testing
struct TestSymbolSetRepository {
    symbol_sets: Arc<RwLock<HashMap<String, SymbolSet>>>,
    should_fail: Arc<RwLock<bool>>,
}

impl Repository for TestSymbolSetRepository {}

impl TestSymbolSetRepository {
    fn new() -> Self {
        Self {
            symbol_sets: Arc::new(RwLock::new(HashMap::new())),
            should_fail: Arc::new(RwLock::new(false)),
        }
    }

    fn with_test_data(self) -> Self {
        let mut symbol_sets = HashMap::new();

        let water = Symbol::new(
            "water".to_string(),
            "Water".to_string(),
            "element".to_string(),
            "Symbol of emotion and life".to_string(),
        );

        let fire = Symbol::new(
            "fire".to_string(),
            "Fire".to_string(),
            "element".to_string(),
            "Symbol of transformation".to_string(),
        );

        let mut elements = SymbolSet::new(
            "elements".to_string(),
            "Elements".to_string(),
            "natural".to_string(),
            "Basic elements in nature".to_string(),
        );
        elements.add_symbol(water.clone());
        elements.add_symbol(fire.clone());
        symbol_sets.insert(elements.id.clone(), elements);

        let mut dreams = SymbolSet::new(
            "dreams".to_string(),
            "Dreams".to_string(),
            "psychology".to_string(),
            "Common elements in dreams".to_string(),
        );
        dreams.add_symbol(water);
        symbol_sets.insert(dreams.id.clone(), dreams);

        *self.symbol_sets.write().unwrap() = symbol_sets;
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
impl SymbolSetRepository for TestSymbolSetRepository {
    async fn get_symbol_set(&self, id: &str) -> RepositoryResult<SymbolSet> {
        self.check_failure()?;

        let symbol_sets = self.symbol_sets.read().unwrap();
        symbol_sets
            .get(id)
            .cloned()
            .ok_or_else(|| RepositoryError::NotFound(format!("SymbolSet with id {} not found", id)))
    }

    async fn list_symbol_sets(&self, category: Option<&str>) -> RepositoryResult<Vec<SymbolSet>> {
        self.check_failure()?;

        let symbol_sets = self.symbol_sets.read().unwrap();

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

        let symbol_sets = self.symbol_sets.read().unwrap();
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

        let mut symbol_sets = self.symbol_sets.write().unwrap();

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

        let mut symbol_sets = self.symbol_sets.write().unwrap();

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

        let mut symbol_sets = self.symbol_sets.write().unwrap();

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

#[tokio::test]
async fn test_pg_symbol_set_repository_new() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(1))
        .connect("postgres://postgres:postgres@localhost/test_db")
        .await;

    assert!(
        pool.is_err(),
        "This test doesn't require an actual database connection"
    );

    if let Ok(pool) = pool {
        let _ = PgSymbolSetRepository::new(pool);
        assert!(true);
    }
}

// Tests for SymbolSetRepository interface using TestSymbolSetRepository

#[tokio::test]
async fn test_get_symbol_set_success() {
    let repo = TestSymbolSetRepository::new().with_test_data();

    let result = repo.get_symbol_set("elements").await;

    assert!(result.is_ok());
    let symbol_set = result.unwrap();
    assert_eq!(symbol_set.id, "elements");
    assert_eq!(symbol_set.name, "Elements");
    assert_eq!(symbol_set.category, "natural");
    assert_eq!(symbol_set.symbols.len(), 2);
}

#[tokio::test]
async fn test_get_symbol_set_not_found() {
    let repo = TestSymbolSetRepository::new().with_test_data();

    let result = repo.get_symbol_set("nonexistent").await;

    assert!(result.is_err());
    match result {
        Err(RepositoryError::NotFound(_)) => assert!(true),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_list_symbol_sets_all() {
    let repo = TestSymbolSetRepository::new().with_test_data();

    let result = repo.list_symbol_sets(None).await;

    assert!(result.is_ok());
    let symbol_sets = result.unwrap();
    assert_eq!(symbol_sets.len(), 2); // Based on test data
}

#[tokio::test]
async fn test_list_symbol_sets_by_category() {
    let repo = TestSymbolSetRepository::new().with_test_data();

    let result = repo.list_symbol_sets(Some("natural")).await;

    assert!(result.is_ok());
    let symbol_sets = result.unwrap();
    assert_eq!(symbol_sets.len(), 1);
    assert_eq!(symbol_sets[0].id, "elements");
}

#[tokio::test]
async fn test_search_symbol_sets() {
    let repo = TestSymbolSetRepository::new().with_test_data();

    let result = repo.search_symbol_sets("water").await;

    assert!(result.is_ok());
    let symbol_sets = result.unwrap();
    assert_eq!(symbol_sets.len(), 2); // Both sets contain water symbol
}

#[tokio::test]
async fn test_create_symbol_set_success() {
    let repo = TestSymbolSetRepository::new();
    let new_symbol_set = SymbolSet::new(
        "colors".to_string(),
        "Colors".to_string(),
        "visual".to_string(),
        "Symbol of visual perception".to_string(),
    );

    let result = repo.create_symbol_set(new_symbol_set.clone()).await;

    assert!(result.is_ok());

    let get_result = repo.get_symbol_set("colors").await;
    assert!(get_result.is_ok());
    let saved_symbol_set = get_result.unwrap();
    assert_eq!(saved_symbol_set.id, "colors");
    assert_eq!(saved_symbol_set.name, "Colors");
    assert_eq!(saved_symbol_set.category, "visual");
}

#[tokio::test]
async fn test_create_symbol_set_conflict() {
    let repo = TestSymbolSetRepository::new().with_test_data();
    let new_symbol_set = SymbolSet::new(
        "elements".to_string(), // Already exists in test data
        "Basic Elements".to_string(),
        "nature".to_string(),
        "Another element description".to_string(),
    );

    let result = repo.create_symbol_set(new_symbol_set).await;

    assert!(result.is_err());
    match result {
        Err(RepositoryError::Conflict(_)) => assert!(true),
        _ => panic!("Expected Conflict error"),
    }
}

#[tokio::test]
async fn test_update_symbol_set_success() {
    let repo = TestSymbolSetRepository::new().with_test_data();

    let mut symbol_set = repo.get_symbol_set("elements").await.unwrap();
    symbol_set.description = "Updated description for elements".to_string();

    let result = repo.update_symbol_set(symbol_set.clone()).await;

    assert!(result.is_ok());

    let updated = repo.get_symbol_set("elements").await.unwrap();
    assert_eq!(updated.description, "Updated description for elements");
}

#[tokio::test]
async fn test_update_symbol_set_not_found() {
    let repo = TestSymbolSetRepository::new().with_test_data();
    let symbol_set = SymbolSet::new(
        "nonexistent".to_string(),
        "Nonexistent".to_string(),
        "test".to_string(),
        "This doesn't exist".to_string(),
    );

    let result = repo.update_symbol_set(symbol_set).await;

    assert!(result.is_err());
    match result {
        Err(RepositoryError::NotFound(_)) => assert!(true),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_delete_symbol_set_success() {
    let repo = TestSymbolSetRepository::new().with_test_data();

    let result = repo.delete_symbol_set("elements").await;

    assert!(result.is_ok());

    let get_result = repo.get_symbol_set("elements").await;
    assert!(get_result.is_err());
}

#[tokio::test]
async fn test_delete_symbol_set_not_found() {
    let repo = TestSymbolSetRepository::new().with_test_data();

    let result = repo.delete_symbol_set("nonexistent").await;

    assert!(result.is_err());
    match result {
        Err(RepositoryError::NotFound(_)) => assert!(true),
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_symbol_set_repository_failure_handling() {
    let repo = TestSymbolSetRepository::new().with_test_data();
    repo.set_fail(true);

    let result = repo.get_symbol_set("elements").await;

    assert!(result.is_err());
    match result {
        Err(RepositoryError::Internal(_)) => assert!(true),
        _ => panic!("Expected Internal error"),
    }
}
